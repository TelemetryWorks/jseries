//! Strict TOML schema-package loader. This is deliberately outside the decode hot path.
#![forbid(unsafe_code)]

use jseries_core::{
    CodeLabel, Condition, FieldSpec, Interpretation, MessageSpec, PackageMetadata, SchemaPackage,
};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fmt, fs,
    path::{Component, Path, PathBuf},
    sync::Arc,
};

const MAX_FILE_BYTES: u64 = 1024 * 1024;
const MAX_PACKAGE_BYTES: u64 = 16 * MAX_FILE_BYTES;

#[derive(Debug)]
pub enum LoadError {
    Io(String),
    Toml(String),
    Invalid(String),
}
impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(s) => write!(f, "I/O: {s}"),
            Self::Toml(s) => write!(f, "TOML: {s}"),
            Self::Invalid(s) => f.write_str(s),
        }
    }
}
impl std::error::Error for LoadError {}

#[derive(Debug)]
pub struct LoadedPackage {
    pub schema: Arc<SchemaPackage>,
    pub vectors: Vec<VectorCase>,
}
#[derive(Debug, Clone)]
pub struct VectorCase {
    pub name: String,
    pub message: String,
    pub input_format: String,
    pub words: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    format_version: u16,
    id: String,
    version: String,
    baseline: String,
    profile: String,
    qualification: String,
    source: String,
    messages: Vec<String>,
    #[serde(default)]
    catalogs: Vec<String>,
    #[serde(default)]
    vectors: Vec<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CatalogDoc {
    id: String,
    #[serde(default)]
    unit: Option<String>,
    entries: Vec<EntryDoc>,
}
#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct EntryDoc {
    raw: String,
    label: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MessageDoc {
    id: String,
    word_count: u8,
    fields: Vec<FieldDoc>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FieldDoc {
    id: String,
    definition_ref: String,
    word: u8,
    lsb: u8,
    width: u8,
    interpretation: InterpretationDoc,
    #[serde(default)]
    specials: Vec<EntryDoc>,
    #[serde(default)]
    condition: Option<ConditionDoc>,
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
enum InterpretationDoc {
    Unsigned,
    TwosComplement,
    ScaledUnsigned {
        numerator: i64,
        denominator: u64,
        unit: String,
    },
    Catalog {
        catalog: String,
    },
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConditionDoc {
    selector: String,
    equals: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VectorsDoc {
    cases: Vec<VectorDoc>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VectorDoc {
    name: String,
    message: String,
    input_format: String,
    words: Vec<String>,
}

pub fn load_package(root: impl AsRef<Path>) -> Result<LoadedPackage, LoadError> {
    let root = root.as_ref();
    let mut total = 0;
    let manifest: Manifest = read_toml(root, Path::new("package.toml"), &mut total)?;
    check_unique(&manifest.messages, "message file")?;
    check_unique(&manifest.catalogs, "catalog file")?;
    check_unique(&manifest.vectors, "vector file")?;
    let mut catalogs = HashMap::with_capacity(manifest.catalogs.len());
    for path in &manifest.catalogs {
        let doc: CatalogDoc = read_toml(root, Path::new(path), &mut total)?;
        let entries = convert_entries(doc.entries)?;
        if catalogs
            .insert(doc.id.clone(), (doc.unit, entries))
            .is_some()
        {
            return Err(LoadError::Invalid(format!("duplicate catalog {}", doc.id)));
        }
    }
    let definition_refs: HashSet<_> = catalogs.keys().cloned().collect();
    let mut messages = Vec::with_capacity(manifest.messages.len());
    for path in &manifest.messages {
        let doc: MessageDoc = read_toml(root, Path::new(path), &mut total)?;
        let mut fields = Vec::with_capacity(doc.fields.len());
        for field in doc.fields {
            if !definition_refs.contains(&field.definition_ref) {
                return Err(LoadError::Invalid(format!(
                    "unknown definition reference {}",
                    field.definition_ref
                )));
            }
            let interpretation = match field.interpretation {
                InterpretationDoc::Unsigned => Interpretation::Unsigned,
                InterpretationDoc::TwosComplement => Interpretation::TwosComplement,
                InterpretationDoc::ScaledUnsigned {
                    numerator,
                    denominator,
                    unit,
                } => Interpretation::ScaledUnsigned {
                    numerator,
                    denominator,
                    unit: unit.into(),
                },
                InterpretationDoc::Catalog { catalog } => {
                    let (_, entries) = catalogs
                        .get(&catalog)
                        .ok_or_else(|| LoadError::Invalid(format!("unknown catalog {catalog}")))?;
                    Interpretation::Enumeration(Arc::clone(entries))
                }
            };
            fields.push(FieldSpec {
                id: field.id.into(),
                definition_ref: field.definition_ref.into(),
                word: field.word,
                lsb: field.lsb,
                width: field.width,
                interpretation,
                specials: convert_entries(field.specials)?,
                condition: field
                    .condition
                    .map(|condition| {
                        parse_raw(&condition.equals).map(|equals| Condition {
                            selector: condition.selector.into(),
                            equals,
                        })
                    })
                    .transpose()?,
            });
        }
        messages.push(MessageSpec {
            id: doc.id.into(),
            word_count: doc.word_count,
            fields: fields.into(),
        });
    }
    let metadata = PackageMetadata {
        format_version: manifest.format_version,
        id: manifest.id.into(),
        version: manifest.version.into(),
        baseline: manifest.baseline.into(),
        profile: manifest.profile.into(),
        qualification: manifest.qualification.into(),
        source: manifest.source.into(),
    };
    let schema = Arc::new(
        SchemaPackage::new(metadata, messages).map_err(|e| LoadError::Invalid(e.to_string()))?,
    );
    let mut vectors = Vec::new();
    for path in &manifest.vectors {
        let doc: VectorsDoc = read_toml(root, Path::new(path), &mut total)?;
        vectors.extend(doc.cases.into_iter().map(|case| VectorCase {
            name: case.name,
            message: case.message,
            input_format: case.input_format,
            words: case.words,
        }));
    }
    Ok(LoadedPackage { schema, vectors })
}

fn read_toml<T: for<'de> Deserialize<'de>>(
    root: &Path,
    relative: &Path,
    total: &mut u64,
) -> Result<T, LoadError> {
    validate_relative(relative)?;
    let path = root.join(relative);
    let size = fs::metadata(&path)
        .map_err(|e| LoadError::Io(format!("{}: {e}", path.display())))?
        .len();
    if size > MAX_FILE_BYTES || total.saturating_add(size) > MAX_PACKAGE_BYTES {
        return Err(LoadError::Invalid(
            "schema package exceeds configured size limits".into(),
        ));
    }
    *total += size;
    let text =
        fs::read_to_string(&path).map_err(|e| LoadError::Io(format!("{}: {e}", path.display())))?;
    toml::from_str(&text).map_err(|e| LoadError::Toml(format!("{}: {e}", path.display())))
}
fn validate_relative(path: &Path) -> Result<(), LoadError> {
    if path.extension().and_then(|s| s.to_str()) != Some("toml")
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(LoadError::Invalid(format!(
            "unsafe or non-TOML package path {}",
            path.display()
        )));
    }
    Ok(())
}
fn check_unique(values: &[String], name: &str) -> Result<(), LoadError> {
    let mut seen = HashSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(LoadError::Invalid(format!("duplicate {name} {value}")));
        }
    }
    Ok(())
}
fn convert_entries(entries: Vec<EntryDoc>) -> Result<Arc<[CodeLabel]>, LoadError> {
    let mut entries: Vec<_> = entries
        .into_iter()
        .map(|entry| {
            Ok(CodeLabel {
                raw: parse_raw(&entry.raw)?,
                label: entry.label.into(),
            })
        })
        .collect::<Result<_, LoadError>>()?;
    entries.sort_unstable_by_key(|entry| entry.raw);
    Ok(entries.into())
}
pub fn parse_raw(text: &str) -> Result<u128, LoadError> {
    if text.is_empty()
        || text.starts_with('+')
        || (text.len() > 1 && text.starts_with('0') && !text.starts_with("0x"))
    {
        return Err(LoadError::Invalid(format!(
            "non-canonical raw integer {text:?}"
        )));
    }
    let parsed = if let Some(hex) = text.strip_prefix("0x") {
        if hex.is_empty()
            || hex
                .bytes()
                .any(|b| !b.is_ascii_hexdigit() || b.is_ascii_uppercase())
        {
            None
        } else {
            u128::from_str_radix(hex, 16).ok()
        }
    } else if text.bytes().all(|b| b.is_ascii_digit()) {
        text.parse().ok()
    } else {
        None
    };
    parsed.ok_or_else(|| LoadError::Invalid(format!("invalid raw integer {text:?}")))
}
pub fn package_manifest_path(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join("package.toml")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn raw_numbers_are_explicit_and_canonical() {
        assert_eq!(parse_raw("0x3f").unwrap(), 63);
        assert_eq!(parse_raw("63").unwrap(), 63);
        for bad in ["", "+1", "01", "0Xff", "0xFF", "-1"] {
            assert!(parse_raw(bad).is_err(), "{bad}");
        }
    }
}
