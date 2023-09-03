use std::collections::HashSet;

pub fn contains_duplicate_v1(nums: Vec<i32>) -> bool {
  nums.len() != HashSet::<i32>::from_iter(nums).len()
}
pub fn contains_duplicate_v2(nums: Vec<i32>) -> bool {
  let mut exists = HashSet::new();
  !nums.into_iter().all(|n| exists.insert(n))
}
pub fn contains_duplicate_v3(nums: Vec<i32>) -> bool {
  let mut exists = HashSet::new();
  nums.into_iter().any(|n| !exists.insert(n))
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let s = vec![7, 1, 5, 3, 5, 6, 4];
    assert_eq!(true, contains_duplicate_v1(s));
    let s = vec![7, 6, 4, 3, 1];
    assert_eq!(false, contains_duplicate_v1(s));
    let s = vec![7, 1, 5, 3, 7, 6, 4];
    assert_eq!(true, contains_duplicate_v2(s));
    let s = vec![7, 6, 4, 3, 1];
    assert_eq!(false, contains_duplicate_v2(s));
    let s = vec![7, 1, 5, 3, 7, 6, 4];
    assert_eq!(true, contains_duplicate_v3(s));
    let s = vec![7, 6, 4, 3, 1];
    assert_eq!(false, contains_duplicate_v3(s));
  }
}
