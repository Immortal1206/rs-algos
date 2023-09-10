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

pub fn merge_two_lists_v1(
  list1: Option<Box<ListNode>>,
  list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  match (list1, list2) {
    (Some(l), None) => Some(l),
    (None, Some(l)) => Some(l),
    (None, None) => None,
    (Some(l1), Some(l2)) => {
      if l1.val <= l2.val {
        Some(Box::new(ListNode {
          val: l1.val,
          next: merge_two_lists_v1(l1.next, Some(l2)),
        }))
      } else {
        Some(Box::new(ListNode {
          val: l2.val,
          next: merge_two_lists_v1(Some(l1), l2.next),
        }))
      }
    }
  }
}
pub fn merge_two_lists_v2(
  list1: Option<Box<ListNode>>,
  list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  let mut pre_head = ListNode::new(-1);
  let mut cur_head = &mut pre_head;
  let mut list1 = list1;
  let mut list2 = list2;
  loop {
    match (&mut list1, &mut list2) {
      (Some(l), None) => {
        cur_head.next = Some(Box::new(ListNode::new(l.val)));
        list1 = l.next.take();
      }
      (None, Some(l)) => {
        cur_head.next = Some(Box::new(ListNode::new(l.val)));
        list2 = l.next.take();
      }
      (None, None) => break pre_head.next,
      (Some(l1), Some(l2)) => {
        if l1.val <= l2.val {
          cur_head.next = Some(Box::new(ListNode::new(l1.val)));
          list1 = l1.next.take();
        } else {
          cur_head.next = Some(Box::new(ListNode::new(l2.val)));
          list2 = l2.next.take();
        }
      }
    }
    cur_head = cur_head.next.as_mut().unwrap();
  }
}

pub fn merge_two_lists_v3(
  list1: Option<Box<ListNode>>,
  list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  let mut list1 = list1;
  let mut list2 = list2;
  let mut prehead = ListNode::new(0);
  let mut cur_node = &mut prehead;

  while let (Some(node1), Some(node2)) = (&list1, &list2) {
    if node1.val <= node2.val {
      cur_node.next = list1.take();
      cur_node = cur_node.next.as_mut().unwrap();
      list1 = cur_node.next.take();
    } else {
      cur_node.next = list2.take();
      cur_node = cur_node.next.as_mut().unwrap();
      list2 = cur_node.next.take();
    }
  }
  cur_node.next = list1.or(list2);
  prehead.next
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let mut n1 = ListNode::new(1);
    let mut n2 = ListNode::new(3);
    let mut n3 = ListNode::new(9);
    let n7 = ListNode::new(9);
    let mut n4 = ListNode::new(2);
    let mut n5 = ListNode::new(4);
    let n6 = ListNode::new(8);
    n3.next = Some(Box::new(n7));
    n2.next = Some(Box::new(n3));
    n1.next = Some(Box::new(n2));
    n5.next = Some(Box::new(n6));
    n4.next = Some(Box::new(n5));
    println!(
      "{:?}",
      merge_two_lists_v1(Some(Box::new(n1.clone())), Some(Box::new(n4.clone())))
    );
    println!(
      "{:?}",
      merge_two_lists_v2(Some(Box::new(n1.clone())), Some(Box::new(n4.clone())))
    );
    println!(
      "{:?}",
      merge_two_lists_v3(Some(Box::new(n1.clone())), Some(Box::new(n4.clone())))
    );
  }
}
