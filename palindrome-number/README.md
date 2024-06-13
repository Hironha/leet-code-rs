## [Palindrome Number](https://leetcode.com/problems/palindrome-number/description/)

Given an integer `x`, return `true` if `x` is a palindrome, and `false` otherwise.

Example 1:

> **Input**: x = 121\
> **Output**: true\
> Explanation: 121 reads as 121 from left to right and from right to left.

Example 2:

> **Input**: x = -121\
> **Output**: false\
> Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

Example 3:

> **Input**: x = 10\
> **Output**: false\
> Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

## Notes

It's really easy to solve this problem in an efficient way. All we need is some math tricks. If we calculate `x mod 10` or `x % 10`, it's possible to get the last digit of a number. And if we calculate `x / 10`, we can get as result `x` without the last digit. And if we calculate `x * 10 + y`, it's possible to shift the digits to the left and append another digit.
