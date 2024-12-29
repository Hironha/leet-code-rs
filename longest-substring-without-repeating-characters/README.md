## [Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters)

Given a string s, find the length of the longest `substring` without repeating characters.

Example 1:

> Input: s = "abcabcbb" \
> Output: 3 \
> Explanation: The answer is "abc", with the length of 3.

Example 2:

> Input: s = "bbbbb" \
> Output: 1 \
> Explanation: The answer is "b", with the length of 1.

Example 3:

> Input: s = "pwwkew" \
> Output: 3 \
> Explanation: The answer is "wke", with the length of 3. Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.


## Notes

Both solutions are presented here share the same time complexity of O(n²). The first naive one uses auxiliary memory, but it's easier to understand. The sliding window one is harder to understand, since it's based on keeping track of indexes, hence being quite easy to be off by one.
