# Contributing

Development requires Python 3.10 or newer, Rust 1.86 or newer, Taplo 0.10.0,
and uv 0.12.19. Bootstrap and provision the locked Python environment before
running the complete gate:

```text
python -m venv .venv
```

Activate `.venv`, then run:

```text
python -m pip install uv==0.12.19
uv sync --directory python --locked --only-group dev
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m maturin develop --uv --locked
```

Run the complete repository gate before submitting a change:

```text
python tools/check.py
```

## Python binding

uv creates `python/.venv` from the committed lockfile. Compile the extension
into that environment and run focused Python checks with:

```text
uv sync --directory python --locked --only-group dev
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m maturin develop --uv --locked
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m ruff format --check . ../tools
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m ruff check . ../tools
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m pylint jseries tests benchmarks ../tools
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m mypy
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m pytest --cov=jseries --cov-branch
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m mypy.stubtest jseries
```

The binding is an adapter. Put decoding behavior and performance-sensitive work
in the Rust crates, expose batch-oriented operations to Python, and avoid
crossing the language boundary once per decoded field. See
[`python/README.md`](python/README.md) for package layout, versioning, wheel
installation, and platform support.

Do not edit a Python version constant. Change `workspace.package.version` in
the root `Cargo.toml`; Cargo, the CLI, maturin wheel metadata, and
`jseries.__version__` all derive from it.

Pull requests run Rust CI, CodeQL, SonarCloud, Ruff, Pylint, mypy, native-stub
verification, Python branch coverage, Python 3.10/3.14 tests on Windows and
Linux, and install tests for both generated wheel artifacts.
