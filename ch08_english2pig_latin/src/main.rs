const CONSONANTS: [char; 21] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'x', 'z',
    'w', 'y',
];

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn english2pig_latin(s: &str) -> String {
    let first_char = match s.chars().next() {
        Some(first_char) => first_char,
        None => return String::new(),
    };

    if CONSONANTS.contains(&first_char.to_ascii_lowercase()) {
        s.chars()
            .skip(1)
            .chain(format!("-{}ay", first_char).chars())
            .collect::<String>()
    } else if VOWELS.contains(&first_char.to_ascii_lowercase()) {
        format!("{}-hay", s)
    } else {
        String::from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_words_with_first_consonant() {
        assert_eq!(english2pig_latin("first"), "irst-fay");
    }

    #[test]
    fn converts_words_with_first_vowel() {
        assert_eq!(english2pig_latin("apple"), "apple-hay");
    }

    #[test]
    fn converts_words_with_non_alphabetical_characters() {
        assert_eq!(english2pig_latin("five-year-old"), "ive-year-old-fay");
    }

    #[test]
    fn converts_one_or_two_letter_words() {
        assert_eq!(english2pig_latin("I"), "I-hay");
        assert_eq!(english2pig_latin("me"), "e-may");
    }

    #[test]
    fn converts_non_wordy_string_to_the_same_string() {
        assert_eq!(english2pig_latin(""), "");
        assert_eq!(english2pig_latin("- _"), "- _");
    }
}

fn print_converted(s: &str) {
    println!("{} -> {}", s, english2pig_latin(s));

}

fn main() {
    print_converted("first");
    print_converted("apple");
}
