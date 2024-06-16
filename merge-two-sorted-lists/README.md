## [Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/description/)

You are given the heads of two sorted linked lists `list1` and `list2`.

Merge the two lists into one `sorted` list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.

Example 1:
![Example 1](https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg)

> **Input**: list1 = [1,2,4], list2 = [1,3,4]\
> **Output**: [1,1,2,3,4,4]

Example 2:

> **Input**: list1 = [], list2 = []\
> **Output**: []\

Example 3:

> **Input**: list1 = [], list2 = [0] >\
> **Output**: [0]

Constraints:

- The number of nodes in both lists is in the range `[0, 50]`.
- `-100 <= Node.val <= 100`
- Both `list1` and `list2` are sorted in non-decreasing order.

## Notes

When dealing with recursive data structures, it's common to iterate it using recursion. Although not very efficient, it's probably the easiest way to implement it. The recursion solution has a really simple idea:

1. If value of `list1` is bigger than value of `list2`, then `list2` pointer is the start of the list, and recursive merge the remaining items as next pointer;
2. Otherwise, do the same using `list1` as the start of the list.

The imperative approach in Rust is kind of a borrow check hack, but the idea is also simple. Just iterate the lists and move the values to the head according to it's comparison. I'm not going to explain here, since it's gonna be a big text, but the code is commented. So go read it.

- Time Complexity: O(n + m), where `n` is the size of `list1` and `m` the size of `list2`
- Space Complexity: O(1)
