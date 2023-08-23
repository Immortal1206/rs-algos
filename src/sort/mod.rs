pub fn insertion_sort(arr: &mut [i32]) {
  for i in 1..arr.len() {
    let mut j = i;
    while j > 0 && arr[j] < arr[j - 1] {
      arr.swap(j, j - 1);
      j -= 1;
    }
  }
}
pub fn binary_insertion_sort(nums: &mut [i32]) {
  for i in 1..nums.len() {
    let mut left = 0;
    let mut right = i - 1;
    let temp = nums[i];
    while left <= right {
      let mid = (left + right) >> 1;
      if temp < nums[mid] {
        if mid == 0 {
          break;
        }
        right = mid - 1;
      } else {
        left = mid + 1;
      }
    }

    for j in (left..=i - 1).rev() {
      nums.swap(j, j + 1);
    }

    if left != i {
      nums[left] = temp;
    }
  }
}

pub fn quick_sort(arr: &mut [i32]) {
  if arr.len() <= 1 {
    return;
  }
  let pivot = partition(arr);
  quick_sort(&mut arr[0..pivot]);
  quick_sort(&mut arr[pivot + 1..]);
}
fn partition(arr: &mut [i32]) -> usize {
  let pivot = arr.len() - 1;
  // 小於pivot的元素個數，同時也是最終的pivot索引
  let mut i = 0;
  // 遍歷之後，比pivot小的元素移到i左邊，比piovt大的元素在i ~ arr.len() - 2中
  for j in 0..pivot {
    if arr[j] <= arr[pivot] {
      arr.swap(i, j);
      i += 1;
    }
  }
  // 交換pivot和arr.len() - 1，最終的pivot就是i，
  arr.swap(i, pivot);
  i
}

pub fn shell_sort(arr: &mut [i32]) {
  let len = arr.len();
  let mut gap = len / 2;
  while gap > 0 {
    for i in gap..len {
      let temp = arr[i];
      let mut j = i;
      while j >= gap && arr[j - gap] > temp {
        arr[j] = arr[j - gap];
        j -= gap;
      }
      arr[j] = temp;
    }
    gap /= 2;
  }
}

pub fn merge_sort(arr: &mut [i32]) {}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_insertion_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    insertion_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums)
  }
  #[test]
  fn test_binary_insertion_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    binary_insertion_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    binary_insertion_sort(&mut nums);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], nums);
  }
  #[test]
  fn test_quick_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    quick_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    quick_sort(&mut nums);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], nums);
  }
  #[test]
  fn test_shell_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    shell_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    shell_sort(&mut nums);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], nums);
  }
  #[test]
  fn test_merge_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    merge_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    merge_sort(&mut nums);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], nums);
  }
}
