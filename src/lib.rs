#[derive(Debug, PartialEq, Eq)]
enum Character {
    Vowel(char),
    Consonant,
}

impl From<char> for Character {
    fn from(value: char) -> Self {
        let value = value.to_ascii_lowercase();
        match value {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => Self::Vowel(value),
            _ => Self::Consonant,
        }
    }
}

fn is_silent_e(word: &str, idx: usize, word_len: usize) -> bool {
    if idx != word_len - 1 {
        return false;
    }

    if word_len <= 3 {
        return true;
    }

    let char_one_space_before = word.chars().nth(idx - 1).unwrap();
    let char_two_spaces_before: Character = word.chars().nth(idx - 2).unwrap().into();
    if char_one_space_before == 'l' && char_two_spaces_before == Character::Consonant {
        return false;
    }

    if let Character::Vowel(_) = char_two_spaces_before {
        if char_one_space_before == 't' {
            return true;
        }
        return false;
    }

    true
}

pub fn count_syllables(word: &str) -> usize {
    let characters: Vec<Character> = word.chars().map(|x| x.into()).collect();
    let word_len = characters.len();
    let mut syllable_count = 0;
    let mut current_syllable: Vec<Character> = Vec::new();

    for (idx, character) in characters.into_iter().enumerate() {
        let last_character = current_syllable.last();

        match character {
            Character::Vowel('e') if is_silent_e(word, idx, word_len) => {
                break;
            }
            Character::Vowel(_) => {
                if let Some(Character::Vowel(_)) = last_character {
                    continue;
                }

                syllable_count += 1;
                current_syllable.clear();
            }
            Character::Consonant => (),
        }

        current_syllable.push(character);
    }

    if syllable_count == 0 {
        syllable_count = 1;
    }

    println!("Word: {}, syllables: {}", word, syllable_count);
    syllable_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_words() {
        assert_eq!(count_syllables("word"), 1);
        assert_eq!(count_syllables("the"), 1);
        assert_eq!(count_syllables("baby"), 2);
        assert_eq!(count_syllables("frog"), 1);
        assert_eq!(count_syllables("haiku"), 2);
    }

    #[test]
    fn silent_e() {
        assert_eq!(count_syllables("ape"), 1);
        assert_eq!(count_syllables("little"), 2);
        assert_eq!(count_syllables("silence"), 2);
        assert_eq!(count_syllables("execute"), 3);
    }
}
