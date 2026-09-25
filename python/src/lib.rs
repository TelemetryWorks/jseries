//! Python adapter for the jseries Rust libraries.
#![forbid(unsafe_code)]

use jseries_core::{
    AssembledMessage, DecodeContext, DecodedField, DecodedRecord, Decoder as CoreDecoder,
    FieldStatus, Value, normalize_word,
};
use jseries_schema::load_package;
use pyo3::{
    create_exception,
    exceptions::{PyException, PyTypeError, PyValueError},
    prelude::*,
    types::{PyAny, PyList, PyModule},
};
use std::{path::PathBuf, sync::Arc};

create_exception!(_jseries, JSeriesError, PyException);
create_exception!(_jseries, SchemaError, JSeriesError);
create_exception!(_jseries, DecodeError, JSeriesError);

#[pyclass(name = "DecodedField", frozen, skip_from_py_object)]
#[derive(Clone)]
struct PythonDecodedField {
    inner: DecodedField,
}

#[pymethods]
impl PythonDecodedField {
    #[getter]
    fn id(&self) -> &str {
        &self.inner.id
    }

    #[getter]
    fn definition_ref(&self) -> &str {
        &self.inner.definition_ref
    }

    #[getter]
    fn word(&self) -> u8 {
        self.inner.word
    }

    #[getter]
    fn lsb(&self) -> u8 {
        self.inner.lsb
    }

    #[getter]
    fn width(&self) -> u8 {
        self.inner.width
    }

    #[getter]
    fn raw(&self) -> u128 {
        self.inner.raw
    }

    #[getter]
    fn status(&self) -> &'static str {
        match &self.inner.status {
            FieldStatus::Value(Value::Unsigned(_)) => "unsigned",
            FieldStatus::Value(Value::Signed(_)) => "signed",
            FieldStatus::Value(Value::Rational { .. }) => "rational",
            FieldStatus::Value(Value::Enumeration { .. }) => "enumeration",
            FieldStatus::Special(_) => "special",
            FieldStatus::NotApplicable { .. } => "not-applicable",
            FieldStatus::UnresolvedContext { .. } => "unresolved-context",
            FieldStatus::InvalidEncoding(_) => "invalid-encoding",
        }
    }

    #[getter]
    fn unsigned_value(&self) -> Option<u128> {
        match self.inner.status {
            FieldStatus::Value(Value::Unsigned(value)) => Some(value),
            _ => None,
        }
    }

    #[getter]
    fn signed_value(&self) -> Option<i128> {
        match self.inner.status {
            FieldStatus::Value(Value::Signed(value)) => Some(value),
            _ => None,
        }
    }

    #[getter]
    fn numerator(&self) -> Option<i128> {
        match self.inner.status {
            FieldStatus::Value(Value::Rational { numerator, .. }) => Some(numerator),
            _ => None,
        }
    }

    #[getter]
    fn denominator(&self) -> Option<u64> {
        match self.inner.status {
            FieldStatus::Value(Value::Rational { denominator, .. }) => Some(denominator),
            _ => None,
        }
    }

    #[getter]
    fn unit(&self) -> Option<&str> {
        match &self.inner.status {
            FieldStatus::Value(Value::Rational { unit, .. }) => Some(unit),
            _ => None,
        }
    }

    #[getter]
    fn label(&self) -> Option<&str> {
        match &self.inner.status {
            FieldStatus::Value(Value::Enumeration { label, .. }) | FieldStatus::Special(label) => {
                Some(label)
            }
            _ => None,
        }
    }

    #[getter]
    fn selector(&self) -> Option<&str> {
        match &self.inner.status {
            FieldStatus::NotApplicable { selector }
            | FieldStatus::UnresolvedContext { selector } => Some(selector),
            _ => None,
        }
    }

    #[getter]
    fn error(&self) -> Option<&str> {
        match &self.inner.status {
            FieldStatus::InvalidEncoding(error) => Some(error),
            _ => None,
        }
    }
}

#[pyclass(name = "DecodedRecord", frozen)]
struct PythonDecodedRecord {
    inner: DecodedRecord,
}

#[pymethods]
impl PythonDecodedRecord {
    #[getter]
    fn package_id(&self) -> &str {
        &self.inner.package_id
    }

