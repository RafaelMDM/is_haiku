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

pub fn count_syllables(word: &str) -> usize {
    let characters: Vec<Character> = word.chars().map(|x| x.into()).collect();
    let number_of_chars = characters.len();
    let mut syllable_count = 0;
    let mut current_syllable: Vec<Character> = Vec::new();

    for (idx, character) in characters.into_iter().enumerate() {
        let last_character = current_syllable.last();

        match character {
            Character::Vowel('e') if idx == number_of_chars - 1 && number_of_chars > 3 => {
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
        assert_eq!(count_syllables("silence"), 2);
        assert_eq!(count_syllables("execute"), 4);
    }
}
