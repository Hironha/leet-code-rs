## Longest Common Prefix

Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string `""`.

Example 1:

> **Input**: strs = ["flower","flow","flight"]\
> **Output**: "fl"\

Example 2:

> **Input**: strs = ["dog","racecar","car"]\
> **Output**: ""\
> Explanation: There is no common prefix among the input strings.

Constraints:

- `1 <= strs.length <= 200`
- `0 <= strs[i].length <= 200`
- `strs[i]` consists of only lowercase English letters.

## Notes

The solution is really easy. The idea is to just compare the characters of some string in the list with all other strings.

Given `n` as the length of LCP and `m` as number of strings in the list:

- Time Complexity: O(n \* m)
- Space Complexity: O(n), since it needs to allocate a new string with `n` chars
