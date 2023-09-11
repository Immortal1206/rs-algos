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
  }
}
