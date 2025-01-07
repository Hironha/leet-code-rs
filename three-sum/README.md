## [3Sum](https://leetcode.com/problems/3sum)

Given an integer array nums, return all the triplets `[nums[i], nums[j], nums[k]]` such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] + nums[k] == 0`.

Notice that the solution set must not contain duplicate triplets. 

Example 1:

> Input: nums = [-1,0,1,2,-1,-4]\
> Output: [[-1,-1,2],[-1,0,1]]\
> Explanation: \
> nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.\
> nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.\
> nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.\
> The distinct triplets are [-1,0,1] and [-1,-1,2].\
> Notice that the order of the output and the order of the triplets does not matter.

Example 2:

> Input: nums = [0,1,1]\
> Output: []\
> Explanation: The only possible triplet does not sum up to 0.

Example 3:

> Input: nums = [0,0,0]\
> Output: [[0,0,0]]\
> Explanation: The only possible triplet sums up to 0.

Constraints:

> 3 <= nums.length <= 3000\
> -105 <= nums[i] <= 105


## Notes

This is actually a hard problem to solve efficiently. I, of course, solved by bruteforce, checking all possible sums and removing duplicates. But the performance is too slow to be accepted as an answer.

The actual (optimal) [solution](https://leetcode.com/problems/3sum/solutions/5055810/video-two-pointer-solution), is quite hard to understand. The main idea is to first sort the array. The array must be sorted so we can easily scan through it to find combination of values using a two pointer approach. So the solution is like this:

    1. Sort the array. The sort can be unstable, does not matter;
    2. Loop through the array. This is the main loop. Take `i` as index of this loop;
    3. Inside the main loop, find a pair of values that the sum of them equals to `-1 * nums[i]`.
        - To find the sum we can use two pointer approach. Since the array is sorted, is quite easy to traverse it. Say we have two pointers `j` and `k`, where `j` starts from `i + 1`  and `k` from `nums.length - 1`. Note that `j` can start from `i + 1` only because the order of the values in the solution sets does not matter. Hence, we can start `j` as `i + 1` to not check the same values twice. 
        - If the sum of `nums[i] + nums[j] + nums[k]` is negative, it means that the sum `nums[j] + nums[k]` needs to be smaller, since `nums[i]` is provided from the main loop. Hence, we need to increment the `j`, because the array is sorted, it's guaranteed for `nums[j + 1]` to be equal or greater to `nums[j]`. If the sum is negative, we can decrement the `k`. And if the sum equal `0`, we add the triple to the answer and increment `j` until `nums[j + n]` is greater than `nums[j]`.
        
