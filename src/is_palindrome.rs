pub fn is_palindrome_v1(x: i32) -> bool {
  if x < 0 {
    return false;
  }
  let x = x.to_string();
  let x: Vec<char> = x.chars().collect();
  let len = x.len();
  for i in 0..len / 2 {
    if x[i] != x[len - 1 - i] {
      return false;
    }
  }
  true
}

pub fn is_palindrome_v2(x: i32) -> bool {
  x.to_string().chars().eq(x.to_string().chars().rev())
}

pub fn is_palindrome_v3(x: i32) -> bool {
  if x < 0 {
    return false;
  }
  let x = x.to_string();
  let len = x.len();
  for i in 0..len / 2 {
    if x.chars().nth(i).unwrap() != x.chars().nth(&len - 1 - i).unwrap() {
      return false;
    }
  }
  true
}

pub fn is_palindrome_v4(num: i32) -> bool {
  if num < 0 {
    return false;
  }

  let mut original_num = num;
  let mut reversed_num = 0;

  while original_num > 0 {
    let digit = original_num % 10;
    reversed_num = reversed_num * 10 + digit;
    original_num /= 10;
  }

  num == reversed_num
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let x = 121;
    assert_eq!(is_palindrome_v1(x), true);
    let x = -121;
    assert_eq!(is_palindrome_v1(x), false);
    let x = 16223921;
    assert_eq!(is_palindrome_v1(x), false);

    let x = 121;
    assert_eq!(is_palindrome_v2(x), true);
    let x = -121;
    assert_eq!(is_palindrome_v2(x), false);
    let x = 16223921;
    assert_eq!(is_palindrome_v2(x), false);

    let x = 121;
    assert_eq!(is_palindrome_v3(x), true);
    let x = -121;
    assert_eq!(is_palindrome_v3(x), false);
    let x = 16223921;
    assert_eq!(is_palindrome_v3(x), false);

    let x = 121;
    assert_eq!(is_palindrome_v4(x), true);
    let x = -121;
    assert_eq!(is_palindrome_v4(x), false);
    let x = 16223921;
    assert_eq!(is_palindrome_v4(x), false);
  }
}
