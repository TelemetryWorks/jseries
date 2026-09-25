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

## Install from a checkout

An editable developer installation compiles the Rust extension into the active
virtual environment:

```text
python -m pip install --requirement python/requirements-dev.txt
python -m maturin develop --manifest-path python/Cargo.toml --locked
python -m unittest discover --start-directory python/tests --verbose
```

Run these commands from the repository root after activating the virtual
environment. `maturin develop` installs into that active environment and
requires a Rust toolchain and a supported Python interpreter. To exercise the
same physical-wheel path as CI instead, run:

```text
python -m maturin build --manifest-path python/Cargo.toml --release --locked --out python/dist
python -m pip install --no-index --find-links python/dist --force-reinstall jseries
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
- `pyproject.toml` defines PEP 517/maturin packaging metadata.

Keep computational work in the appropriate Rust library and keep the Python
layer thin. Release the Python interpreter before future long-running decode
operations, avoid per-field crossing of the Python/Rust boundary, and prefer
batch-oriented APIs so Python does not compromise the Rust hot path.

Run the Python boundary benchmark against an installed release wheel:

```text
python python/benchmarks/decode.py --rows 10000 --iterations 7 --include-single
```

The output distinguishes batch return time, batch decoding plus access to every
field, and the optional per-row call loop. Treat results as local measurements
and record CPU, operating system, Python, Rust, and commit information before
comparing runs.
