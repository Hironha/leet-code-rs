#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { next, val }
    }
}

// Trivial solution using recursion
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(x), None) | (None, Some(x)) => Some(x),
        (Some(mut x), Some(mut y)) => {
            if x.val >= y.val {
                y.next = merge_two_lists(Some(x), y.next);
                Some(y)
            } else {
                x.next = merge_two_lists(x.next, Some(y));
                Some(x)
            }
        }
    }
}

// Really smart solution to merge lists without recursion and borrow errors
// reference: https://leetcode.com/problems/merge-two-sorted-lists/solution s/2715864/rust-no-allocations-annotated/
pub fn merge_two_lists_proc(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut href = &mut head;

    while list1.is_some() && list2.is_some() {
        let l1ref = &mut list1;
        let l2ref = &mut list2;

        // safe to unwrap since list1 and list2 are some
        if l2ref.as_ref().unwrap().val > l1ref.as_ref().unwrap().val {
            // move list1 value to head
            std::mem::swap(href, l1ref);
            // increment iteration of list1, i.e. set list1 to be next list1 value
            std::mem::swap(l1ref, &mut href.as_mut().unwrap().next);
        } else {
            // move list2 value to head
            std::mem::swap(href, l2ref);
            // increment iteration of list2, i.e. set list2 to be next list2 value
            std::mem::swap(l2ref, &mut href.as_mut().unwrap().next);
        };

        // increment iteration of head, so we always move values to the end of head
        href = &mut href.as_mut().unwrap().next;
    }

    if list1.is_none() {
        // if list1 is empty, then append list2 as next head values
        std::mem::swap(href, &mut list2);
    } else {
        // if list2 is empty, then append list1 as next head values
        std::mem::swap(href, &mut list1);
    };

    head
}

#[cfg(test)]
mod tests {
    use super::{merge_two_lists_proc, ListNode};

    fn is_sorted(head: Option<&ListNode>) -> bool {
        let Some(head) = head else { return true };
        let mut href = head;

        while let (val, Some(next)) = (href.val, &href.next) {
            if val > next.val {
                return false;
            }

            href = next;
        }

        true
    }

    #[test]
    fn case1() {
        let list1 = Box::new(ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                2,
                Some(Box::new(ListNode::new(3, None))),
            ))),
        ));

        let list2 = Box::new(ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                3,
                Some(Box::new(ListNode::new(4, None))),
            ))),
        ));

        let sorted = merge_two_lists_proc(Some(list1), Some(list2));
        assert!(is_sorted(sorted.as_deref()));
    }

    #[test]
    fn case2() {
        let list1 = None;
        let list2 = None;
        let sorted = merge_two_lists_proc(list1, list2);

        assert!(is_sorted(sorted.as_deref()));
    }

    #[test]
    fn case3() {
        let list1 = None;
        let list2 = Box::new(ListNode::new(0, None));
        let sorted = merge_two_lists_proc(list1, Some(list2));

        assert!(is_sorted(sorted.as_deref()));
    }
}
