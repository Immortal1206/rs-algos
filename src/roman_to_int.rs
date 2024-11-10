use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
  let character: HashMap<char, i32> = HashMap::from([
    ('I', 1),
    ('V', 5),
    ('X', 10),
    ('L', 50),
    ('C', 100),
    ('D', 500),
    ('M', 1000),
  ]);
  let mut prev = 'I';
  let mut res = 0i32;
  for char in s.chars().rev() {
    let cur_val = character.get(&char).unwrap();
    let prev_val = character.get(&prev).unwrap();
    prev = char;
    if cur_val >= prev_val {
      res += cur_val;
    } else {
      res -= cur_val;
    }
  }
  res
}

pub fn roman_to_int_v1(s: String) -> i32 {
  let map = HashMap::from([
    ('I', 1),
    ('V', 5),
    ('X', 10),
    ('L', 50),
    ('C', 100),
    ('D', 500),
    ('M', 1000),
  ]);
  s.chars()
    .zip(s.chars().skip(1).chain(" ".chars()))
    .map(|(a, b)| {
      let a_val = map.get(&a).unwrap_or(&0);
      let b_val = map.get(&b).unwrap_or(&0);
      if a_val >= b_val {
        *a_val
      } else {
        -*a_val
      }
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let s = "III";
    assert_eq!(3, roman_to_int(s.into()));
    let s = "LVIII";
    assert_eq!(58, roman_to_int(s.into()));
    let s = "MCMXCIV";
    assert_eq!(1994, roman_to_int(s.into()));
    let s = "III";
    assert_eq!(3, roman_to_int_v1(s.into()));
    let s = "LVIII";
    assert_eq!(58, roman_to_int_v1(s.into()));
    let s = "MCMXCIV";
    assert_eq!(1994, roman_to_int_v1(s.into()));
  }
}
