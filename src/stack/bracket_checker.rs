use super::Stack;
pub fn round_bracket(str: &str) -> bool {
  let mut stack = Stack::new();
  let mut balance = true;
  if str.len() % 2 != 0 {
    return false;
  }
  for c in str.chars() {
    if '(' == c {
      stack.push(c);
    } else {
      if stack.is_empty() {
        balance = false;
      } else {
        stack.pop();
      }
    }
  }
  balance && stack.is_empty()
}

fn is_match(open: char, close: char) -> bool {
  let opens = "([{<";
  let closers = ")]}>";
  opens.find(open) == closers.find(close)
}

pub fn all_bracket(str: &str) -> bool {
  let mut stack = Stack::new();
  let mut balance = true;
  if str.len() % 2 != 0 {
    return false;
  }
  for c in str.chars() {
    if c == '(' || c == '{' || c == '<' || c == '[' {
      stack.push(c);
    } else {
      if stack.is_empty() {
        balance = false;
      } else {
        let top = stack.pop().unwrap();
        balance = is_match(top, c);
        if !balance {
          break;
        }
      }
    }
  }
  balance && stack.is_empty()
}
pub fn all_bracket_with_character(str: &str) -> bool {
  let mut stack = Stack::new();
  let mut balance = true;
  if str.len() % 2 != 0 {
    return false;
  }
  for c in str.chars() {
    if c == '(' || c == '{' || c == '<' || c == '[' {
      stack.push(c);
    } else if c == ')' || c == '}' || c == ']' || c == '>' {
      if stack.is_empty() {
        balance = false;
      } else {
        let top = stack.pop().unwrap();
        balance = is_match(top, c);
        if !balance {
          break;
        }
      }
    }
  }
  balance && stack.is_empty()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn test_round_bracket() {
    let sa = "()(())";
    let sb = "()(()()";
    let res1 = round_bracket(sa);
    let res2 = round_bracket(sb);
    assert_eq!(true, res1);
    assert_eq!(false, res2);
  }
  #[test]
  pub fn test_all_bracket() {
    let sa = "(){}[]<>";
    let sb = "(()<>{)[})";
    let res1 = all_bracket(sa);
    let res2 = all_bracket(sb);
    assert_eq!(true, res1);
    assert_eq!(false, res2);
  }
  #[test]
  pub fn test_all_bracket_with_character() {
    let sa = "(2+3){func}[abc]";
    let sb = "(2+3)*(3-1";
    let res1 = all_bracket_with_character(sa);
    let res2 = all_bracket_with_character(sb);
    assert_eq!(true, res1);
    assert_eq!(false, res2);
  }
}
