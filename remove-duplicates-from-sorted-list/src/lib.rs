#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }
}

pub fn remove_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => None,
        Some(mut h) => match h.next {
            None => Some(h),
            Some(n) => {
                if n.val == h.val {
                    // removes the intermediate(duplicated) value 
                    h.next = n.next;
                    // loop through the list recursively doing the swap above
                    return remove_duplicates(Some(h));
                }
                // loop through the list recursively doing the swap above
                h.next = remove_duplicates(Some(n));
                Some(h)
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_success() {
        // [1, 1, 2]
        let input = ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                1,
                Some(Box::new(ListNode::new(2, None))),
            ))),
        );
        // [1, 2]
        let result = ListNode::new(1, Some(Box::new(ListNode::new(2, None))));
        assert_eq!(remove_duplicates(Some(Box::new(input))), Some(Box::new(result)));
    }
}
