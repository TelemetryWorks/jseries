# jseries for Python

The `jseries` Python package is a native extension backed by the repository's
Rust libraries. It exposes package version information, persistent schema-backed
decoders, typed decoded records, and single-message or batch 70-bit decoding.

## Requirements

- CPython 3.10 or newer on 64-bit Windows or 64-bit Linux;
- `pip` capable of installing platform wheels; and
- Rust 1.86 or newer only when building from source.

## Install a wheel

Download the wheel artifact for your operating system from the relevant GitHub
Actions run, then install the exact file in a virtual environment:

```text
python -m venv .venv
```

Activate it with `.venv\\Scripts\\activate` on Windows or
`source .venv/bin/activate` on Linux, then run:

```text
python -m pip install path/to/the-downloaded-jseries-wheel.whl
python -c "import jseries; print(jseries.__version__)"
```

The Linux artifact is a manylinux x86-64 wheel. The Windows artifact is an
x86-64 wheel. Both use CPython's stable ABI starting at Python 3.10, so the
same platform wheel can be installed by supported newer CPython versions.

The package has not been published to PyPI. A plain `pip install jseries`
should not be treated as installing this project until a release workflow and
trusted PyPI publishing are established.

## Decode 70-bit messages

Construct `Decoder` once. It loads and validates the TOML package and compiles
the immutable Rust decode plans; reuse it for every message using that package:

```python
from pathlib import Path
import jseries

decoder = jseries.Decoder(Path("schemas"))
print(decoder.message_ids())

record = decoder.decode(
    "EXAMPLE-70",
    [0x1C94],
    source_id="capture-a",
    source_offset=42,
)
print(record.message_id, record.source_offset)
for field in record.fields:
    print(field.id, field.raw, field.status, field.label)
```

The sequence passed as the second argument contains all words for one message.
Values must contain only the 70 information bits; any higher bits are rejected.
Transport framing and packed byte representations are not accepted.

Errors from package loading and decoding derive from `jseries.JSeriesError`.
Out-of-range Python values and invalid word assembly raise `ValueError`.

## Decode many rows

Do not call `decode` once per row when a homogeneous collection is already
available. Pass a sequence of rows and cross the Python/Rust boundary once:

```python
rows = [[0x1C94], [0x1494], [0x0C94]]
records = decoder.decode(
    "EXAMPLE-70",
    rows,
    source_id="capture-a",
    source_offset=1_000,
)
```

`decode` distinguishes one record from a batch by input shape: a sequence of
integers is one message, while a sequence of integer sequences is a batch. Empty
input is rejected because its shape is ambiguous. For a batch, `source_offset`
is the first row's offset.

The batch path preserves input order and assigns consecutive offsets. It releases
the Python interpreter while Rust normalizes and decodes the batch. The current
API returns rich row-oriented objects; for very wide or multi-million-row data,
a future compact columnar output will avoid constructing one Python-facing field
object per accessed row and field.

## Contributor environment

Maturin remains the package build backend and Cargo remains the version source.
uv manages only the reproducible contributor environment. From the repository
root, create a bootstrap environment, activate it, and install the pinned uv:

```text
python -m venv .venv
python -m pip install uv==0.12.19
```

uv creates `python/.venv` from the cross-platform lockfile. Install the locked
build, lint, type-check, and test groups and compile the native extension:

```text
uv sync --directory python --locked --only-group dev
uv run --directory python --locked --no-sync maturin develop --uv --locked
```

Run the complete Python gate with:

```text
uv run --directory python --locked --no-sync ruff format --check . ../tools
uv run --directory python --locked --no-sync ruff check . ../tools
uv run --directory python --locked --no-sync pylint jseries tests benchmarks ../tools
uv run --directory python --locked --no-sync mypy
uv run --directory python --locked --no-sync pytest --cov=jseries --cov-branch --cov-report=term-missing
uv run --directory python --locked --no-sync python -m mypy.stubtest jseries
```

Ruff is the formatter and fast linter. Pylint is an additional independent
semantic and style gate. Mypy checks the public package, tests, benchmarks, and
repository utilities in strict mode; `mypy.stubtest` compares the checked-in
stubs with the compiled PyO3 module. Pytest-cov measures Python branch coverage;
Rust coverage remains the responsibility of `cargo-llvm-cov`.

To exercise the same physical-wheel path as CI instead, run:

```text
uv run --directory python --locked --no-sync maturin build --release --locked --out dist
uv run --project python --locked --no-sync python tools/install_python_wheel.py python/dist
uv run --project python --locked --no-sync pytest python/tests
```

## Version source

The only maintained project version is `workspace.package.version` in the
repository's root `Cargo.toml`. The `jseries-core` Rust library exposes that
Cargo package version, the native Python module assigns it to `__version__`,
and maturin uses the same Cargo version for wheel metadata. The Python source
does not contain a second version string.

## Contributor layout

- `Cargo.toml` defines the PyO3 extension crate.
- `src/lib.rs` is the narrow Rust-to-Python adapter.
- `jseries/` contains the Python package facade and type information.
- `tests/` verifies the installed package rather than importing from a source
  tree fallback.
- `benchmarks/` measures the installed wheel's end-to-end Python/native path.
- `pyproject.toml` defines PEP 517/maturin metadata, dependency groups, and
  Python tool configuration.
- `uv.lock` pins the cross-platform contributor tool environment.

Keep computational work in the appropriate Rust library and keep the Python
layer thin. Release the Python interpreter before future long-running decode
operations, avoid per-field crossing of the Python/Rust boundary, and prefer
batch-oriented APIs so Python does not compromise the Rust hot path.

Run the Python boundary benchmark against an installed release wheel:

```text
uv run --project python --locked --no-sync python python/benchmarks/decode.py --rows 10000 --iterations 7 --include-single
```

The output distinguishes batch return time, batch decoding plus access to every
field, and the optional per-row call loop. Treat results as local measurements
and record CPU, operating system, Python, Rust, and commit information before
comparing runs.
