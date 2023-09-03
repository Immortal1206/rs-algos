pub fn longest_common_prefix_v1(strs: Vec<String>) -> String {
  let mut common_prefix = String::new();
  if strs.is_empty() {
    return common_prefix;
  }
  let first_str = strs[0].as_str();
  for (idx, char) in first_str.char_indices() {
    for str in &strs[1..] {
      if idx > str.len() || str.chars().nth(idx).is_none() || str.chars().nth(idx).unwrap() != char
      {
        return common_prefix;
      }
    }
    common_prefix.push(char);
  }
  common_prefix
}

pub fn longest_common_prefix_v2(strs: Vec<String>) -> String {
  let mut output = strs[0].clone();
  for s in strs.iter() {
    while !s.starts_with(&output) {
      output.pop();
    }
  }
  output.to_string()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let input = vec![
      String::from("flower"),
      String::from("flow"),
      String::from("flight"),
    ];

    let common_prefix = longest_common_prefix_v1(input);
    assert_eq!("fl".to_string(), common_prefix);
    let input = vec![
      String::from("flower"),
      String::from("flow"),
      String::from("flight"),
    ];

    let common_prefix = longest_common_prefix_v2(input);
    assert_eq!("fl".to_string(), common_prefix);
  }
}
