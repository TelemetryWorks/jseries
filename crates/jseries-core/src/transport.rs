use crate::word::{INFORMATION_BITS, InformationWord, WordError, mask};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputFormat {
    Logical70,
    Word75,
    Simple80,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NormalizedWord {
    pub information: InformationWord,
    pub parity: Option<u8>,
    pub source_format: InputFormat,
    pub source_value: u128,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NormalizeError {
    Word(WordError),
    HighBitsSet,
    NonzeroPadding(u8),
}

impl fmt::Display for NormalizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Word(error) => write!(f, "{error}"),
            Self::HighBitsSet => f.write_str("input has bits above its declared representation"),
            Self::NonzeroPadding(value) => {
                write!(
                    f,
                    "SIMPLE 80-bit padding must be zero; received 0x{value:02x}"
                )
            }
        }
    }
}

impl std::error::Error for NormalizeError {}

impl From<WordError> for NormalizeError {
    fn from(error: WordError) -> Self {
        Self::Word(error)
    }
}

pub fn normalize_logical70(value: u128) -> Result<NormalizedWord, NormalizeError> {
    Ok(NormalizedWord {
        information: InformationWord::new(value)?,
        parity: None,
        source_format: InputFormat::Logical70,
        source_value: value,
    })
}

pub fn normalize_word75(value: u128) -> Result<NormalizedWord, NormalizeError> {
    if value & !mask(75) != 0 {
        return Err(NormalizeError::HighBitsSet);
    }
    Ok(NormalizedWord {
        information: InformationWord::new(value & mask(INFORMATION_BITS))?,
        parity: Some(((value >> INFORMATION_BITS) & 0x1f) as u8),
        source_format: InputFormat::Word75,
        source_value: value,
    })
}

pub fn normalize_simple80(bytes: [u8; 10]) -> Result<NormalizedWord, NormalizeError> {
    let mut value = 0u128;
    for (index, byte) in bytes.into_iter().enumerate() {
        value |= u128::from(byte) << (index * 8);
    }
    let padding = ((value >> 75) & 0x1f) as u8;
    if padding != 0 {
        return Err(NormalizeError::NonzeroPadding(padding));
    }
    Ok(NormalizedWord {
        information: InformationWord::new(value & mask(INFORMATION_BITS))?,
        parity: Some(((value >> INFORMATION_BITS) & 0x1f) as u8),
        source_format: InputFormat::Simple80,
        source_value: value,
    })
}
