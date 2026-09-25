use jseries_core::{
    AssembledMessage, DecodeContext, Decoder, FieldStatus, Value, normalize_logical70,
};
use jseries_schema::load_package;

#[test]
fn loads_and_decodes_repository_example() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas");
    let loaded = load_package(&root).unwrap();
    assert_eq!(loaded.schema.metadata.id.as_ref(), "jseries-schema-starter");
    assert_eq!(loaded.vectors.len(), 2);
    let decoder = Decoder::new(loaded.schema).unwrap();
    let message = AssembledMessage::new(vec![normalize_logical70(0x1c94).unwrap()]).unwrap();
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
