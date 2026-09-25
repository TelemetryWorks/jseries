use jseries_core::*;
use proptest::prelude::*;
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::sync::Arc;

fn package() -> Arc<SchemaPackage> {
    Arc::new(
        SchemaPackage::new(
            PackageMetadata {
                format_version: 1,
                id: "test".into(),
                version: "1".into(),
                baseline: "synthetic".into(),
                profile: "test".into(),
                qualification: "test-only".into(),
                source: "project test".into(),
            },
            vec![MessageSpec {
                id: "M".into(),
                word_count: 1,
                fields: vec![
                    FieldSpec {
                        id: "format".into(),
                        definition_ref: "T:F".into(),
                        word: 0,
                        lsb: 0,
                        width: 2,
                        interpretation: Interpretation::Unsigned,
                        specials: Arc::from([]),
                        condition: None,
                    },
                    FieldSpec {
                        id: "value".into(),
                        definition_ref: "T:V".into(),
                        word: 0,
                        lsb: 2,
                        width: 4,
                        interpretation: Interpretation::Unsigned,
                        specials: Arc::from([]),
                        condition: None,
                    },
                ]
                .into(),
            }],
        )
        .unwrap(),
    )
}

#[test]
fn normalizes_70_bit_information_words() {
    assert_eq!(normalize_word(0x24).unwrap().value(), 0x24);
}

#[test]
fn rejects_bits_above_70_bit_information_word() {
    assert!(normalize_word(1_u128 << 70).is_err());
}

#[test]
fn assembly_requires_initial_then_noninitial_words() {
    assert!(AssembledMessage::new(vec![normalize_word(1).unwrap()]).is_err());
    assert!(
        AssembledMessage::new(vec![normalize_word(0).unwrap(), normalize_word(0).unwrap()])
            .is_err()
    );
    assert!(
        AssembledMessage::new(vec![normalize_word(0).unwrap(), normalize_word(1).unwrap()]).is_ok()
    );
}

#[test]
fn decoder_extracts_prevalidated_fields() {
    let decoder = Decoder::new(package()).unwrap();
    let message = AssembledMessage::new(vec![normalize_word(0b100100).unwrap()]).unwrap();
    let record = decoder
        .decode(
            "M",
            &message,
            &DecodeContext {
                source_id: "test".into(),
                source_offset: 4,
            },
        )
        .unwrap();
    assert_eq!(record.fields[1].raw, 9);
    assert_eq!(record.source_offset, 4);
}

#[test]
fn batch_decode_preserves_order_and_assigns_offsets() {
    let decoder = Decoder::new(package()).unwrap();
    let messages = [0b000100_u128, 0b001000, 0b001100]
        .into_iter()
        .map(|value| AssembledMessage::new(vec![normalize_word(value).unwrap()]).unwrap())
        .collect::<Vec<_>>();
    let records = decoder
        .decode_many("M", &messages, "batch".into(), 40)
        .unwrap();

    assert_eq!(records.len(), 3);
    assert_eq!(records[0].source_offset, 40);
    assert_eq!(records[2].source_offset, 42);
    assert_eq!(records[0].fields[1].raw, 1);
    assert_eq!(records[1].fields[1].raw, 2);
    assert_eq!(records[2].fields[1].raw, 3);
}

#[test]
fn schema_rejects_overlap_and_missing_selectors() {
    let mut fields = package().messages[0].fields.to_vec();
    fields[1].lsb = 1;
    let message = MessageSpec {
        id: "bad".into(),
        word_count: 1,
        fields: fields.into(),
    };
    assert!(jseries_core::schema::validate_message(&message).is_err());
}

proptest! {
    #[test]
    fn all_70_bit_values_round_trip(value in 0_u128..(1_u128 << 70)) {
        prop_assert_eq!(normalize_word(value).unwrap().value(), value);
    }
}

#[test]
fn seeded_random_words_preserve_all_information_bits() {
    let mut random = StdRng::seed_from_u64(0x4a534552494553);
    for _ in 0..1_000 {
        let value = random.random_range(0_u128..(1_u128 << 70));
        assert_eq!(normalize_word(value).unwrap().value(), value);
    }
}
