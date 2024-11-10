pub fn length_of_last_word_v1(s: String) -> i32 {
  match s.split_whitespace().last() {
    Some(x) => x.len() as i32,
    None => 0,
  }
}

pub fn length_of_last_word_v2(s: String) -> i32 {
  s.trim_end().split(" ").last().map_or(0, |s| s.len() as i32)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn test_length_of_last_word() {
    assert_eq!(5, length_of_last_word_v1("Hello World".to_string()));
    assert_eq!(
      4,
      length_of_last_word_v1("   fly me   to   the moon  ".to_string())
    );
    assert_eq!(
      6,
      length_of_last_word_v1("luffy is still joyboy".to_string())
    );
    assert_eq!(0, length_of_last_word_v2("".to_string()));
    assert_eq!(5, length_of_last_word_v2("Hello World".to_string()));
    assert_eq!(
      4,
      length_of_last_word_v2("   fly me   to   the moon  ".to_string())
    );
    assert_eq!(
      6,
      length_of_last_word_v2("luffy is still joyboy".to_string())
    );
    assert_eq!(0, length_of_last_word_v2("".to_string()));
  }
}
