pub mod base_converter;
pub mod bracket_checker;
pub mod calculator;
#[derive(Debug)]
pub struct Stack<T> {
  top: usize,
  data: Vec<T>,
}

impl<T> Stack<T> {
  pub fn new() -> Self {
    Stack {
      top: 0,
      data: Vec::new(),
    }
  }

  pub fn push(&mut self, val: T) {
    self.data.push(val);
    self.top += 1;
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.top == 0 {
      return None;
    }
    self.top -= 1;
    self.data.pop()
  }

  pub fn peek(&self) -> Option<&T> {
    if self.top == 0 {
      return None;
    }
    self.data.get(self.top - 1)
  }

  pub fn is_empty(&self) -> bool {
    0 == self.top
  }

  pub fn size(&self) -> usize {
    self.top
  }
}
