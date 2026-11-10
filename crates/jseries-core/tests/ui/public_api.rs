use jseries_core::{InformationWord, normalize_word};

fn accepts_word(word: InformationWord) -> u128 {
    word.value()
}

fn main() {
    let word = normalize_word(0x1234).unwrap();
    assert_eq!(accepts_word(word), 0x1234);
}
