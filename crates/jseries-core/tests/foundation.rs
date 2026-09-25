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
fn normalizes_70_75_and_80_without_conflating_representations() {
    let logical = normalize_logical70(0x24).unwrap();
    let word75 = normalize_word75((0x1f_u128 << 70) | 0x24).unwrap();
    let simple = normalize_simple80([0x24, 0, 0, 0, 0, 0, 0, 0, 0xc0, 0x07]).unwrap();
    assert_eq!(logical.information, word75.information);
    assert_eq!(word75.information, simple.information);
    assert_eq!(logical.parity, None);
    assert_eq!(word75.parity, Some(0x1f));
    assert_eq!(simple.parity, Some(0x1f));
}

#[test]
fn rejects_high_information_bits_and_nonzero_simple_padding() {
    assert!(normalize_logical70(1_u128 << 70).is_err());
    let mut bytes = [0; 10];
    bytes[9] = 0x80;
    assert!(matches!(
        normalize_simple80(bytes),
        Err(NormalizeError::NonzeroPadding(_))
    ));
}

#[test]
fn assembly_requires_initial_then_noninitial_words() {
    assert!(AssembledMessage::new(vec![normalize_logical70(1).unwrap()]).is_err());
    assert!(
        AssembledMessage::new(vec![
            normalize_logical70(0).unwrap(),
            normalize_logical70(0).unwrap()
        ])
        .is_err()
    );
    assert!(
        AssembledMessage::new(vec![
            normalize_logical70(0).unwrap(),
            normalize_logical70(1).unwrap()
        ])
        .is_ok()
    );
}

#[test]
fn decoder_extracts_prevalidated_fields() {
    let decoder = Decoder::new(package()).unwrap();
    let message = AssembledMessage::new(vec![normalize_logical70(0b100100).unwrap()]).unwrap();
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
        .map(|value| AssembledMessage::new(vec![normalize_logical70(value).unwrap()]).unwrap())
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
    fn logical_and_word75_normalization_agree(value in 0_u128..(1_u128 << 70), parity in 0_u8..32) {
        let logical = normalize_logical70(value).unwrap();
        let framed = normalize_word75((u128::from(parity) << 70) | value).unwrap();
        prop_assert_eq!(logical.information, framed.information);
        prop_assert_eq!(framed.parity, Some(parity));
    }
}

#[test]
fn seeded_random_words_preserve_all_information_bits() {
    let mut random = StdRng::seed_from_u64(0x4a534552494553);
    for _ in 0..1_000 {
        let value = random.random_range(0_u128..(1_u128 << 70));
        assert_eq!(
            normalize_logical70(value).unwrap().information.value(),
            value
        );
    }
}
