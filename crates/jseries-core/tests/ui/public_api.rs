use jseries_core::{InformationWord, normalize_logical70};

fn accepts_word(word: InformationWord) -> u128 {
    word.value()
}

fn main() {
    let normalized = normalize_logical70(0x1234).unwrap();
    assert_eq!(accepts_word(normalized.information), 0x1234);
}
