pub fn remove_element_v1(nums: &mut Vec<i32>, val: i32) -> i32 {
  nums.retain(|&n| n != val);
  nums.len() as i32
}

pub fn remove_element_v2(nums: &mut Vec<i32>, val: i32) -> i32 {
  // note that each iteration need to create a new iterator since position will change iterator
  while let Some(index) = nums.iter().position(|v| *v == val) {
    nums.remove(index);
  }

  nums.len() as i32
}

pub fn remove_element_v3(nums: &mut Vec<i32>, val: i32) -> i32 {
  let mut preserved = 0;
  for i in 0..nums.len() {
    if nums[i] != val {
      nums[preserved] = nums[i];
      preserved += 1;
    }
  }
  nums.truncate(preserved);
  preserved as i32
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let mut v = vec![3, 2, 2, 3];
    let num = 3;
    let i = remove_element_v1(&mut v, num);
    println!("{:?}  {}", v, i);
    let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let num = 2;
    let i = remove_element_v1(&mut v, num);
    println!("{:?}  {}", v, i);
    let mut v = vec![3, 2, 2, 3];
    let num = 3;
    let i = remove_element_v2(&mut v, num);
    println!("{:?}  {}", v, i);
    let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let num = 2;
    let i = remove_element_v2(&mut v, num);
    println!("{:?}  {}", v, i);
    let mut v = vec![3, 2, 2, 3];
    let num = 3;
    let i = remove_element_v3(&mut v, num);
    println!("{:?}  {}", v, i);
    let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let num = 2;
    let i = remove_element_v3(&mut v, num);
    println!("{:?}  {}", v, i);
  }
}
