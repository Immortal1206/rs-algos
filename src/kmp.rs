/// ## 前缀函数(部分匹配表（longest prefix suffix）/(partial match table))与 KMP 算法
/// ### 前缀函数
/**
  #### 字符串的前缀和后缀
  * 如果字符串A和B，存在A = BS，其中S是任意的非空字符串，那就称B为A的前缀
  * 同样可以定义后缀A = SB， 其中S是任意的非空字符串，那就称B为A的后缀

  要注意的是，字符串本身并不是自己的前后缀
*/

/**
  给定一个长度为 n 的字符串 s，其前缀函数被定义为一个长度为 n 的数组 lps.其中 lps\[i\] 的定义是：
  1. 如果子串 s\[0..i\] 有一对相等的前缀与后缀：s\[0..k-1\] 和 s\[i - (k - 1)..i]，<br>
  那么 lps\[i\] 就是这个相等的前缀（或者后缀，因为它们相等）的长度，也就是 lps\[i\]=k；
  2. 如果不止有一对相等的，那么 lps\[i\] 就是其中最长的那一对的长度；
  3. 如果没有相等的，那么 lps\[i\]=0。

  简单来说, lps\[i\]就是子串 s[0..i] 最长的相等的前缀与后缀的长度
  特别地，lps\[0\]=0。
*/

/**
#### 计算前缀函数的朴素算法
* 在一个循环中以 i = 1 -> n - 1 的顺序计算前缀函数 lps\[i\] 的值（lps\[0\] 被赋值为 0）。
* 为了计算当前的前缀函数值 lps\[i\]，我们令变量 j 从最大的前缀长度 i 开始尝试(注意，i 为索引)。
* 如果当前长度下前缀和后缀相等，则此时长度为 lps\[i\]，否则令 j 自减 1，继续匹配，直到 j=0。
* 如果 j = 0 并且仍没有任何一次匹配，则置 lps\[i\] = 0 并移至下一个下标 i + 1。
*/
#[allow(unused)]
fn make_lps_v1(s: &str) -> Vec<usize> {
  let mut lps = vec![0; s.len()];
  for i in 1..s.len() {
    for j in (0..=i).rev() {
      let prefix = &s[0..j];
      let suffix = &s[i - j + 1..=i];
      if prefix == suffix {
        lps[i] = j;
        break;
      }
    }
  }
  lps
}

/// 计算前缀函数的优化一： **相邻的前缀函数值至多增加 1**
///
/// 当移动到下一个位置时，前缀函数的值要么增加一，要么维持不变，要么减少。
///
/// 最大前缀长度为 lps\[i - 1\] + 1，即 lps\[i\] <= lps\[i - 1\] + 1
#[allow(unused)]
fn make_lps_v2(s: &str) -> Vec<usize> {
  let mut lps = vec![0; s.len()];
  for i in 1..s.len() {
    for j in (0..=lps[i - 1] + 1).rev() {
      let prefix = &s[0..j];
      let suffix = &s[i - j + 1..=i];
      if prefix == suffix {
        lps[i] = j;
        break;
      }
    }
  }
  lps
}

/// 计算前缀函数的优化二
/**
  对于如下字符串s:
  ```rs
  s[0], s[1], s[2], s[3], .. , s[i - 2], s[i - 1], s[i], s[i + 1]
  ```
  * 若 lps\[i\] = 1，那么如果lps\[i + 1\]能够增加，即lps\[i + 1\] = 2，则 s\[1\] == s\[i + 1\]
  * 若 lps\[i\] = 2，那么如果lps\[i + 1\]能够增加，即lps\[i + 1\] = 3，则 s\[2\] == s\[i + 1\]
  * 若 lps\[i\] = 3，那么如果lps\[i + 1\]能够增加，即lps\[i + 1\] = 4，则 s\[3\] == s\[i + 1\]

  也就是
  ```rs
  if s[lps[i]] == s[i + 1] {
    lps[i + 1] = lps[i] + 1
  }
  ```
  那如果 s\[lps\[i\]\] != s\[i + 1\] 怎么办？
  * 一个简单的想法是从 0 开始重新计算 lps\[i\]。但这显然是低效的！

  能不能利用前缀函数的性质呢？
  * 如果能够找到s\[0..i\]的一个仅次于 lps\[i\] 的长度 j1，使得 s\[0..j1 - 1\] == s\[i - j1 + 1..i\]，<br>
  那么仅需要再次比较 s\[i + 1\] 和 s\[j1\]，如果相等，那么 lps\[i\ + 1\] = j1 + 1
  * 否则，需要找到子串 s\[0..i\] 仅次于 j 的第二长度 j2，如此反复。直到 j == 0，如果  s\[i + 1\] != s\[0\]，lps\[i\ + 1\] = 0
  ```rs
  let s = "abcabcxxabcabca";
  s[0], s[1], s[2], s[3], s[4], s[5], s[6], .. , s[i - 6], s[i - 5], s[i - 4], s[i - 3], s[i - 2], s[i - 1], s[i], s[i + 1]
  lps = [0, 0, 0, 1, 2, 3, 0, 0, 1, 2, 3, 4, 5, 6];
  ```
  观察上面的字符串，i == 13; j1 == 3，因为 s\[0..lps\[i\] - 1\] == s\[i - lps\[i\] + 1..i\]，<br>
  所以对于 s\[0..i\] 的第二长度 j1，有 s\[0..j1 - 1\] == s\[i - j1 + 1..i\] == ***s\[lps\[i\] - j1 ..lps\[i\] - 1\]***

  而 s\[lps\[i\] - j1..lps\[i\] - 1\] 恰好是字串 s\[0..lps\[i\] - 1\]的后缀

  那么 j1 的最大值就是 lps\[lps\[i\] - 1\]

  同理，次于 j1 的第二长度 j2 等价于 s\[j1 - 1\] 的前缀函数值 lps\[j1 - 1\]

  显然我们可以得到一个关于 j 的状态转移方程：

  **jn = lps\[j(n-1) -1\], j(n - 1) > 0.**
*/
fn make_lps(s: &str) -> Vec<usize> {
  let s = s.as_bytes();
  let mut lps = vec![0; s.len()];
  for i in 1..s.len() {
    let mut j = lps[i - 1];
    while j > 0 && s[j] != s[i] {
      j = lps[j - 1];
    }
    if s[j] == s[i] {
      j += 1;
    }
    lps[i] = j;
  }
  lps
}

