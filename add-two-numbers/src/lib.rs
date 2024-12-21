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

    #[allow(unused)]
    fn new_with_next(val: i32, next: Box<ListNode>) -> Self {
        ListNode {
            val,
            next: Some(next),
        }
    }
}

/// The solution's idea is actually quite easy. The hard part is dealing with rust ownership rules.
/// Specially the mutable references. Those are the evil man. Anyways, the ideia is basically
/// traversing both linked lists with two pointer approach and adding the values while keeping track
/// of the carry. Simple as that.
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut curr = &mut head;
    let mut p1 = l1.as_deref();
    let mut p2 = l2.as_deref();

    let mut carry = 0;
    while p1.is_some() || p2.is_some() || carry != 0 {
        let sum = p1.map_or(0, |x| x.val) + p2.as_ref().map_or(0, |x| x.val) + carry;
        carry = sum / 10;

        let sum_node = curr.insert(Box::new(ListNode::new(sum % 10)));
        curr = &mut sum_node.next;
        p1 = p1.and_then(|node| node.next.as_deref());
        p2 = p2.and_then(|node| node.next.as_deref());
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_to_i32(list: Option<&ListNode>) -> i32 {
        let mut sum = 0i32;
        let mut pad = 1i32;
        let mut p = list;
        while p.is_some() {
            let node = p.unwrap();
            sum += node.val * pad;

            pad *= 10;
            p = node.next.as_deref();
        }

        sum
    }

    #[test]
    fn case1() {
        let l1 = Some(Box::from(ListNode::new_with_next(
            2,
            Box::from(ListNode::new_with_next(4, Box::from(ListNode::new(3)))),
        )));
        let l2 = Some(Box::from(ListNode::new_with_next(
            5,
            Box::from(ListNode::new_with_next(6, Box::from(ListNode::new(4)))),
        )));
        let out = add_two_numbers(l1, l2);
        assert_eq!(list_to_i32(out.as_deref()), 807);
    }

    #[test]
    fn case2() {
        let l1 = Some(Box::from(ListNode::new_with_next(
            9,
            Box::from(ListNode::new_with_next(9, Box::from(ListNode::new(9)))),
        )));
        let l2 = Some(Box::from(ListNode::new(1)));
        let out = add_two_numbers(l1, l2);
        assert_eq!(list_to_i32(out.as_deref()), 1000);
    }

    #[test]
    fn case3() {
        let l1 = Some(Box::from(ListNode::new_with_next(
            9,
            Box::from(ListNode::new_with_next(9, Box::from(ListNode::new(1)))),
        )));
        let l2 = Some(Box::from(ListNode::new(1)));
        let out = add_two_numbers(l1, l2);
        assert_eq!(list_to_i32(out.as_deref()), 200);
    }

    #[test]
    fn case4() {
        let l1 = Some(Box::from(ListNode::new_with_next(
            1,
            Box::from(ListNode::new(8)),
        )));
        let l2 = Some(Box::from(ListNode::new(0)));
        let out = add_two_numbers(l1, l2);
        assert_eq!(list_to_i32(out.as_deref()), 81);
    }

    #[test]
    fn case5() {
        let l1 = Some(Box::from(ListNode::new_with_next(
            9,
            Box::from(ListNode::new_with_next(1, Box::from(ListNode::new(6)))),
        )));
        let l2 = Some(Box::from(ListNode::new(0)));
        let out = add_two_numbers(l1, l2);
        assert_eq!(list_to_i32(out.as_deref()), 619);
    }
}
