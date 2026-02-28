//Definition for singly-linked list.
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
#[allow(dead_code)]
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut p1 = l1;
    let mut p2 = l2;

    let mut carry = 0;

    let mut dummy = ListNode::new(0);
    let mut tail: &mut ListNode = &mut dummy;

    while p1.is_some() || p2.is_some() || carry != 0 {
        let v1 = p1.as_ref().map(|n| n.val).unwrap_or(0);
        let v2 = p2.as_ref().map(|n| n.val).unwrap_or(0);

        let sum = v1 + v2 + carry;
        let digit = sum % 10;
        carry = sum / 10;

        tail.next = Some(Box::new(ListNode::new(digit)));
        tail = tail.next.as_deref_mut().unwrap();

        p1 = p1.and_then(|n| n.next);
        p2 = p2.and_then(|n| n.next);
    }

    dummy.next
}
