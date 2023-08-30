use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
  let mut hash: HashMap<char, i32> = HashMap::new();
  let mut substring_len = 0;
  let mut start_idx = -1;
  for (idx, char) in s.chars().enumerate() {
    if let Some(i) = hash.insert(char, idx as i32) {
      start_idx = start_idx.max(i);
    }
    substring_len = substring_len.max(idx as i32 - start_idx);
  }
  substring_len
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let s = "abcabcbb";
    assert_eq!(3, length_of_longest_substring(s.into()));
    let s = "bbbbb";
    assert_eq!(1, length_of_longest_substring(s.into()));
    let s = "pwwkew";
    assert_eq!(3, length_of_longest_substring(s.into()));
  }
}
