pub fn binary_search_iteration(nums: &[i32], num: i32) -> bool {
  let mut low = 0;
  let mut high = nums.len() - 1;
  let mut found = false;

  // 注意是 <= 不是 <
  while low <= high && !found {
    let mid: usize = (low + high) >> 1;

    // 若 low + high 可能溢出，可转换为减法
    // let mid: usize = low  + ((high - low) >> 1);

    if num == nums[mid] {
      found = true;
    } else if num < nums[mid] {
      high = mid - 1; // num < 中间值，省去后半部数据
    } else {
      low = mid + 1; // num >= 中间值，省去前半部数据
    }
  }

  found
}

pub fn binary_search_recursion(nums: &[i32], num: i32) -> bool {
  // 基本情况1: 项不存在
  if 0 == nums.len() {
    return false;
  }

  let mid: usize = nums.len() >> 1;
  if num == nums[mid] {
    // 基本情况2: 项存在
    return true;
  } else if num < nums[mid] {
    return binary_search_recursion(&nums[..mid], num);
  } else {
    return binary_search_recursion(&nums[mid + 1..], num);
  }
}
/**
  给定直线两点 **(x0, y0)** 和 **(x1, y1)** ，可以求出 **(x0, x1)** 范围内任意点 x 对应的值 y 或者任意 y 对应的 x.
  ```rs
  (y − y0) / (x − x0) = (y1 − y0) / (x1 − x0)

  x = (y − y0)(x1 − x0) / (y1 − y0) + x0
  ```
  比如要在 \[1,9,10,15,16,17,19,23,27,28,29,30,32,35\]<br>
  这个已排序的 14 个元素的集合中查找到元素 27，那么可以将索引当做 x 轴，元素值当做 y 轴。<br>
  可知 x0 = 0, x1 = 13，而 y0 = 1, y1 = 35 所以可以计算 y = 27 对应的 x 值 x = 9 <br>
  查看 nums\[9] 发现值为 28，大于 27，所以将 28 当做上界。28 的下标为 9，所以搜索\[0,8\] 范围内的元素，继续执行插值算法
*/
pub fn interpolation_search(nums: &[i32], target: i32) -> bool {
  if nums.is_empty() {
    return false;
  }

  // 查询范围
  let mut low = 0usize;
  let mut high = nums.len() - 1;
  loop {
    let low_val = nums[low];
    let high_val = nums[high];
    if high <= low || target < low_val || target > high_val {
      break;
    }

    // 计算插值位置
    let offset = (target - low_val) * (high - low) as i32 / (high_val - low_val);
    let interpolant = low + offset as usize;

    // 更新上下界 high、low
    if nums[interpolant] > target {
      high = interpolant - 1;
    } else if nums[interpolant] < target {
      low = interpolant + 1;
    } else {
      break;
    }
  }

  // 判断最终确定的上界处是否是 target
  target == nums[high]
}

/**
  假设要在 \[2,3,4,6,7,8,10,13,15,19,20,22,23,24,28\] 这个 15 个元素已排序集合中查找 22，<br>
  那么首先查看 2 ^ 0 = 1 位置上的数字是否超过 22，得到 3 < 22，所以继续查找 2 ^ 1, 2 ^ 2, 2 ^ 3 位置处元素，<br>
  发现对应的值 4, 7, 15 均小于 22。继续查看 16 = 2 ^ 4 处的值，可是 16 大于集合元素个数，超出范围了，<br>
  所以查找上界就是最后一个索引 14 <br>
  能找到一个上界，那么说明前一次访问处一定小于待查找的值，作为下界是合理的
 */
pub fn exponential_search(nums: &[i32], target: i32) -> bool {
  let size = nums.len();
  if size == 0 { return false; }

  // 逐步找到上界
  let mut high = 1usize;
  while high < size && nums[high] < target {
      high <<= 1;
  }

  //  上界的一半一定可以作为下界
  let low = high >> 1;

  // 区间内二分搜索加速查找
  binary_search_iteration(&nums[low..size.min(high+1)], target)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_binary_search() {
    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];

    let mut target = 3;
    let found = binary_search_iteration(&nums, target);
    assert_eq!(found, true);

    target = 63;
    let found = binary_search_iteration(&nums, target);
    assert_eq!(found, false);

    target = 3;
    let found = binary_search_recursion(&nums, target);
    assert_eq!(found, true);

    target = 63;
    let found = binary_search_recursion(&nums, target);
    assert_eq!(found, false);
  }
  #[test]
  fn test_interpolation_search() {
    let nums = [1, 9, 10, 15, 16, 17, 19, 23, 27, 28, 29, 30, 32, 35];
    let target = 27;
    let found = interpolation_search(&nums, target);
    assert_eq!(found, true);

    let nums = [0, 1, 2, 10, 16, 19, 31, 35, 36, 38, 40, 42, 43, 55];
    let found = interpolation_search(&nums, target);
    assert_eq!(found, false);
  }
}
