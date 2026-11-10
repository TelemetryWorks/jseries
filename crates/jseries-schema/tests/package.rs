use jseries_core::{AssembledMessage, DecodeContext, Decoder, FieldStatus, Value, normalize_word};
use jseries_schema::load_package;

#[test]
fn loads_and_decodes_repository_example() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas");
    let loaded = load_package(&root).unwrap();
    assert_eq!(loaded.schema.metadata.id.as_ref(), "jseries-schema-starter");
    assert_eq!(loaded.vectors.len(), 1);
    let decoder = Decoder::new(loaded.schema).unwrap();
    let message = AssembledMessage::new(vec![normalize_word(0x1c94).unwrap()]).unwrap();
    let decoded = decoder
        .decode(
            "EXAMPLE-70",
            &message,
            &DecodeContext {
                source_id: "vector".into(),
                source_offset: 0,
            },
        )
        .unwrap();
    assert_eq!(
        decoded.fields[1].status,
        FieldStatus::Value(Value::Enumeration {
            raw: 1,
            label: "active".into()
        })
    );
}

#[test]
fn vectors_reject_removed_input_format_key() {
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas");
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("jseries-vector-{}-{nonce}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    for relative in [
        "package.toml",
        "messages/example.toml",
        "catalogs/mode.toml",
        "catalogs/word-format.toml",
        "vectors/examples.toml",
    ] {
        let target = root.join(relative);
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        std::fs::copy(source.join(relative), target).unwrap();
    }
    let vector = root.join("vectors/examples.toml");
    let contents = std::fs::read_to_string(&vector).unwrap();
    std::fs::write(
        &vector,
        format!("{contents}\ninput_format = \"logical70\"\n"),
    )
    .unwrap();
    let error = load_package(&root).unwrap_err();
    std::fs::remove_dir_all(root).unwrap();
    assert!(
        matches!(error, jseries_schema::LoadError::Toml(ref message) if message.contains("unknown field `input_format`")),
        "{error}"
    );
}
