## [Climbing Stairs](https://leetcode.com/problems/climbing-stairs)

You are climbing a staircase. It takes `n` steps to reach the top.

Each time you can either climb `1` or `2` steps. In how many distinct ways can you climb to the top?

Example 1:

> Input: n = 2\
> Output: 2\
> Explanation: There are two ways to climb to the top.\
> 1\. 1 step + 1 step\
> 2\. 2 steps

Example 2:

> Input: n = 3\
> Output: 3\
> Explanation: There are three ways to climb to the top.\
> 1\. 1 step + 1 step + 1 step
> 2\. 1 step + 2 steps
> 3\. 2 steps + 1 step

Constraints:

- `1 <= n <= 45`

## Notes

Actually quite hard problem to think about. All the problems that require a bottom-up approach confuses me. But anyway, the solution to this problems is, as said before, to use a bottom-up approach. If we start calculating from the end, it's quite easy to get the result.

Let's say we want to climb `4` stairs and we start at the end, _i.e_ at the step `5`. Then, to get to `4`, we only have one option, which is `1` step, since `2` steps would get us to step `3`.

Now, what about to step `3`? Well, we can either go `1` step twice or `2` steps once. Se we have `2` options.

Now, here is the magic. What about to step `2`? If we think that it's the same as trying to go from `2` to `5`, we can assume that it would required the same amount of steps to get from `5` to `3` plus the amount of steps required from `5` to `4`. That is because from `2` we can either go `1` step, reaching `3`, or `2` steps, reaching `4`. Then, the result is `2` (from `5` to `3`) + `1` (from `5` to `4`), which equals `3`.

We can use the previous idea for all the next steps. Leaving us with the equation: `f(n) = f(n - 1) + f(n - 2)`, where
`f(0) = 0` and `f(1) = 1`. Yes, this equation is the same as the fibonacci sequence.
