//! Framework tests only. None of these fixtures proves MIL-STD-6016 conformance.
use l16_core::*;
use l16_core::schema::{Condition, FieldSpec, Interpretation, MessageSpec, validate_message};
use std::collections::HashSet;

fn decode(b: Baseline, raw: u128) -> DecodedRecord {
    Decoder::new(b, RunMode::SyntheticTest).unwrap().decode("SYNTH-01",
        Word::new(raw, 16).unwrap(), &SelectionEvidence::new("operator: synthetic-test").unwrap(),
        &DecodeContext { source_id: "fixture:normal".into(), source_offset: 1024 }).unwrap()
}
fn field<'a>(record: &'a DecodedRecord, id: &str) -> &'a DecodedField {
    record.fields.iter().find(|f| f.id == id).unwrap()
}
fn fields() -> Vec<FieldSpec> {
    bundled_schema(Baseline::D).messages[0].fields.to_vec()
}
fn message(fields: Vec<FieldSpec>, bits: u8) -> MessageSpec {
    MessageSpec { id: "SYNTH-CUSTOM", word_bits: bits, fields: Box::leak(fields.into_boxed_slice()) }
}
fn custom(fields: Vec<FieldSpec>, bits: u8) -> Decoder {
    let bundle = Bundle { messages: Box::leak(vec![message(fields, bits)].into_boxed_slice()),
        ..*bundled_schema(Baseline::D) };
    Decoder::from_bundle(Box::leak(Box::new(bundle)), Baseline::D, RunMode::SyntheticTest).unwrap()
}
fn custom_record(decoder: &Decoder, value: u128, bits: u8) -> DecodedRecord {
    decoder.decode("SYNTH-CUSTOM", Word::new(value, bits).unwrap(),
        &SelectionEvidence::new("custom synthetic fixture").unwrap(),
        &DecodeContext { source_id: "test".into(), source_offset: 0 }).unwrap()
}
const RAW: FieldSpec = FieldSpec { id: "raw", definition_ref: "SYNTH:RAW", lsb: 0, width: 8,
    interpretation: Interpretation::Unsigned, specials: &[], condition: None };

