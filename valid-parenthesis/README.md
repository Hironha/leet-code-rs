## [Valid Parenthesis](https://leetcode.com/problems/valid-parentheses/)

Given a string s containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.

An input string is valid if:

1. Open brackets must be closed by the same type of brackets.
2. Open brackets must be closed in the correct order.
3. Every close bracket has a corresponding open bracket of the same type.

Example 1:

> **Input**: s = "()"\
> **Output**: true

Example 2:

> **Input**: s = "()[]{}"\
> **Output**: true

Example 3:

> **Input**: s = "(]"\
> **Output**: false

Constraints:

- `1 <= s.length <= 104`
- `s` consists of parentheses only `'()[]{}'`.

## Notes

The idea to solve this problem is to have a stack to store open parenthesis. While iterating over the string, check if it's a open parenthesis. If it is, push to the stack. Otherwise, if it's a close parenthesis, check if the last open is the correct pair. If it's correct, pop the last open parenthesis from the stack, otherwise return false. After iterating the string, check if there is no remaining open parenthesis in the stack.

- Time Complexity: O(n)
- Space Complexity: O(n)
