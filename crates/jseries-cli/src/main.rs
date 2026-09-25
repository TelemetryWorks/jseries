#![forbid(unsafe_code)]
use jseries_core::{
    AssembledMessage, DecodeContext, Decoder, FieldStatus, PACKAGE_VERSION, normalize_logical70,
    normalize_simple80, normalize_word75,
};
use jseries_schema::{load_package, parse_raw};
use std::{env, process::ExitCode};

const HELP: &str = "jseries — schema-driven 70-bit information-word decoder\n\
Usage:\n\
  jseries --version\n\
  jseries schema validate <package-directory>\n\
  jseries schema inspect <package-directory>\n\
  jseries decode --schema DIR --message ID --input-format logical70|word75|simple80 --word VALUE [--word VALUE ...]\n\
logical70 and word75 values are decimal or lowercase 0x hex. simple80 is exactly 20 hex digits in capture byte order.";

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("ERROR: {message}");
            ExitCode::from(2)
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    match args.as_slice() {
        [] => {
            println!("{HELP}");
            Ok(())
        }
        [help] if help == "--help" || help == "-h" || help == "help" => {
            println!("{HELP}");
            Ok(())
        }
        [version] if version == "--version" || version == "-V" => {
            println!("jseries {PACKAGE_VERSION}");
            Ok(())
        }
        [schema, action, directory]
            if schema == "schema" && (action == "validate" || action == "inspect") =>
        {
            let loaded = load_package(directory).map_err(|e| e.to_string())?;
            println!(
                "valid package={} version={} messages={} vectors={}",
                loaded.schema.metadata.id,
                loaded.schema.metadata.version,
                loaded.schema.messages.len(),
                loaded.vectors.len()
            );
            if action == "inspect" {
                for message in &*loaded.schema.messages {
                    println!(
                        "message={} words={} fields={}",
                        message.id,
                        message.word_count,
                        message.fields.len()
                    );
                }
            }
            Ok(())
        }
        [command, rest @ ..] if command == "decode" => decode(rest),
        _ => Err(format!("invalid command\n{HELP}")),
    }
}

fn decode(args: &[String]) -> Result<(), String> {
    let mut schema = None;
    let mut message_id = None;
    let mut format = None;
    let mut words = Vec::new();
    let mut index = 0;
    while index < args.len() {
        let value = args
            .get(index + 1)
            .ok_or_else(|| format!("missing value for {}", args[index]))?;
        match args[index].as_str() {
            "--schema" => schema = Some(value.as_str()),
            "--message" => message_id = Some(value.as_str()),
            "--input-format" => format = Some(value.as_str()),
            "--word" => words.push(value.as_str()),
            option => return Err(format!("unknown option {option}")),
        }
        index += 2;
    }
    let schema = schema.ok_or("missing --schema")?;
    let message_id = message_id.ok_or("missing --message")?;
    let format = format.ok_or("missing --input-format")?;
    if words.is_empty() {
        return Err("at least one --word is required".into());
    }
    let normalized = words
        .into_iter()
        .map(|word| match format {
            "logical70" => normalize_logical70(parse_raw(word).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string()),
            "word75" => normalize_word75(parse_raw(word).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string()),
            "simple80" => normalize_simple80(parse_simple80(word)?).map_err(|e| e.to_string()),
            _ => Err("input format must be logical70, word75, or simple80".into()),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let assembled = AssembledMessage::new(normalized).map_err(|e| e.to_string())?;
    let loaded = load_package(schema).map_err(|e| e.to_string())?;
    let decoder = Decoder::new(loaded.schema).map_err(|e| e.to_string())?;
    let record = decoder
        .decode(
            message_id,
            &assembled,
            &DecodeContext {
                source_id: "cli".into(),
                source_offset: 0,
            },
        )
        .map_err(|e| e.to_string())?;
    println!(
        "package={} version={} qualification={} message={}",
        record.package_id, record.package_version, record.qualification, record.message_id
    );
    for field in record.fields {
        println!(
            "field={} word={} bits={}..{} raw={} status={}",
            field.id,
            field.word,
            field.lsb,
            u16::from(field.lsb) + u16::from(field.width) - 1,
            field.raw,
            status(&field.status)
        );
    }
    Ok(())
}

fn parse_simple80(text: &str) -> Result<[u8; 10], String> {
    if text.len() != 20 || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("simple80 must contain exactly 20 hexadecimal digits".into());
    }
    let mut bytes = [0; 10];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte =
            u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).map_err(|e| e.to_string())?;
    }
    Ok(bytes)
}
fn status(value: &FieldStatus) -> String {
    format!("{value:?}")
}
