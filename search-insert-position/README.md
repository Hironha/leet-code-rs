## [Search Insert Position](https://leetcode.com/problems/search-insert-position)

Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You must write an algorithm with `O(log n)` runtime complexity.

Example 1:

> Input: nums = [1,3,5,6], target = 5\
> Output: 2

Example 2:

> Input: nums = [1,3,5,6], target = 2\
> Output: 1

Example 3:

> Input: nums = [1,3,5,6], target = 7\
> Output: 4

Constraints:

- 1 <= nums.length <= 104
- -104 <= nums[i] <= 104
- nums contains distinct values sorted in ascending order.
- -104 <= target <= 104

## Notes

This problems is basically asking to implement binary search. Binary search is a really simple algorithm, we just gotta be careful about being off by one. The implemented solution uses a lower bound solution, se we increment `+1` in `low` after assigning a new value to it.

- Time Complexity: O(log n), where `n` is the length of `nums`, as stated in the problem
- Space Complexity: O(1), since no auxiliary space is required
