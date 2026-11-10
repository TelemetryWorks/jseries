use crate::{schema::MAX_WORDS_PER_MESSAGE, word::InformationWord};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WordFormat {
    Initial,
    Continuation,
    Extension,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssembledMessage {
    words: Vec<InformationWord>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssemblyError {
    Empty,
    TooManyWords,
    MissingInitial,
    UnexpectedInitial,
    ReservedWordFormat,
}

impl fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Empty => "message contains no words",
            Self::TooManyWords => "message exceeds the configured word bound",
            Self::MissingInitial => "the first word is not an initial word",
            Self::UnexpectedInitial => "an additional initial word starts a new message",
            Self::ReservedWordFormat => "word format 3 is reserved or unsupported",
        })
    }
}

impl std::error::Error for AssemblyError {}

impl InformationWord {
    pub fn word_format(self) -> Result<WordFormat, AssemblyError> {
        match self.value() & 0b11 {
            0 => Ok(WordFormat::Initial),
            1 => Ok(WordFormat::Continuation),
            2 => Ok(WordFormat::Extension),
            _ => Err(AssemblyError::ReservedWordFormat),
        }
    }
}

impl AssembledMessage {
    pub fn new(words: Vec<InformationWord>) -> Result<Self, AssemblyError> {
        if words.is_empty() {
            return Err(AssemblyError::Empty);
        }
        if words.len() > usize::from(MAX_WORDS_PER_MESSAGE) {
            return Err(AssemblyError::TooManyWords);
        }
        if words[0].word_format()? != WordFormat::Initial {
            return Err(AssemblyError::MissingInitial);
        }
        for word in &words[1..] {
            if word.word_format()? == WordFormat::Initial {
                return Err(AssemblyError::UnexpectedInitial);
            }
        }
        Ok(Self { words })
    }

    pub fn words(&self) -> &[InformationWord] {
        &self.words
    }
}
