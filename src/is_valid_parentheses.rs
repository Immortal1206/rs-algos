pub fn is_valid_parentheses(s: String) -> bool {
  if s.len() % 2 != 0 {
    return false;
  }
  let mut v = Vec::with_capacity(s.len());
  for c in s.chars() {
    match c {
      '(' | '[' | '{' => v.push(c),
      _ => match v.pop() {
        Some('(') if c == ')' => (),
        Some('[') if c == ']' => (),
        Some('{') if c == '}' => (),
        _ => return false,
      },
    }
  }
  v.is_empty()
}

pub fn is_valid_parentheses_v2(s: &str) -> bool {
  s.chars()
    .fold(Vec::new(), |mut stack, c| {
      match c {
        '(' | '{' | '[' => stack.push(c), // Push opening bracket onto the stack
        ')' => {
          if stack.last() == Some(&'(') {
            stack.pop(); // Matching bracket, pop from stack
          } else {
            stack.push(')'); // Unmatched bracket
          }
        }
        '}' => {
          if stack.last() == Some(&'{') {
            stack.pop();
          } else {
            stack.push('}');
          }
        }
        ']' => {
          if stack.last() == Some(&'[') {
            stack.pop();
          } else {
            stack.push(']');
          }
        }
        _ => (), // Ignore invalid characters
      }
      stack
    })
    .is_empty()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let s = "()[]{}";
    assert_eq!(true, is_valid_parentheses(s.into()));
    let s = "(){}[)(}{[]}";
    assert_eq!(false, is_valid_parentheses(s.into()));
    let s = "()[]{}";
    assert_eq!(true, is_valid_parentheses_v2(s.into()));
    let s = "(){}[)(}{[]}";
    assert_eq!(false, is_valid_parentheses_v2(s.into()));
  }
}
