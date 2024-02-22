const VOWELS = ['a', 'e', 'i', 'o', 'u', 'y'];

function count_syllables(word) {
  word = word.toLowerCase();
  let syllable_count = 0;
  let current_syllable = [];

  for (i = 0; i < word.length - 1; i++) {
    const char = word[i];
    const last_char = current_syllable[current_syllable.length - 1];

    if (VOWELS.includes(char) && !VOWELS.includes(last_char)) {
      console.log(char, last_char)
      syllable_count++;
      current_syllable = [];
    }

    current_syllable.push(char);
   }

  return syllable_count;
}
