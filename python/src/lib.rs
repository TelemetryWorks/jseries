//! Python adapter for the jseries Rust libraries.
#![forbid(unsafe_code)]

use pyo3::{prelude::*, types::PyModule};

#[pymodule]
fn _jseries(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", jseries_core::PACKAGE_VERSION)?;
    Ok(())
}
