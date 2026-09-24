use std::{fmt, str::FromStr};

/// Document identity only: a variant does not establish message coverage.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Baseline { D, E, F, FChange1, G, H }

impl Baseline {
    pub const ALL: [Self; 6] = [Self::D, Self::E, Self::F, Self::FChange1, Self::G, Self::H];
    pub const fn id(self) -> &'static str {
        match self { Self::D => "D", Self::E => "E", Self::F => "F",
            Self::FChange1 => "F-C1", Self::G => "G", Self::H => "H" }
    }
}
impl fmt::Display for Baseline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.id()) }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseBaselineError(pub String);
impl fmt::Display for ParseBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown baseline {:?}; expected D, E, F, F-C1, G, or H", self.0)
    }
}
impl std::error::Error for ParseBaselineError {}
impl FromStr for Baseline {
    type Err = ParseBaselineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s { "D" => Ok(Self::D), "E" => Ok(Self::E), "F" => Ok(Self::F),
            "F-C1" => Ok(Self::FChange1), "G" => Ok(Self::G), "H" => Ok(Self::H),
            _ => Err(ParseBaselineError(s.into())) }
    }
}
