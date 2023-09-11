pub fn remove_duplicates_v1(nums: &mut Vec<i32>) -> i32 {
  nums.dedup();
  nums.len() as i32
}

pub fn remove_duplicates_v2(nums: &mut Vec<i32>) -> i32 {
  let mut write = 1;
  for i in 1..nums.len() {
    if nums[i] != nums[i - 1] {
      nums[write] = nums[i];
      write += 1;
    }
  }
  write as i32
}

/// note that if the first element is i32::MIN,this solution is wrong
pub fn remove_duplicates_v3(nums: &mut Vec<i32>) -> i32 {
  let mut previous: i32 = i32::MIN;
  nums.retain(|&n| match n == previous {
    true => false,
    false => {
      previous = n;
      true
    }
  });
  nums.len() as i32
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let i = remove_duplicates_v1(&mut v);
    println!("{:?}  {}", v, i);
    let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let i = remove_duplicates_v2(&mut v);
    println!("{:?}  {}", v, i);
    let mut v = vec![0, 1, 1, 1, 1, 2, 2, 3, 3, 4];
    let i = remove_duplicates_v2(&mut v);
    println!("{:?}  {}", v, i);
    let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let i = remove_duplicates_v3(&mut v);
    println!("{:?}  {}", v, i);
    let mut v = vec![i32::MIN, 1, 1, 1, 1, 2, 2, 3, 3, 4];
    let i = remove_duplicates_v3(&mut v);
    println!("{:?}  {}", v, i);
  }
}
