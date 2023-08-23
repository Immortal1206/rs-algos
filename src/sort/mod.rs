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
/// 将待排序的数组逐步分割成较小的子数组，然后对这些子数组进行排序和合并，最终得到一个有序的数组
pub fn merge_sort(nums: &mut [i32]) {
  if nums.len() > 1 {
    let mid = nums.len() >> 1;
    merge_sort(&mut nums[..mid]); // 排序前半部分
    merge_sort(&mut nums[mid..]); // 排序后半部分
    merge(nums, mid); // 合并排序结果
  }
}

fn merge(nums: &mut [i32], mid: usize) {
  let mut i = 0; // 标记前半部分数据
  let mut k = mid; // 标记后半部分数据
  let mut temp = Vec::new();

  // 合并分开的两个数组
  for _j in 0..nums.len() {
    if k == nums.len() || i == mid {
      break;
    }

    // 数据放到临时集合 temp
    if nums[i] < nums[k] {
      temp.push(nums[i]);
      i += 1;
    } else {
      temp.push(nums[k]);
      k += 1;
    }
  }

  // 合并的两部分数据长度大概率不一样长
  // 所以要将未处理完的集合中数据全部加入
  if i < mid && k == nums.len() {
    for j in i..mid {
      temp.push(nums[j]);
    }
  } else if i == mid && k < nums.len() {
    for j in k..nums.len() {
      temp.push(nums[j]);
    }
  }

  // temp 中数据放回 nums，完成排序
  for j in 0..nums.len() {
    nums[j] = temp[j];
  }
}

/**
  堆排序使用最大（或最小）堆上的操作进行排序。堆是一棵完全二叉树，可以直接保存在一个数组里<br>
  第一个元素作为树的根节点，可以将左右子节点表示为 arr\[2i + 1\] 和 arr\[2i + 2\]

  借助数组表示，按照完全二叉树的节点关系，堆满足如下的定义：<br>
  1. （最大堆）大顶堆：arr\[i\] >= arr\[2i + 1\] 且 arr\[i\] >= arr\[2i + 2\]
  2. （最小堆）小顶堆：arr\[i\] <= arr\[2i + 1\] 且 arr\[i\] <= arr\[2i + 2\]
  3. 最大的节点在数组末尾，最小的节点在数组头部
*/
pub struct MaxHeap<T: Ord> {
  pub elems: Vec<T>, // 保存完全二叉树
}

impl<T: Ord> MaxHeap<T> {
  pub fn new() -> MaxHeap<T> {
    MaxHeap { elems: Vec::new() }
  }

  // 从向量创建一个最大堆
  pub fn from(elems: Vec<T>) -> MaxHeap<T> {
    let mut heap = MaxHeap { elems: elems };
    // 自底向上遍历非叶节点
    for i in (0..heap.len() / 2).rev() {
      // 下沉节点i
      heap.max_heapify(i)
    }
    heap
  }

  // 计算父节点下标
  pub fn parent(i: usize) -> usize {
    if i > 0 {
      (i - 1) / 2
    } else {
      0
    }
  }

  // 计算左子节点下标
  pub fn left(i: usize) -> usize {
    i * 2 + 1
  }

  // 计算右子节点下标
  pub fn right(i: usize) -> usize {
    i * 2 + 2
  }

  /// 对节点i进行下沉操作
  pub fn max_heapify(&mut self, i: usize) {
    let (left, right, mut largest) = (MaxHeap::<T>::left(i), MaxHeap::<T>::right(i), i);
    // 左子节点大于当前节点，交换
    if left < self.len() && self.elems[left] > self.elems[largest] {
      largest = left;
    }
    // 右子节点大于当前节点，交换
    if right < self.len() && self.elems[right] > self.elems[largest] {
      largest = right;
    }
    // 若发生交换，继续下沉节点，保证满足大顶堆的定义
    if largest != i {
      self.elems.swap(largest, i);
      self.max_heapify(largest);
    }
  }

  // 插入一个元素
  pub fn push(&mut self, v: T) {
    self.elems.push(v);
    // 上升元素
    let mut i = self.elems.len() - 1;
    while i > 0 && self.elems[MaxHeap::<T>::parent(i)] < self.elems[i] {
      self.elems.swap(i, MaxHeap::<T>::parent(i));
      i = MaxHeap::<T>::parent(i);
    }
  }

  // 弹出最大元素
  pub fn pop(&mut self) -> Option<T> {
    if self.is_empty() {
      None
    } else {
      let b = self.elems.len() - 1;
      self.elems.swap(0, b);
      let v = self.elems.pop();
      if !self.is_empty() {
        // 下沉根节点
        self.max_heapify(0);
      }
      v
    }
  }

  pub fn is_empty(&self) -> bool {
    self.elems.is_empty()
  }

  pub fn len(&self) -> usize {
    self.elems.len()
  }
}

/**
  堆排序的基本思想是：将待排序序列构造成一个小顶堆，此时，整个序列的最小值就是堆顶根节点。<br>
  将其与末尾元素进行交换，此时末尾就为最小值。这个最小值不再计算到堆内，<br>
  那么再将剩余的 n - 1 个元素重新构造成一个堆，这样会得到一个新的最小值。<br>
  此时将该最小值再次交换到新堆的末尾，这样就有了两个排序的值。重复这个过程，直到得到一个有序序列。<br>
  当然，小顶堆得到的是降序排序，大顶堆得到的才是升序排序。
*/
pub fn max_heap_sort(nums: &mut Vec<usize>) {
  let mut heap: MaxHeap<usize> = MaxHeap::from(nums.clone());
  for i in (0..nums.len()).rev() {
    nums[i] = heap.pop().unwrap();
  }
}

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
  #[test]
  fn test_heap_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    max_heap_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    max_heap_sort(&mut nums);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], nums);
  }
}