/// #### 前缀函数的应用： KMP算法
/**
  给定一个文本 t 和一个字符串 s，我们尝试找到并展示 s 在 t 中的所有出现（occurrence）

  为了简便起见，用 n 表示字符串 s 的长度，用 m 表示文本 t 的长度。

  1. 构造一个字符串 st = s + # + t，其中 # 为一个既不出现在 s 中也不出现在 t 中的分隔符
  2. 计算该字符串的前缀函数

  考虑该前缀函数除去最开始 n + 1 个值（即属于字符串 s 和分隔符的函数值）后其余函数值的意义

  根据定义，lps\[i\] 为右端点在 i 且同时为一个前缀的最长子串的长度，具体到这种情况下，其值为与 s 的前缀相同且右端点位于 i 的最长子串的长度。

  由于分隔符的存在，该长度不可能超过 n。

  而如果等式 lps\[i\] = n 成立，则意味着 s 完整出现在该位置（即其右端点位于位置 i）。注意该位置的下标是对字符串 s + # + t 而言的。

  因此如果在某一位置 i 有 lps\[i\] = n 成立，则字符串 s 在字符串 st 的 i - (n - 1) 处出现，

  字符串 s 在 t 的 i - (n - 1) - (n + 1) = i -2n 处出现
  ```rs
  let t = "4sadbutsad";
  let s = "sad";
  let st = "sad#4sadbutsad";
  let lps = [0, 0, 0, 0, 0, 1, 2, 3, 0, 0, 0, 1, 2, 3]
  ```
*/
pub fn kmp(t: String, s: String) -> Vec<usize> {
  // let st: Vec<u8> = s.bytes().chain("#".bytes()).chain(t.bytes()).collect();
  let st = s.clone() + "#" + &t;
  let lps = make_lps(&st);
  let mut res = vec![];

  for i in s.len() + 1..st.len() {
    if lps[i] == s.len() {
      res.push(i - s.len() * 2);
    }
  }
  res
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_kmp() {
    let t = "ababababca";
    let s = "abababca";
    assert_eq!(vec![2], kmp(t.into(), s.into()));
    let t = "leetcode";
    let s = "leeto";
    assert_eq!(Vec::<usize>::new(), kmp(t.into(), s.into()));
    let t = "4sadbutsad";
    let s = "sad";
    assert_eq!(vec![1, 7], kmp(t.into(), s.into()));
  }
  #[test]
  fn test_make_lps() {
    let needle = "abababca";
    assert_eq!(vec![0, 0, 1, 2, 3, 4, 0, 1], make_lps_v1(needle));
    let needle = "ababcababa";
    assert_eq!(vec![0, 0, 1, 2, 0, 1, 2, 3, 4, 3], make_lps(needle));
    let needle = "abababca";
    assert_eq!(vec![0, 0, 1, 2, 3, 4, 0, 1], make_lps(needle));
    let needle = "abababca";
    assert_eq!(vec![0, 0, 1, 2, 3, 4, 0, 1], make_lps_v2(needle));
    let needle = "ababcababa";
    assert_eq!(vec![0, 0, 1, 2, 0, 1, 2, 3, 4, 3], make_lps_v2(needle));
    let needle = "sad#4sadbutsad";
    assert_eq!(vec![0, 0, 1, 2, 0, 1, 2, 3, 4, 3], make_lps(needle));
  }
}
