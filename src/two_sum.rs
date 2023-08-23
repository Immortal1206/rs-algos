use std::collections::HashMap;

pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut map = HashMap::with_capacity(nums.len());
  for (index, num) in nums.into_iter().enumerate() {
    match map.get(&num) {
      Some(&j) => return vec![j as i32, index as i32],
      None => {
        map.insert(target - num, index);
      }
    }
  }
  unreachable!()
}
