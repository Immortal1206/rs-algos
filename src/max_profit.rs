pub fn max_profit_v1(prices: Vec<i32>) -> i32 {
  let (mut profit, mut buy) = (0, prices[0]);

  for i in 1..prices.len() {
    profit = i32::max(profit, prices[i] - buy);
    buy = i32::min(buy, prices[i]);
  }

  profit
}
pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
  prices
    .iter()
    .fold((0, i32::MAX), |(mut max_profit, mut cost), &price| {
      cost = i32::min(price, cost);
      max_profit = i32::max(max_profit, price - cost);
      (max_profit, cost)
    })
    .0
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let s = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(5, max_profit_v1(s));
    let s = vec![7, 6, 4, 3, 1];
    assert_eq!(0, max_profit_v1(s));
    let s = vec![7, 2, 5, 3, 8, 1, 5, 6, 4];
    assert_eq!(6, max_profit_v1(s));
    let s = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(5, max_profit_v2(s));
    let s = vec![7, 6, 4, 3, 1];
    assert_eq!(0, max_profit_v2(s));
    let s = vec![7, 2, 5, 3, 8, 1, 5, 6, 4];
    assert_eq!(6, max_profit_v2(s));
  }
}
