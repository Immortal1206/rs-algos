pub fn plus_one_v1(digits: Vec<i32>) -> Vec<i32> {
  let mut digits = digits;
  let n = digits.len();
  for i in (0..n).rev() {
    if digits[i] < 9 {
      digits[i] += 1;
      return digits;
    }
    digits[i] = 0;
  }
  let mut new_digits = vec![0; n + 1];
  new_digits[0] = 1;
  new_digits
}

pub fn plus_one_v2(mut digits: Vec<i32>) -> Vec<i32> {
  for x in digits.iter_mut().rev() {
    match *x == 9 {
      true => *x = 0,
      false => {
        *x += 1;
        return digits;
      }
    }
  }
  digits.insert(0, 1);
  digits
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_plus_one() {
    let input = vec![1, 2, 3];
    let output = vec![1, 2, 4];
    assert_eq!(plus_one_v1(input), output);
    let input = vec![9, 9, 9];
    let output = vec![1, 0, 0, 0];
    assert_eq!(plus_one_v1(input), output);
    let input = vec![1, 2, 3];
    let output = vec![1, 2, 4];
    assert_eq!(plus_one_v2(input), output);
    let input = vec![9, 9, 9];
    let output = vec![1, 0, 0, 0];
    assert_eq!(plus_one_v2(input), output);
  }
}