#[test]
fn req_rev_001_exact_baseline_roundtrip() {
    for b in Baseline::ALL { assert_eq!(b.id().parse::<Baseline>().unwrap(), b); }
}
#[test]
fn req_rev_001_unknown_baseline_is_rejected() {
    for name in ["", "latest", "I", "f", "F1", " F", "H "] { assert!(name.parse::<Baseline>().is_err()); }
}
#[test]
fn req_rev_002_f_change_one_is_distinct() {
    assert_ne!(Baseline::F, Baseline::FChange1);
    assert_ne!(bundled_schema(Baseline::F).schema_id, bundled_schema(Baseline::FChange1).schema_id);
}
#[test]
fn req_auth_001_every_standards_decode_is_blocked() {
    for b in Baseline::ALL {
        assert!(matches!(Decoder::new(b, RunMode::StandardsDecode), Err(DecodeError::NoVerifiedSchema(x)) if x == b));
    }
}
#[test]
fn req_rev_003_mismatched_bundle_is_rejected() {
    assert!(matches!(Decoder::from_bundle(bundled_schema(Baseline::D), Baseline::H,
        RunMode::SyntheticTest), Err(DecodeError::BaselineMismatch)));
}
#[test]
fn req_rev_003_no_cross_revision_fallback() {
    let d = Decoder::new(Baseline::D, RunMode::SyntheticTest).unwrap();
    let raw = Word::new(42, 8).unwrap();
    let e = SelectionEvidence::new("test").unwrap();
    let c = DecodeContext { source_id: "test".into(), source_offset: 0 };
    assert!(matches!(d.decode("SYNTH-H-ONLY", raw, &e, &c),
        Err(DecodeError::UnsupportedMessage { baseline: Baseline::D, word, .. }) if word == raw));
    let h = Decoder::new(Baseline::H, RunMode::SyntheticTest).unwrap();
    assert!(h.decode("SYNTH-H-ONLY", raw, &e, &c).is_ok());
}
#[test]
fn req_rev_004_all_six_literal_golden_cases() {
    // Independently established toy fixture: 0x0765 -> level raw 25.
    let cases = [(Baseline::D,25), (Baseline::E,50), (Baseline::F,75),
        (Baseline::FChange1,100), (Baseline::G,125), (Baseline::H,250)];
    for (b, expected) in cases {
        let r = decode(b, 0x0765);
        assert_eq!(field(&r,"level").raw, 25);
        assert_eq!(field(&r,"level").status, FieldStatus::Value(Value::Rational {
            numerator: expected, denominator: 1, unit: "test_units" }));
        assert_eq!(field(&r,"auxiliary").status, FieldStatus::Value(Value::Unsigned(7)));
    }
}
#[test]
fn req_rev_004_unchanged_toy_definition_is_equivalent() {
    for b in Baseline::ALL {
        assert_eq!(field(&decode(b, 0x0765),"mode").status,
            FieldStatus::Value(Value::Enumeration { raw: 1, label: "active" }));
    }
}
#[test]
fn req_rev_005_independent_instances_keep_their_baseline() {
    let first = decode(Baseline::D, 0x0765);
    let other = decode(Baseline::H, 0x0765);
    let again = decode(Baseline::D, 0x0765);
    assert_eq!(first.fields, again.fields);
    assert_ne!(field(&first,"level").status, field(&other,"level").status);
}
#[test]
fn req_prov_001_selection_evidence_is_required_and_bounded() {
    assert!(SelectionEvidence::new("   ").is_err());
    assert!(SelectionEvidence::new("x".repeat(4097)).is_err());
    assert!(SelectionEvidence::new("explicit run manifest: 123").is_ok());
}
#[test]
fn req_prov_001_source_is_required() {
    let d = Decoder::new(Baseline::D, RunMode::SyntheticTest).unwrap();
    assert!(d.decode("SYNTH-01", Word::new(0,16).unwrap(),
        &SelectionEvidence::new("test").unwrap(),
        &DecodeContext { source_id: "".into(), source_offset: 0 }).is_err());
}
#[test]
fn req_prov_002_preserves_raw_word_offsets_and_schema() {
    let r = decode(Baseline::G, 0x0765);
    assert_eq!(r.original_word.value(), 0x0765);
    assert_eq!(r.original_word.bit_len(), 16);
    assert_eq!(r.source_offset, 1024);
    assert_eq!(r.source_id, "fixture:normal");
    assert_eq!(r.selection_evidence, "operator: synthetic-test");
    assert_eq!(r.schema_digest_sha256, bundled_schema(Baseline::G).digest_sha256);
    assert_eq!(field(&r,"level").lsb, 2);
    assert_eq!(field(&r,"level").width, 6);
    assert!(r.synthetic);
}
#[test]
fn req_prov_003_six_distinct_content_identities() {
    let digests: HashSet<_> = Baseline::ALL.iter().map(|b| bundled_schema(*b).digest_sha256).collect();
    assert_eq!(digests.len(), 6);
    assert!(digests.iter().all(|d| d.len() == 64));
}
#[test]
fn req_bit_001_rejects_invalid_word_lengths_and_high_bits() {
    assert_eq!(Word::new(0,0), Err(WordError::InvalidLength));
    assert_eq!(Word::new(0,129), Err(WordError::InvalidLength));
    assert_eq!(Word::new(65536,16), Err(WordError::HighBitsSet));
}
#[test]
fn req_bit_002_checks_every_one_of_128_bit_positions() {
    for bit in 0..128u8 {
        let w = Word::new(1u128 << bit,128).unwrap();
        for field in 0..128u8 {
            assert_eq!(w.extract(BitRange::new(field,1,128).unwrap()).unwrap(), u128::from(bit == field));
        }
    }
}
#[test]
fn req_bit_002_full_width_and_top_bit_extract_without_shift_overflow() {
    let w = Word::new(u128::MAX,128).unwrap();
    assert_eq!(w.extract(BitRange::new(0,128,128).unwrap()).unwrap(), u128::MAX);
    assert_eq!(w.extract(BitRange::new(127,1,128).unwrap()).unwrap(), 1);
}
#[test]
fn req_bit_003_rejects_empty_or_out_of_bounds_field() {
    assert!(BitRange::new(0,0,16).is_err());
    assert!(BitRange::new(127,2,128).is_err());
    assert!(BitRange::new(255,1,128).is_err());
    let w=Word::new(0,8).unwrap();
    assert!(w.extract(BitRange::new(8,1,16).unwrap()).is_err());
}
#[test]
fn req_bit_004_selected_layout_rejects_word_length_mismatch() {
    let d=Decoder::new(Baseline::D,RunMode::SyntheticTest).unwrap();
    assert!(matches!(d.decode("SYNTH-01",Word::new(0,32).unwrap(),
        &SelectionEvidence::new("test").unwrap(), &DecodeContext { source_id:"t".into(), source_offset:0 }),
        Err(DecodeError::LengthMismatch { expected:16,actual:32 })));
}
#[test]
fn req_sem_001_special_is_not_scaled_into_a_number() {
    for b in Baseline::ALL {
        let r=decode(b,0x00fc);
        assert_eq!(field(&r,"level").raw,63);
        assert_eq!(field(&r,"level").status,FieldStatus::Special("no_statement"));
    }
}
#[test]
fn req_sem_002_undefined_enum_is_invalid_not_a_guessed_value() {
    assert_eq!(field(&decode(Baseline::D,0x2765),"flags").status,
        FieldStatus::InvalidEncoding("undefined enumeration code"));
}
#[test]
fn req_sem_003_false_condition_is_not_applicable_but_raw_is_retained() {
    let r=decode(Baseline::D,0x0764);
    assert_eq!(field(&r,"auxiliary").raw,7);
    assert_eq!(field(&r,"auxiliary").status,FieldStatus::NotApplicable { selector:"mode" });
}
#[test]
fn req_sem_003_special_selector_makes_context_unresolved() {
    let r=decode(Baseline::D,0x0767);
    assert_eq!(field(&r,"mode").status,FieldStatus::Special("not_reported"));
    assert_eq!(field(&r,"auxiliary").status,FieldStatus::UnresolvedContext { selector:"mode" });
}
#[test]
fn req_sem_003_invalid_selector_makes_context_unresolved() {
    let r=decode(Baseline::D,0x0766);
    assert_eq!(field(&r,"auxiliary").status,FieldStatus::UnresolvedContext { selector:"mode" });
}
#[test]
fn req_sem_003_applicable_auxiliary_can_have_its_own_special() {
    assert_eq!(field(&decode(Baseline::D,0x0f65),"auxiliary").status,FieldStatus::Special("unknown"));
}
#[test]
fn req_sem_004_dependency_order_is_not_source_field_order() {
    let mut f=fields(); f.swap(0,2);
    let d=custom(f,16);
    assert_eq!(field(&custom_record(&d,0x0765,16),"auxiliary").status,
        FieldStatus::Value(Value::Unsigned(7)));
}
#[test]
fn req_sem_005_signed_boundaries_are_explicit_twos_complement() {
    let d=custom(vec![FieldSpec { interpretation:Interpretation::TwosComplement,..RAW }],8);
    for (raw,expected) in [(0x7f,127),(0x80,-128),(0xff,-1)] {
        assert_eq!(custom_record(&d,raw,8).fields[0].status,FieldStatus::Value(Value::Signed(expected)));
    }
    let wide=custom(vec![FieldSpec { width:128,interpretation:Interpretation::TwosComplement,..RAW }],128);
    assert_eq!(custom_record(&wide,1u128<<127,128).fields[0].status,
        FieldStatus::Value(Value::Signed(i128::MIN)));
}
#[test]
fn req_sem_006_rational_scaling_does_not_require_floating_point() {
    let d=custom(vec![FieldSpec { interpretation:Interpretation::ScaledUnsigned {
        numerator:1,denominator:3,unit:"toy" },..RAW }],8);
    assert_eq!(custom_record(&d,1,8).fields[0].status,FieldStatus::Value(Value::Rational {
        numerator:1,denominator:3,unit:"toy" }));
}
#[test]
fn req_sch_001_rejects_overlapping_fields() {
    let mut f=fields(); f[1].lsb=1;
    assert!(validate_message(&message(f,16)).is_err());
}
#[test]
fn req_sch_001_rejects_out_of_bounds_field() {
    let mut f=fields(); f[3].width=5;
    assert!(validate_message(&message(f,16)).is_err());
}
#[test]
fn req_sch_002_rejects_duplicate_field_identity() {
    let mut f=fields(); f[1].id="mode";
    assert!(validate_message(&message(f,16)).is_err());
}
#[test]
fn req_sch_003_rejects_missing_selector() {
    let mut f=fields(); f[2].condition=Some(Condition { selector:"missing",equals:1 });
    assert!(validate_message(&message(f,16)).is_err());
}
#[test]
fn req_sch_003_rejects_dependency_cycles() {
    let mut f=fields(); f[0].condition=Some(Condition { selector:"auxiliary",equals:1 });
    assert!(validate_message(&message(f,16)).is_err());
}
#[test]
fn req_sch_004_rejects_zero_denominator() {
    let f=FieldSpec { interpretation:Interpretation::ScaledUnsigned { numerator:1,denominator:0,unit:"toy" },..RAW };
    assert!(validate_message(&message(vec![f],8)).is_err());
}
#[test]
fn req_sch_004_rejects_unsupported_scaled_width() {
    let f=FieldSpec { width:64,interpretation:Interpretation::ScaledUnsigned { numerator:1,denominator:1,unit:"toy" },..RAW };
    assert!(validate_message(&message(vec![f],64)).is_err());
}
#[test]
fn req_sch_005_rejects_special_enum_conflict() {
    let mut f=fields(); f[0].specials=&[(1,"not_reported")];
    assert!(validate_message(&message(f,16)).is_err());
}
#[test]
fn req_sch_005_rejects_duplicate_special() {
    let f=FieldSpec { specials:&[(255,"a"),(255,"b")],..RAW };
    assert!(validate_message(&message(vec![f],8)).is_err());
}
#[test]
fn req_sch_005_rejects_out_of_width_code() {
    let f=FieldSpec { specials:&[(256,"outside")],..RAW };
    assert!(validate_message(&message(vec![f],8)).is_err());
}
#[test]
fn req_auth_002_real_message_names_are_not_implemented() {
    let d=Decoder::new(Baseline::H,RunMode::SyntheticTest).unwrap();
    assert_eq!(d.message_word_bits("J3.2"),None);
}
