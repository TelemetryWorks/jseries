use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use jseries_core::{
    AssembledMessage, DecodeContext, Decoder, FieldSpec, Interpretation, MessageSpec,
    PackageMetadata, SchemaPackage, normalize_word,
};
use std::{hint::black_box, sync::Arc};

fn package() -> Arc<SchemaPackage> {
    Arc::new(
        SchemaPackage::new(
            PackageMetadata {
                format_version: 1,
                id: "benchmark".into(),
                version: "1".into(),
                baseline: "example".into(),
                profile: "benchmark".into(),
                qualification: "benchmark-only".into(),
                source: "project benchmark".into(),
            },
            vec![MessageSpec {
                id: "BENCH".into(),
                word_count: 1,
                fields: vec![
                    FieldSpec {
                        id: "format".into(),
                        definition_ref: "BENCH:FORMAT".into(),
                        word: 0,
                        lsb: 0,
                        width: 2,
                        interpretation: Interpretation::Unsigned,
                        specials: Arc::from([]),
                        condition: None,
                    },
                    FieldSpec {
                        id: "payload".into(),
                        definition_ref: "BENCH:PAYLOAD".into(),
                        word: 0,
                        lsb: 2,
                        width: 32,
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

fn decode_single_word(criterion: &mut Criterion) {
    let decoder = Decoder::new(package()).unwrap();
    let message = AssembledMessage::new(vec![normalize_word(0x2af37c).unwrap()]).unwrap();
    let context = DecodeContext {
        source_id: "criterion".into(),
        source_offset: 0,
    };
    criterion.bench_function("decode_single_word_two_fields", |bencher| {
        bencher.iter(|| {
            black_box(
                decoder
                    .decode(black_box("BENCH"), black_box(&message), black_box(&context))
                    .unwrap(),
            )
        });
    });
}

fn decode_batch(criterion: &mut Criterion) {
    const BATCH_SIZE: usize = 1_024;
    let decoder = Decoder::new(package()).unwrap();
    let messages = (0..BATCH_SIZE)
        .map(|index| {
            AssembledMessage::new(vec![normalize_word((index as u128) << 2).unwrap()]).unwrap()
        })
        .collect::<Vec<_>>();
    let mut group = criterion.benchmark_group("decode_batch");
    group.throughput(Throughput::Elements(BATCH_SIZE as u64));
    group.bench_function("1024_single_word_two_fields", |bencher| {
        bencher.iter(|| {
            black_box(
                decoder
                    .decode_many(
                        black_box("BENCH"),
                        black_box(&messages),
                        "criterion".into(),
                        0,
                    )
                    .unwrap(),
            )
        });
    });
    group.finish();
}

criterion_group!(benches, decode_single_word, decode_batch);
criterion_main!(benches);
