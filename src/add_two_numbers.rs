#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

// pub fn solution(
//   l1: Option<Box<ListNode>>,
//   l2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//   let (mut l1, mut l2) = (l1, l2);
//   let mut result = Some(Box::new(ListNode::new(0)));
//   let mut current = &mut result;
//   let (mut carry, mut n1, mut n2) = (0, 0, 0);
//   while l1.is_some() || l2.is_some() {
//     if let Some(num1) = &l1 {
//       n1 = num1.val;
//       l1 = l1.unwrap().next;
//     } else {
//       n1 = 0;
//     }
//     if let Some(num2) = &l2 {
//       n2 = num2.val;
//       l2 = l2.unwrap().next;
//     } else {
//       n2 = 0;
//     }
//     let sum = n1 + n2 + carry;
//     current.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
//     current = &mut current.as_mut().unwrap().next;
//     carry = sum / 10;
//     if carry > 0 {
//       current.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
//     }
//   }
//   result.unwrap().next
// }

pub fn solution(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let (mut l1, mut l2) = (l1, l2);
  let mut l3 = Some(Box::new(ListNode::new(0)));
  let mut tail = &mut l3;
  loop {
    let a = l1.take().unwrap_or(Box::new(ListNode::new(0)));
    let b = l2.take().unwrap_or(Box::new(ListNode::new(0)));
    tail = match tail.as_mut() {
      Some(inner_box) => {
        let sum = a.val + b.val + inner_box.val;
        let carry = sum / 10;
        inner_box.val = sum % 10;
        if a.next.is_none() && b.next.is_none() && carry == 0 {
          break l3;
        } else {
          inner_box.next = Some(Box::new(ListNode::new(carry)))
        };
        &mut inner_box.next
      }
      _ => unreachable!(),
    };
    l1 = a.next;
    l2 = b.next;
  }
}

#[cfg(test)]
mod tests {
  use crate::add_two_numbers::{self, ListNode};
  #[test]
  pub fn test_add_two_numbers() {
    let mut n1 = ListNode::new(9);
    let mut n2 = ListNode::new(9);
    let mut n3 = ListNode::new(9);
    let n7 = ListNode::new(9);
    let mut n4 = ListNode::new(9);
    let mut n5 = ListNode::new(9);
    let n6 = ListNode::new(9);
    n3.next = Some(Box::new(n7));
    n2.next = Some(Box::new(n3));
    n1.next = Some(Box::new(n2));
    n5.next = Some(Box::new(n6));
    n4.next = Some(Box::new(n5));
    println!(
      "{:?}",
      add_two_numbers::solution(Some(Box::new(n1)), Some(Box::new(n4)))
    );
  }
}
