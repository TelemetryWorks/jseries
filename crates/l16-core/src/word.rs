use std::fmt;

/// Internal convention: bit zero is the least-significant bit of the integer.
/// No statement is made about capture byte order, padding, or wire bit order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Word { value: u128, bit_len: u8 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BitRange { lsb: u8, width: u8 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WordError { InvalidLength, HighBitsSet, InvalidRange }
impl fmt::Display for WordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self { Self::InvalidLength => "word length must be 1..=128",
            Self::HighBitsSet => "input has set bits above the declared word length",
            Self::InvalidRange => "field range is empty or exceeds the word" })
    }
}
impl std::error::Error for WordError {}
/// Low-bit mask without shifting by 128; width is validated by callers.
pub(crate) const fn mask(width: u8) -> u128 {
    if width == 128 { u128::MAX } else { (1u128 << width) - 1 }
}
impl Word {
    pub fn new(value: u128, bit_len: u8) -> Result<Self, WordError> {
        if bit_len == 0 || bit_len > 128 { return Err(WordError::InvalidLength); }
        if value & !mask(bit_len) != 0 { return Err(WordError::HighBitsSet); }
        Ok(Self { value, bit_len })
    }
    pub const fn value(self) -> u128 { self.value }
    pub const fn bit_len(self) -> u8 { self.bit_len }
    pub fn extract(self, range: BitRange) -> Result<u128, WordError> {
        if u16::from(range.lsb) + u16::from(range.width) > u16::from(self.bit_len) {
            return Err(WordError::InvalidRange);
        }
        Ok((self.value >> range.lsb) & mask(range.width))
    }
}
impl BitRange {
    pub fn new(lsb: u8, width: u8, word_bits: u8) -> Result<Self, WordError> {
        if word_bits == 0 || word_bits > 128 || width == 0 || width > 128
            || u16::from(lsb) + u16::from(width) > u16::from(word_bits) {
            return Err(WordError::InvalidRange);
        }
        Ok(Self { lsb, width })
    }
    pub const fn lsb(self) -> u8 { self.lsb }
    pub const fn width(self) -> u8 { self.width }
}
