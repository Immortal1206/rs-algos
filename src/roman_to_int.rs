use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
  let mut character: HashMap<char, i32> = HashMap::new();
  character.insert('I', 1);
  character.insert('V', 5);
  character.insert('X', 10);
  character.insert('L', 50);
  character.insert('C', 100);
  character.insert('D', 500);
  character.insert('M', 1000);
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
  }
}
