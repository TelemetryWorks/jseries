#![forbid(unsafe_code)]
use l16_core::{Baseline, DecodeContext, Decoder, RunMode, SelectionEvidence, Word};
use std::{collections::HashMap, env, process::ExitCode};

const HELP: &str = "l16-inspect: SYNTHETIC decoder foundation, not a Link 16 decoder\n\
Commands:\n\
  baselines\n\
  demo --baseline D|E|F|F-C1|G|H --word 0x0765 --evidence TEXT\n\
       [--message SYNTH-01] [--source TEXT] [--offset INTEGER]\n\
  decode --baseline D|E|F|F-C1|G|H\n\
The demo consumes a normalized toy integer, not captured Link 16 bytes.\n\
The decode command always refuses: no authoritative schemas are bundled.\n\
Exit codes: 0 success; 2 argument error; 3 schema/baseline unavailable;\n\
            4 malformed normalized input. Field statuses are data, not exit errors.";

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err((code, message)) => { eprintln!("ERROR: {message}"); ExitCode::from(code) }
    }
}
fn run(args: Vec<String>) -> Result<(), (u8, String)> {
    let Some(command) = args.first().map(String::as_str) else { println!("{HELP}"); return Ok(()); };
    match command {
        "--help" | "-h" | "help" => { println!("{HELP}"); return Ok(()); }
        "baselines" => {
            if args.len() != 1 { return Err((2, "baselines takes no options".into())); }
            println!("baseline | authoritative_schema | verified_real_messages");
            for b in Baseline::ALL { println!("{b} | NOT_POPULATED | 0"); }
            return Ok(());
        }
        "demo" | "decode" => {}
        _ => return Err((2, format!("unknown command {command:?}\n{HELP}"))),
    }
    let allowed: &[&str] = if command == "decode" { &["--baseline"] }
        else { &["--baseline", "--word", "--evidence", "--message", "--source", "--offset"] };
    let mut options = HashMap::new();
    let mut iter = args[1..].iter();
    while let Some(key) = iter.next() {
        if !allowed.contains(&key.as_str()) { return Err((2, format!("unknown option {key:?}"))); }
        let value = iter.next().ok_or_else(|| (2, format!("missing value for {key}")))?;
        if value.starts_with("--") { return Err((2, format!("missing value for {key}"))); }
        if options.insert(key.as_str(), value.as_str()).is_some() {
            return Err((2, format!("duplicate option {key}")));
        }
    }
    let required = |key: &str| options.get(key).copied().ok_or_else(|| (2, format!("missing {key}")));
    let baseline = required("--baseline")?.parse::<Baseline>().map_err(|e| (2, e.to_string()))?;
    let mode = if command == "demo" { RunMode::SyntheticTest } else { RunMode::StandardsDecode };
    let decoder = Decoder::new(baseline, mode).map_err(|e| (3, e.to_string()))?;
    let evidence = SelectionEvidence::new(required("--evidence")?).map_err(|e| (2, e.to_string()))?;
    let text = required("--word")?;
    if text.len() > 34 { return Err((4, "normalized word is too long".into())); }
    let raw = match text.strip_prefix("0x") {
        Some(hex) if !hex.is_empty() => u128::from_str_radix(hex, 16),
        _ => text.parse::<u128>(),
    }.map_err(|e| (4, format!("invalid normalized integer: {e}")))?;
    let message = options.get("--message").copied().unwrap_or("SYNTH-01");
    let bits = decoder.message_word_bits(message)
        .ok_or_else(|| (3, format!("message {message:?} is unavailable for {baseline}; no fallback")))?;
    let word = Word::new(raw, bits).map_err(|e| (4, e.to_string()))?;
    let offset = options.get("--offset").copied().unwrap_or("0").parse::<u64>()
        .map_err(|e| (2, format!("invalid source offset: {e}")))?;
    let context = DecodeContext { source_id: options.get("--source").copied()
        .unwrap_or("synthetic-cli-input").into(), source_offset: offset };
    let record = decoder.decode(message, word, &evidence, &context).map_err(|e| (4, e.to_string()))?;
    println!("SYNTHETIC ONLY — not MIL-STD-6016 message semantics");
    println!("baseline={} schema={} version={} profile={}", record.baseline, record.schema_id,
        record.schema_version, record.profile_id);
    println!("schema_sha256={}", record.schema_digest_sha256);
    println!("decoder={} output={}", record.decoder_version, record.output_version);
    println!("selection_evidence={:?}", record.selection_evidence);
    println!("source={:?} offset={}", record.source_id, record.source_offset);
    println!("message={} logical_bits={} logical_word=0x{:x}", record.message_id,
        record.original_word.bit_len(), record.original_word.value());
    for field in record.fields {
        println!("field={} definition={} bits={}..{} raw={} status={:?}", field.id,
            field.definition_ref, field.lsb, u16::from(field.lsb)+u16::from(field.width)-1,
            field.raw, field.status);
    }
    Ok(())
}
