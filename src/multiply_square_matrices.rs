pub fn matrix_multiply_v1(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let n = a.len();
  let mut c = vec![vec![0; n]; n];
  for i in 0..n {
    for j in 0..n {
      for k in 0..n {
        c[i][j] += a[i][k] * b[k][j];
      }
    }
  }
  c
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_v1() {
    let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let b = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];

    let c = matrix_multiply_v1(&a, &b);

    for row in c {
      println!("{:?}", row);
    }
  }
}
