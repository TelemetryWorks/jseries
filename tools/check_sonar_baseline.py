#!/usr/bin/env python3
"""Read and verify the fixed SonarCloud baseline without changing any settings."""

import base64
import json
import os
import sys
import urllib.error
import urllib.parse
import urllib.request
from http import HTTPStatus
from http.client import HTTPException

PROJECT = "TelemetryWorks_jseries"
EXPECTED = {"sonar.leak.period": "2026-09-23", "sonar.leak.period.type": "date"}
MAX_RESPONSE_BYTES = 65536


class NoRedirect(urllib.request.HTTPRedirectHandler):
    """Refuse redirects so the credential remains on the fixed request target."""

    def redirect_request(self, *_args: object, **_kwargs: object) -> None:
        """Do not follow redirects, including redirects on the same host."""
        return None


def baseline_matches(payload: object) -> bool:
    """Require exactly the two explicit settings, accepting omitted false flags."""
    if not isinstance(payload, dict):
        return False
    settings = payload.get("settings")
    if not isinstance(settings, list) or len(settings) != len(EXPECTED):
        return False
    observed: dict[str, object] = {}
    for setting in settings:
        if not isinstance(setting, dict):
            return False
        key = setting.get("key")
        if not isinstance(key, str) or key not in EXPECTED or key in observed:
            return False
        if setting.get("inherited", False) is not False:
            return False
        observed[key] = setting.get("value")
    return observed == EXPECTED


def read_baseline(token: str) -> object:
    """Perform one bounded authenticated GET; never log response bodies or headers."""
    query = urllib.parse.urlencode({"component": PROJECT, "keys": ",".join(EXPECTED)})
    authorization = "Basic " + base64.b64encode((token + ":").encode()).decode()
    request = urllib.request.Request(
        "https://sonarcloud.io/api/settings/values?" + query,
        method="GET",
        headers={"Authorization": authorization, "Accept": "application/json"},
    )
    opener = urllib.request.build_opener(NoRedirect, urllib.request.ProxyHandler({}))
    with opener.open(request, timeout=10) as response:
        if response.status != HTTPStatus.OK:
            raise ValueError("unexpected response status")
        body = response.read(MAX_RESPONSE_BYTES + 1)
    if len(body) > MAX_RESPONSE_BYTES:
        raise ValueError("response too large")
    payload: object = json.loads(body)
    return payload


def main() -> int:
    """Fail closed before analysis if the selected project baseline cannot be verified."""
    token = os.environ.get("SONAR_TOKEN", "")
    if not token or os.environ.get("SONAR_PROJECT_KEY") != PROJECT:
        print("Sonar baseline credentials or project target are not configured.", file=sys.stderr)
        return 1
    try:
        payload = read_baseline(token)
    except (OSError, ValueError, HTTPException, RecursionError):
        print("Sonar baseline could not be read safely; refusing analysis.", file=sys.stderr)
        return 1
    if not baseline_matches(payload):
        print("Sonar baseline is unset, inherited, or changed; refusing analysis.", file=sys.stderr)
        return 1
    print("Sonar baseline verified: explicit project date 2026-09-23.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
