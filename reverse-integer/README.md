## [Reverse Integer](https://leetcode.com/problems/reverse-integer)

Given a signed 32-bit integer `x`, return `x` _with its digits reversed_. If reversing x causes the value to go outside the signed 32-bit integer range `[-231, 231 - 1]`, then return `0`.

**Assume the environment does not allow you to store 64-bit integers (signed or unsigned).**

Example 1:

> Input: x = 123\
> Output: 321

Example 2:

> Input: x = -123\
> Output: -321

Example 3:

> Input: x = 120\
> Output: 21

Constraints:

> -231 <= x <= 231 - 1

## Notes

This problem is actually quite easy. This solutin is O(n), where `n` is the amount of digits in `x`. Just code the following steps.

1. Extract the last digit from `x`;
2. Multiply the answer, starting from 0, by `10` and add extract digit;
    - This step is responsible for concatenating the digit to the answer;
3. Assign `x` to being `x` divided by 10;
    - This step is responsible for removing the last digit from `x`; 