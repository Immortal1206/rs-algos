pub fn search_insert_v1(nums: Vec<i32>, target: i32) -> i32 {
  let mut left = 0;
  let mut right = nums.len();
  while left < right {
    let middle = left + ((right - left) >> 1);
    if nums[middle] == target {
      return middle as i32;
    } else if nums[middle] > target {
      right = middle - 1;
    } else {
      left = middle + 1;
    }
  }
  left as _
}

pub fn search_insert_v2(nums: Vec<i32>, target: i32) -> i32 {
  match nums.binary_search(&target) {
    Ok(x) => x as _,
    Err(y) => y as _,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    assert_eq!(2, search_insert_v1(nums, target));
    let nums = vec![1, 3, 5, 6];
    let target = 2;
    assert_eq!(1, search_insert_v1(nums, target));
    let nums = vec![1, 3, 5, 6];
    let target = 7;
    assert_eq!(4, search_insert_v1(nums, target));
  }
}
