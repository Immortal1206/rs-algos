use std::collections::HashMap;

pub fn refund_min_cashes_count(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
  // 动态收集从 1 到 amount 的最小找零纸币数
  for denm in 1..=amount {
    // 此 min_cash_num 等于全用 1 元纸币找零的纸币数
    let mut min_cashes_count = denm;
    for c in cashes.iter().filter(|&&c| c <= denm) {
      let index = (denm - c) as usize;

      // 加 1 是因为当前最小找零数等于上一最小找零数加 1 张 c 面额纸币
      let cashe_num = min_cashes[index] + 1;
      if cashe_num < min_cashes_count {
        min_cashes_count = cashe_num;
      }
    }

    min_cashes[denm as usize] = min_cashes_count;
  }

  // 因为收集了所有的最小找零纸币数，所以直接返回
  min_cashes[amount as usize]
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct MinCash {
  pub count: u32,
  pub cashes: HashMap<u32, u32>,
}

pub fn refund_min_cashes(cashes: &[u32], amount: u32, min_cashes: &mut [MinCash]) -> MinCash {
  for denm in 1..=amount {
    let mut min_cash_num = denm;
    let mut min_cash_map: HashMap<u32, u32> = HashMap::new();
    for c in cashes.iter().filter(|&&c| c <= denm) {
      let index = (denm - c) as usize;

      let cashe_num = min_cashes[index].count + 1;
      if cashe_num < min_cash_num {
        min_cash_map.clone_from(&min_cashes[index].cashes);
        let count = match min_cash_map.get(c) {
          None => 0,
          Some(&val) => val,
        };
        min_cash_map.insert(*c, count + 1);
        min_cash_num = cashe_num;
      }
    }

    min_cashes[denm as usize] = MinCash {
      count: min_cash_num,
      cashes: min_cash_map,
    };
  }

  min_cashes[amount as usize].clone()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_refund_min_cashes_count() {
    let amount = 81u32;
    let cashes = [1, 5, 10, 20, 50];
    let mut min_cashes: [u32; 82] = [0; 82];
    let cashe_num = refund_min_cashes_count(&cashes, amount, &mut min_cashes);
    assert_eq!(4, cashe_num);
  }
  #[test]
  fn test_refund_min_cashes() {
    let amount = 81u32;
    let cashes = [1, 5, 10, 20, 50];
    let mut min_cashes = Vec::with_capacity(82);
    for _ in 0..82 {
      min_cashes.push(MinCash::default());
    }
    let cashe_num = refund_min_cashes(&cashes, amount, &mut min_cashes);
    let mut min_caches_info = HashMap::new();
    min_caches_info.insert(1, 1);
    min_caches_info.insert(10, 1);
    min_caches_info.insert(20, 1);
    min_caches_info.insert(50, 1);
    // println!("{:#?}", cashe_num);
    assert_eq!(
      MinCash {
        count: 4,
        cashes: min_caches_info
      },
      cashe_num
    );
    let amount = 63u32;
    let cashes = [1, 5, 10, 20, 21, 50];
    let mut min_cashes = Vec::with_capacity(64);
    for _ in 0..64 {
      min_cashes.push(MinCash::default());
    }
    let cashe_num = refund_min_cashes(&cashes, amount, &mut min_cashes);
    let mut min_caches_info = HashMap::new();
    min_caches_info.insert(21, 3);
    // println!("{:#?}", cashe_num);
    assert_eq!(
      MinCash {
        count: 3,
        cashes: min_caches_info
      },
      cashe_num
    );
  }
}
