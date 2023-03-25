#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T: PartialOrd> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T: PartialOrd> ListNode<T> {
    pub fn new(val: T, next: Option<Box<ListNode<T>>>) -> ListNode<T> {
        ListNode { val, next }
    }
}

// when dealing with recursive types, it's generally a good idea
// to use recursion, because it will probably make the code easier
// to reason and therefore maintain
// Learned this solution from here:
// https://leetcode.com/problems/merge-two-sorted-lists/solutions/2349126/simple-small-recursive-0ms/?languageTags=rust
pub fn merge_two_lists<T: PartialOrd>(
    list1: Option<Box<ListNode<T>>>,
    list2: Option<Box<ListNode<T>>>,
) -> Option<Box<ListNode<T>>> {
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

#[cfg(test)]
mod tests {
    use crate::merge_two_lists;

    use super::ListNode;

    #[test]
    fn it_correctly_merges() {
        let list1 = Some(Box::new(ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                2,
                Some(Box::new(ListNode::new(4, None))),
            ))),
        )));
        let list2 = Some(Box::new(ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                3,
                Some(Box::new(ListNode::new(4, None))),
            ))),
        )));

        let result = Some(Box::new(ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                1,
                Some(Box::new(ListNode::new(
                    2,
                    Some(Box::new(ListNode::new(
                        3,
                        Some(Box::new(ListNode::new(
                            4,
                            Some(Box::new(ListNode::new(4, None))),
                        ))),
                    ))),
                ))),
            ))),
        )));

        assert_eq!(merge_two_lists(list1, list2), result);
    }
}
