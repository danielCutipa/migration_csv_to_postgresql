pub fn is_number(text: &str) -> bool {
  let test = text.parse::<u64>();
  match test {
    Ok(_) => true,
    Err(_) => false,
  }
}

pub fn is_upper(word: &str) -> bool {
  // let mut ascii = Vec::new();
  // for elem in 65..91 {
  //     ascii.push(elem);
  // }
  // let ascii_word = &word.as_bytes();
  // let ascii_word = ascii_word[0];
  // ascii.contains(&ascii_word)

  let word_char = word.chars().next().expect("string is empty");
  word_char.is_ascii_uppercase()
}

pub fn is_lower(word: &str) -> bool {
  // let mut ascii = Vec::new();
  // for elem in 97..123 {
  //     ascii.push(elem);
  // }
  // let ascii_word = &word.as_bytes();
  // let ascii_word = ascii_word[0];
  // ascii.contains(&ascii_word)

  let word_char = word.chars().next().expect("string is empty");
  word_char.is_ascii_lowercase()
}

pub fn is_dot(word: &str) -> bool {
  let ascii_word = &word.as_bytes();
  ascii_word.contains(&46)
}

pub fn is_middle_dash(word: &str) -> bool {
  let ascii_word = &word.as_bytes();
  ascii_word.contains(&45)
}

pub fn is_underscore(word: &str) -> bool {
  let ascii_word = &word.as_bytes();
  ascii_word.contains(&95)
}

pub fn is_upper_enie(word: &str) -> bool {
  let ascii_word = &word.as_bytes();
  ascii_word.contains(&195) && ascii_word.contains(&145)
}

pub fn is_two_words(text: &str) -> bool {
  let words: Vec<&str> = text.split(' ').collect();
  words.len() >= 2
}
