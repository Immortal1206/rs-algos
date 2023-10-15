pub fn evaluate_v1(coefficients: Vec<i32>, x: i32) -> i32 {
  let mut res = 0;
  for (i, a) in coefficients.iter().enumerate() {
    let mut temp = 1;
    // calculate x to the ith power
    for _ in 0..i {
      temp = temp * x as i32;
    }
    res = res + a * temp;
  }
  res
}
pub fn evaluate_v2(coefficients: Vec<i32>, x: i32) -> i32 {
  let mut res = 0;
  let mut temp = 1;
  for (i, a) in coefficients.iter().enumerate() {
    // calculate x to the ith power, note x to the 0 power is 1
    if i > 0 {
      temp = temp * x as i32;
    }
    res = res + a * temp;
  }
  res
}

pub fn evaluate_v3(coefficients: Vec<i32>, x: i32) -> i32 {
  let mut res = 0;
  for i in (0..coefficients.len()).rev() {
    res = coefficients[i] + res * x;
  }
  res
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn test_evaluate_v1() {
    assert_eq!(632, evaluate_v1(vec![5, 2, 3, 4, 6], 3));
  }
  #[test]
  pub fn test_evaluate_v2() {
    assert_eq!(632, evaluate_v2(vec![5, 2, 3, 4, 6], 3));
  }
  #[test]
  pub fn test_evaluate_v3() {
    assert_eq!(632, evaluate_v3(vec![5, 2, 3, 4, 6], 3));
  }
}
