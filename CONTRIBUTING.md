# Contributing

Development requires Python 3.10 or newer, Rust 1.86 or newer, and Taplo 0.10.0.
Run the complete repository gate before submitting a change:

```text
python tools/check.py
```

## Python binding

Create and activate a virtual environment, then install the pinned binding
tooling and compile the extension into that environment:

```text
python -m pip install --requirement python/requirements-dev.txt
python -m maturin develop --manifest-path python/Cargo.toml --locked
python -m unittest discover --start-directory python/tests --verbose
```

The binding is an adapter. Put decoding behavior and performance-sensitive work
in the Rust crates, expose batch-oriented operations to Python, and avoid
crossing the language boundary once per decoded field. See
[`python/README.md`](python/README.md) for package layout, versioning, wheel
installation, and platform support.

Do not edit a Python version constant. Change `workspace.package.version` in
the root `Cargo.toml`; Cargo, the CLI, maturin wheel metadata, and
`jseries.__version__` all derive from it.

Pull requests run Rust CI, CodeQL, SonarCloud, Python 3.10/3.14 tests on Windows
and Linux, and install tests for both generated wheel artifacts.
