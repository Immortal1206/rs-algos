pub fn str_str_v1(haystack: String, needle: String) -> i32 {
  match haystack.find(&needle) {
    Some(i) => i as i32,
    None => -1,
  }
}
pub fn str_str_v2(haystack: String, needle: String) -> i32 {
  haystack.find(&needle).map(|i| i as i32).unwrap_or(-1)
}

pub fn str_str_v3(haystack: String, needle: String) -> i32 {
  let haystack_len = haystack.len();
  let needle_len = needle.len();
  if needle_len == 0 {
    return 0;
  }
  if needle_len > haystack_len {
    return -1;
  }
  for i in 0..=haystack_len - needle_len {
    if haystack[i..i + needle_len] == needle {
      return i as i32;
    }
  }
  -1
}

pub fn str_str_v4(haystack: String, needle: String) -> i32 {
  let s: Vec<u8> = needle
    .bytes()
    .chain("#".bytes())
    .chain(haystack.bytes())
    .collect();

  let mut pi = vec![0usize; s.len()];

  for i in 1..s.len() {
    let mut j = pi[i - 1];
    while j > 0 && s[j] != s[i] {
      j = pi[j - 1];
    }
    if s[i] == s[j] {
      j += 1;
    }
    pi[i] = j;
  }

  for i in needle.len() + 1..s.len() {
    if pi[i] == needle.len() {
      return (i - needle.len() * 2) as i32;
    }
  }
  -1
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let haystack = "4sadbutsad";
    let needle = "sad";
    assert_eq!(1, str_str_v1(haystack.into(), needle.into()));
    let haystack = "leetcode";
    let needle = "leeto";
    assert_eq!(-1, str_str_v1(haystack.into(), needle.into()));
    let haystack = "4sadbutsad";
    let needle = "sad";
    assert_eq!(1, str_str_v2(haystack.into(), needle.into()));
    let haystack = "leetcode";
    let needle = "leeto";
    assert_eq!(-1, str_str_v2(haystack.into(), needle.into()));
    let haystack = "4sadbutsad";
    let needle = "sad";
    assert_eq!(1, str_str_v3(haystack.into(), needle.into()));
    let haystack = "leetcode";
    let needle = "leeto";
    assert_eq!(-1, str_str_v3(haystack.into(), needle.into()));
    let haystack = "4sadbutsad";
    let needle = "sad";
    assert_eq!(1, str_str_v4(haystack.into(), needle.into()));
    let haystack = "leetcode";
    let needle = "leeto";
    assert_eq!(-1, str_str_v4(haystack.into(), needle.into()));
  }
}
