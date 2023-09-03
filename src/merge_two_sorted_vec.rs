pub fn merge_v1(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
  nums1.truncate(m as usize);
  nums2.truncate(n as usize);
  nums1.append(nums2);
  nums1.sort();
}

pub fn merge_v2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
  for idx in 0..n {
    nums1[m as usize + idx as usize] = nums2[idx as usize]
  }
  nums1.sort()
}

pub fn merge_v3(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
  let (mut m, mut n) = (m as usize, n as usize);

  while n > 0 {
    if m > 0 && nums1[m - 1] > nums2[n - 1] {
      nums1[m + n - 1] = nums1[m - 1];
      m -= 1;
    } else {
      nums1[m + n - 1] = nums2[n - 1];
      n -= 1;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let mut s1 = vec![1, 2, 3, 0, 0, 0];
    let mut s2 = vec![2, 5, 6];
    merge_v1(&mut s1, 3, &mut s2, 3);
    assert_eq!(vec![1, 2, 2, 3, 5, 6], s1);
    let mut s1 = vec![0];
    let mut s2 = vec![2, 5, 6];
    merge_v1(&mut s1, 0, &mut s2, 1);
    assert_eq!(vec![2], s1);
    let mut s1 = vec![1];
    let mut s2 = vec![];
    merge_v1(&mut s1, 1, &mut s2, 0);
    assert_eq!(vec![1], s1);
    let mut s1 = vec![1, 2, 3, 0, 0, 0];
    let mut s2 = vec![2, 5, 6];
    merge_v2(&mut s1, 3, &mut s2, 3);
    assert_eq!(vec![1, 2, 2, 3, 5, 6], s1);
    let mut s1 = vec![0];
    let mut s2 = vec![2, 5, 6];
    merge_v2(&mut s1, 0, &mut s2, 1);
    assert_eq!(vec![2], s1);
    let mut s1 = vec![1];
    let mut s2 = vec![];
    merge_v2(&mut s1, 1, &mut s2, 0);
    assert_eq!(vec![1], s1);
    let mut s1 = vec![1, 2, 3, 0, 0, 0];
    let mut s2 = vec![2, 5, 6];
    merge_v3(&mut s1, 3, &mut s2, 3);
    assert_eq!(vec![1, 2, 2, 3, 5, 6], s1);
    let mut s1 = vec![0];
    let mut s2 = vec![2, 5, 6];
    merge_v3(&mut s1, 0, &mut s2, 1);
    assert_eq!(vec![2], s1);
    let mut s1 = vec![1];
    let mut s2 = vec![];
    merge_v3(&mut s1, 1, &mut s2, 0);
    assert_eq!(vec![1], s1);
  }
}