    #[getter]
    fn package_version(&self) -> &str {
        &self.inner.package_version
    }

    #[getter]
    fn qualification(&self) -> &str {
        &self.inner.qualification
    }

    #[getter]
    fn source_id(&self) -> &str {
        &self.inner.source_id
    }

    #[getter]
    fn source_offset(&self) -> u64 {
        self.inner.source_offset
    }

    #[getter]
    fn message_id(&self) -> &str {
        &self.inner.message_id
    }

    #[getter]
    fn fields(&self) -> Vec<PythonDecodedField> {
        self.inner
            .fields
            .iter()
            .cloned()
            .map(|inner| PythonDecodedField { inner })
            .collect()
    }
}

#[pyclass(name = "Decoder", frozen)]
struct PythonDecoder {
    inner: CoreDecoder,
}

#[pymethods]
impl PythonDecoder {
    #[new]
    fn new(schema_path: PathBuf) -> PyResult<Self> {
        let loaded =
            load_package(schema_path).map_err(|error| SchemaError::new_err(error.to_string()))?;
        let inner = CoreDecoder::new(loaded.schema)
            .map_err(|error| SchemaError::new_err(error.to_string()))?;
        Ok(Self { inner })
    }

    fn message_ids(&self) -> Vec<String> {
        self.inner
            .package()
            .messages
            .iter()
            .map(|message| message.id.to_string())
            .collect()
    }

    #[pyo3(signature = (message_id, values, *, source_id=None, source_offset=0))]
    fn decode(
        &self,
        py: Python<'_>,
        message_id: String,
        values: &Bound<'_, PyAny>,
        source_id: Option<String>,
        source_offset: u64,
    ) -> PyResult<Py<PyAny>> {
        let source_id: Arc<str> = source_id.unwrap_or_else(|| "python".into()).into();
        if let Ok(words) = values.extract::<Vec<u128>>() {
            if words.is_empty() {
                return Err(PyValueError::new_err("decode input cannot be empty"));
            }
            let record = py.detach(|| {
                let message = assemble_words(words)?;
                self.inner
                    .decode(
                        &message_id,
                        &message,
                        &DecodeContext {
                            source_id,
                            source_offset,
                        },
                    )
                    .map(|inner| PythonDecodedRecord { inner })
                    .map_err(|error| DecodeError::new_err(error.to_string()))
            })?;
            return Ok(Py::new(py, record)?.into_any());
        }

        let rows = values.extract::<Vec<Vec<u128>>>().map_err(|_| {
            PyTypeError::new_err(
                "decode values must be a non-empty sequence of words or a non-empty sequence of rows",
            )
        })?;
        if rows.is_empty() {
            return Err(PyValueError::new_err("decode input cannot be empty"));
        }
        let records: Vec<PythonDecodedRecord> = py.detach(|| {
            let messages = rows
                .into_iter()
                .map(assemble_words)
                .collect::<PyResult<Vec<_>>>()?;
            self.inner
                .decode_many(&message_id, &messages, source_id, source_offset)
                .map(|records| {
                    records
                        .into_iter()
                        .map(|inner| PythonDecodedRecord { inner })
                        .collect::<Vec<_>>()
                })
                .map_err(|error| DecodeError::new_err(error.to_string()))
        })?;
        Ok(PyList::new(py, records)?.into_any().unbind())
    }
}

fn assemble_words(words: Vec<u128>) -> PyResult<AssembledMessage> {
    let words = words
        .into_iter()
        .map(|word| normalize_word(word).map_err(|error| PyValueError::new_err(error.to_string())))
        .collect::<PyResult<Vec<_>>>()?;
    AssembledMessage::new(words).map_err(|error| PyValueError::new_err(error.to_string()))
}

#[pymodule]
fn _jseries(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", jseries_core::PACKAGE_VERSION)?;
    module.add_class::<PythonDecoder>()?;
    module.add_class::<PythonDecodedRecord>()?;
    module.add_class::<PythonDecodedField>()?;
    module.add("JSeriesError", py.get_type::<JSeriesError>())?;
    module.add("SchemaError", py.get_type::<SchemaError>())?;
    module.add("DecodeError", py.get_type::<DecodeError>())?;
    Ok(())
}
