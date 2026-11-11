"""Exercise the read-only Sonar preflight without credentials or network access."""

import importlib.util
import json
from email.message import Message
from pathlib import Path
from types import ModuleType
from unittest.mock import MagicMock, patch
from urllib.error import HTTPError
from urllib.parse import parse_qs, urlsplit

import pytest

EXPECTED = {"sonar.leak.period": "2026-09-23", "sonar.leak.period.type": "date"}
PROJECT = "TelemetryWorks_jseries"


@pytest.fixture(name="guard", scope="module")
def guard_fixture() -> ModuleType:
    """Import the repository utility without running its network entry point."""
    path = Path(__file__).resolve().parents[2] / "tools/check_sonar_baseline.py"
    spec = importlib.util.spec_from_file_location("jseries_sonar_guard_test", path)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def baseline() -> dict[str, object]:
    """Return the explicit project settings in the API's response shape."""
    return {"settings": [{"key": key, "value": value} for key, value in EXPECTED.items()]}


def test_explicit_baseline_accepts_omitted_or_false_inheritance(guard: ModuleType) -> None:
    """Accept Cloud's omitted-false serialization without accepting inheritance."""
    assert guard.baseline_matches(baseline())
    assert guard.baseline_matches(
        {
            "settings": [
                {"key": key, "value": value, "inherited": False} for key, value in EXPECTED.items()
            ]
        }
    )


@pytest.mark.parametrize(
    "payload",
    [
        None,
        {},
        {"settings": []},
        {"settings": None},
        {"settings": [None, None]},
        {"settings": [{"key": [], "value": "date"}, {"key": "unknown"}]},
        {"settings": [{"key": "sonar.leak.period", "value": "2026-09-23"}] * 2},
    ],
)
def test_missing_malformed_or_duplicate_baseline_fails(guard: ModuleType, payload: object) -> None:
    """Unverifiable or ambiguous responses never allow analysis."""
    assert not guard.baseline_matches(payload)


@pytest.mark.parametrize("inheritance", [True, None, 0, "false"])
def test_explicit_nonfalse_inheritance_fails(guard: ModuleType, inheritance: object) -> None:
    """Only absent or boolean false means explicit project ownership."""
    rows = [
        {"key": key, "value": value, "inherited": inheritance} for key, value in EXPECTED.items()
    ]
    assert not guard.baseline_matches({"settings": rows})


@pytest.mark.parametrize(
    "key,value", [("sonar.leak.period", "2026-11-10"), ("sonar.leak.period.type", "days")]
)
def test_baseline_drift_fails(guard: ModuleType, key: str, value: str) -> None:
    """Moving the cutoff or changing its type requires an intentional review."""
    rows = [
        {"key": item, "value": value if item == key else expected}
        for item, expected in EXPECTED.items()
    ]
    assert not guard.baseline_matches({"settings": rows})


def test_fetch_is_one_bounded_get_to_fixed_host(guard: ModuleType) -> None:
    """The production preflight cannot mutate settings or follow redirects."""
    response = MagicMock(status=200)
    response.read.return_value = json.dumps(baseline()).encode()
    with patch("urllib.request.build_opener") as build:
        build.return_value.open.return_value.__enter__.return_value = response
        assert guard.read_baseline("fake-token") == baseline()
        build.return_value.open.assert_called_once()
        request = build.return_value.open.call_args.args[0]
        assert request.get_method() == "GET"
        assert request.data is None
        url = urlsplit(request.full_url)
        assert (url.scheme, url.netloc, url.path) == (
            "https",
            "sonarcloud.io",
            "/api/settings/values",
        )
        assert parse_qs(url.query) == {"component": [PROJECT], "keys": [",".join(EXPECTED)]}
        assert build.return_value.open.call_args.kwargs == {"timeout": 10}
        response.read.assert_called_once_with(65537)
    assert (
        guard.NoRedirect().redirect_request(None, None, 302, "", {}, "https://example.com") is None
    )


@pytest.mark.parametrize("body", [b"not json", b"x" * 65537], ids=["malformed-json", "oversized"])
def test_malformed_or_oversized_fetch_fails(guard: ModuleType, body: bytes) -> None:
    """Bound and validate the payload before inspecting any settings."""
    response = MagicMock(status=200)
    response.read.return_value = body
    with patch("urllib.request.build_opener") as build:
        build.return_value.open.return_value.__enter__.return_value = response
        with pytest.raises(ValueError):
            guard.read_baseline("fake-token")


def test_entry_point_needs_no_admin_reads_and_sanitizes_failure(
    guard: ModuleType, monkeypatch: pytest.MonkeyPatch, capsys: pytest.CaptureFixture[str]
) -> None:
    """Only readable settings are needed; HTTP errors never reveal credential/body text."""
    monkeypatch.setenv("SONAR_TOKEN", "fake-token-must-not-leak")
    monkeypatch.setenv("SONAR_PROJECT_KEY", PROJECT)
    with patch.object(guard, "read_baseline", return_value=baseline()):
        assert guard.main() == 0
    with patch.object(
        guard,
        "read_baseline",
        side_effect=HTTPError("", 403, "fake-token-must-not-leak", Message(), None),
    ):
        assert guard.main() == 1
    with patch.object(guard, "read_baseline", return_value={"settings": []}):
        assert guard.main() == 1
    monkeypatch.delenv("SONAR_TOKEN")
    with patch.object(guard, "read_baseline") as fetch:
        assert guard.main() == 1
        fetch.assert_not_called()
    captured = capsys.readouterr()
    assert "fake-token-must-not-leak" not in captured.out + captured.err
