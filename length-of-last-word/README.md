## [Length of Last Word](https://leetcode.com/problems/length-of-last-word)

Given a string `s` consisting of words and spaces, return _the length of the **last** word in the string_.

A word is a maximal substring consisting of non-space characters only.

Example 1:

> Input: s = "Hello World"\
> Output: 5\
> Explanation: The last word is "World" with length 5.

Example 2:

> Input: s = " fly me to the moon "\
> Output: 4\
> Explanation: The last word is "moon" with length 4.

Example 3:

> Input: s = "luffy is still joyboy"\
> Output: 6\
> Explanation: The last word is "joyboy" with length 6.

Constraints:

- `1 <= s.length <= 104`
- `s` consists of only English letters and spaces `' '`.
- There will be at least one word in `s`.

## Notes

Really easy problem. Just iterate in the string in reverse until you find a sequence of characters that are different than whitespace and count them.

- Time Complexity: O(n), where `n` is the length of `s`
- Space Complexity: O(1), since no auxiliary space is required
