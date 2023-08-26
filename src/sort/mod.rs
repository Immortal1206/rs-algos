use std::collections::VecDeque;

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
*/
pub struct MaxHeap<T: Ord> {
  pub elems: Vec<T>, // 保存完全二叉树
}

impl<T: Ord> MaxHeap<T> {
  pub fn new() -> MaxHeap<T> {
    MaxHeap { elems: Vec::new() }
  }

  // 计算父节点下标
  pub fn parent(i: usize) -> usize {
    if i > 0 {
      (i - 1) >> 1
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
  fn sift_down(&mut self, i: usize) {
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
      self.sift_down(largest);
    }
  }

  /// 对节点i进行上浮操作
  fn sift_up(&mut self, i: usize) {
    if i == 0 {
      return;
    }
    let parent = MinHeap::<T>::parent(i);
    // 父节点小于当前节点，交换
    if self.elems[parent] < self.elems[i] {
      self.elems.swap(i, parent);
      self.sift_up(parent);
    }
  }

  // 插入一个元素
  pub fn push(&mut self, v: T) {
    self.elems.push(v);
    // 上升元素
    let last_index = self.len() - 1;
    self.sift_up(last_index);
  }

  // 弹出最大元素
  pub fn pop(&mut self) -> Option<T> {
    if self.is_empty() {
      None
    } else {
      let b = self.len() - 1;
      self.elems.swap(0, b);
      let v = self.elems.pop();
      if !self.is_empty() {
        // 下沉根节点
        self.sift_down(0);
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

impl<T: Ord> From<Vec<T>> for MaxHeap<T> {
  /**
  在将一个数组转换为最大堆时，从数组的中间位置开始逐步向前进行下移操作

  从数组的中间位置开始是因为，
  - 最后一个非叶子节点在完全二叉树中的索引位置大致位于数组长度的一半减一处。
  - 最后一个非叶子节点的下层节点都是叶子节点，所以对这些节点进行下移操作不会影响堆的性质。

  因此，我们可以从最后一个非叶子节点开始，往根节点的方向逐步进行下移操作
   */
  fn from(value: Vec<T>) -> Self {
    let mut heap = MaxHeap { elems: value };
    for i in (0..heap.len() / 2).rev() {
      // 下沉节点i
      heap.sift_down(i)
    }
    heap
  }
}

pub struct MinHeap<T: Ord> {
  pub elems: VecDeque<T>,
}

impl<T: Ord> MinHeap<T> {
  pub fn new() -> MinHeap<T> {
    MinHeap {
      elems: VecDeque::new(),
    }
  }

  // 计算父节点下标
  fn parent(i: usize) -> usize {
    if i > 0 {
      (i - 1) >> 1
    } else {
      0
    }
  }

  // 计算左子节点下标
  fn left(i: usize) -> usize {
    i * 2 + 1
  }

  // 计算右子节点下标
  fn right(i: usize) -> usize {
    i * 2 + 2
  }

  /// 对节点i进行下沉操作
  fn sift_down(&mut self, i: usize) {
    let (left, right, mut smallest) = (MinHeap::<T>::left(i), MinHeap::<T>::right(i), i);
    // 左子节点小于当前节点，交换
    if left < self.len() && self.elems[left] < self.elems[smallest] {
      smallest = left;
    }
    // 右子节点小于当前节点，交换
    if right < self.len() && self.elems[right] < self.elems[smallest] {
      smallest = right;
    }
    // 若发生交换，继续下沉节点，保证满足小顶堆的定义
    if smallest != i {
      self.elems.swap(smallest, i);
      self.sift_down(smallest);
    }
  }
  /// 对节点i进行上浮操作
  fn sift_up(&mut self, i: usize) {
    if i == 0 {
      return;
    }
    let parent = MinHeap::<T>::parent(i);
    // 父节点大于当前节点，交换
    if self.elems[parent] > self.elems[i] {
      self.elems.swap(i, parent);
      self.sift_up(parent);
    }
  }

  /// 插入一个元素
  pub fn push(&mut self, v: T) {
    self.elems.push_back(v);
    // 上升元素
    let last_index = self.len() - 1;
    self.sift_up(last_index);
  }

  /// 弹出最小元素
  pub fn pop(&mut self) -> Option<T> {
    if self.is_empty() {
      None
    } else {
      let b = self.elems.len() - 1;
      self.elems.swap(0, b);
      let v = self.elems.pop_back();
      if !self.is_empty() {
        // 下沉根节点
        self.sift_down(0);
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

impl<T: Ord> From<VecDeque<T>> for MinHeap<T> {
  /**
  在将一个数组转换为最小堆时，从数组的中间位置开始逐步向前进行下移操作

  从数组的中间位置开始是因为，
  - 最后一个非叶子节点在完全二叉树中的索引位置大致位于数组长度的一半减一处。
  - 最后一个非叶子节点的下层节点都是叶子节点，所以对这些节点进行下移操作不会影响堆的性质。

  因此，我们可以从最后一个非叶子节点开始，往根节点的方向逐步进行下移操作
   */
  fn from(value: VecDeque<T>) -> Self {
    let mut heap = MinHeap { elems: value };
    // 从非叶子节点开始逐个进行下沉操作
    let last_parent_idx = (heap.elems.len() - 2) >> 1;
    // 自顶向下遍历非叶节点
    for i in (0..=last_parent_idx).rev() {
      heap.sift_down(i);
    }
    heap
  }
}
impl<T: Ord> From<Vec<T>> for MinHeap<T> {
  fn from(value: Vec<T>) -> Self {
    let mut heap = MinHeap {
      elems: VecDeque::from(value),
    };
    // 从非叶子节点开始逐个进行下沉操作
    let last_parent_idx = (heap.elems.len() - 2) >> 1;
    // 自顶向下遍历非叶节点
    for i in (0..=last_parent_idx).rev() {
      heap.sift_down(i);
    }
    heap
  }
}

/**
  堆排序的基本思想是：将待排序序列构造成一个小顶堆，此时，整个序列的最小值就是堆顶根节点。<br>
  弹出根节点，再将剩余的 n - 1 个元素重新构造成一个堆，这样会得到一个新的最小值。<br>
*/
pub fn max_heap_sort(nums: &mut Vec<usize>) {
  let mut heap = MaxHeap::from(nums.clone());
  for i in (0..nums.len()).rev() {
    nums[i] = heap.pop().unwrap();
  }
}
pub fn min_heap_sort(nums: &mut Vec<usize>) {
  let mut heap = MinHeap::from(nums.clone());
  for i in 0..nums.len() {
    nums[i] = heap.pop().unwrap();
  }
}

/**
#### 桶排序基本思路
  1. 将待排序元素划分到不同的桶，先遍历求出 maxV 和 minV，设桶个数为k，<br>
    则把区间\[minV, maxV\]均匀划分成k个区间，每个区间就是一个桶，将序列中的元素分配到各自的桶。
  2. 对每个桶内的元素进行排序，排序算法可用任意排序算法。
  3. 将各个桶中的有序元素合并成一个大的有序集合。

假设数据是均匀分布的，则每个桶的元素平均个数为 n / k。<br>
假设选择用快速排序对每个桶内的元素进行排序，那么每次排序的时间复杂度为 O((n / k)log(n / k))。<br>
总的时间复杂度为O(n)+O(k)O(n/klog(n/k)) = O(n+nlog(n/k)) = O(n+nlogn-nlogk)。<br>
当 k 接近于 n 时，桶排序的时间复杂度就可以认为是 O(n)。<br>
即桶越多，时间效率就越高，而桶越多，空间就越大，越费内存，可见这是用空间换时间<br>

桶排序的一个缺点是桶的数量太多。比如待排序数组 \[1,100,20,9,4,8,50\]，<br>
可能桶排序算法会创建 100 个桶，然而大部分桶用不上，造成了空间浪费。
 */
pub fn bucket_sort(arr: &mut [i32]) {
  let len = arr.len();
  if len <= 1 {
    return;
  }

  // 寻找区间长度
  let mut min = 0;
  let mut max = 0;
  for &num in arr.iter() {
    if num > max {
      max = num;
    }
    if num < min {
      min = num;
    }
  }

  // 创建桶   如何创建合理数量的桶？
  let num_buckets = (max - min) / (len as i32) + 1;
  let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); num_buckets as usize];

  // 将元素放入对应的桶中
  for &num in arr.iter() {
    let bucket_index = ((num - min) / (len as i32)) as usize;
    buckets[bucket_index].push(num);
  }
  // println!("{:?}", buckets.clone());

  // 对每个桶内的元素进行排序
  for bucket in buckets.iter_mut() {
    quick_sort(bucket);
  }

  // 合并各个桶的元素得到排序结果
  let mut index = 0;
  for bucket in buckets.iter() {
    for &num in bucket.iter() {
      arr[index] = num;
      index += 1;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_insertion_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    insertion_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![93, 84, 72, 82, 31, 66, 56, 19, 44, 24];
    insertion_sort(&mut nums);
    assert_eq!(vec![19, 24, 31, 44, 56, 66, 72, 82, 84, 93], nums);
  }
  #[test]
  fn test_binary_insertion_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    binary_insertion_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    binary_insertion_sort(&mut nums);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], nums);
    let mut nums = vec![93, 84, 72, 82, 31, 66, 56, 19, 44, 24];
    binary_insertion_sort(&mut nums);
    assert_eq!(vec![19, 24, 31, 44, 56, 66, 72, 82, 84, 93], nums);
  }
  #[test]
  fn test_quick_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    quick_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    quick_sort(&mut nums);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], nums);
    let mut nums = vec![93, 84, 72, 82, 31, 66, 56, 19, 44, 24];
    quick_sort(&mut nums);
    assert_eq!(vec![19, 24, 31, 44, 56, 66, 72, 82, 84, 93], nums);
  }
  #[test]
  fn test_shell_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    shell_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    shell_sort(&mut nums);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], nums);
    let mut nums = vec![93, 84, 72, 82, 31, 66, 56, 19, 44, 24];
    shell_sort(&mut nums);
    assert_eq!(vec![19, 24, 31, 44, 56, 66, 72, 82, 84, 93], nums);
  }
  #[test]
  fn test_merge_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    merge_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    merge_sort(&mut nums);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], nums);
    let mut nums = vec![93, 84, 72, 82, 31, 66, 56, 19, 44, 24];
    merge_sort(&mut nums);
    assert_eq!(vec![19, 24, 31, 44, 56, 66, 72, 82, 84, 93], nums);
  }
  #[test]
  fn test_max_heap_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    max_heap_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    max_heap_sort(&mut nums);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], nums);
    let mut nums = vec![93, 84, 72, 82, 31, 66, 56, 19, 44, 24];
    max_heap_sort(&mut nums);
    assert_eq!(vec![19, 24, 31, 44, 56, 66, 72, 82, 84, 93], nums);
  }
  #[test]
  fn test_min_heap_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    min_heap_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    min_heap_sort(&mut nums);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], nums);
    let mut nums = vec![93, 84, 72, 82, 31, 66, 56, 19, 44, 24];
    min_heap_sort(&mut nums);
    assert_eq!(vec![19, 24, 31, 44, 56, 66, 72, 82, 84, 93], nums);
  }
  #[test]
  fn test_bucket_sort() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    bucket_sort(&mut nums);
    assert_eq!(vec![1, 2, 5, 5, 6, 9], nums);
    let mut nums = vec![1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    bucket_sort(&mut nums);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], nums);
    let mut nums = vec![93, 84, 72, 82, 31, 66, 56, 19, 44, 24];
    bucket_sort(&mut nums);
    assert_eq!(vec![19, 24, 31, 44, 56, 66, 72, 82, 84, 93], nums);
  }
  #[test]
  fn test_min_heap() {
    let vec_nums = vec![93, 19, 72, 31, 84, 66, 56, 44, 82, 24];
    let nums = VecDeque::from(vec_nums.clone());
    let mut min_heap = MinHeap::new();
    for num in nums.clone() {
      min_heap.push(num)
    }
    assert_eq!(
      VecDeque::from(vec![19, 24, 56, 44, 31, 72, 66, 93, 82, 84]),
      min_heap.elems
    );
    let reversed: Vec<i32> = vec_nums.clone().into_iter().rev().collect();
    assert_eq!(
      MinHeap::from(nums).elems,
      VecDeque::from(vec![19, 24, 56, 31, 84, 66, 72, 44, 82, 93])
    );
    assert_eq!(
      MinHeap::from(reversed).elems,
      VecDeque::from(vec![19, 24, 31, 56, 66, 84, 44, 72, 82, 93])
    );
  }
  #[test]
  fn test_max_heap() {
    let vec_nums = vec![93, 19, 72, 31, 84, 66, 56, 44, 82, 24];
    let mut max_heap = MaxHeap::new();
    for num in vec_nums.clone() {
      max_heap.push(num)
    }
    assert_eq!(vec![93, 84, 72, 82, 31, 66, 56, 19, 44, 24], max_heap.elems);
    let reversed: Vec<i32> = vec_nums.clone().into_iter().rev().collect();
    assert_eq!(
      MaxHeap::from(vec_nums).elems,
      vec![93, 84, 72, 82, 24, 66, 56, 44, 31, 19]
    );
    assert_eq!(
      MaxHeap::from(reversed).elems,
      vec![93, 82, 84, 72, 66, 44, 31, 56, 19, 24]
    );
  }
}
