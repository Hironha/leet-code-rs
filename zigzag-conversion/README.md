## [Zigzag Conversion](https://leetcode.com/problems/zigzag-conversion)

The string `"PAYPALISHIRING"` is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

> P   A   H   N\
> A P L S I I G\
> Y   I   R

And then read line by line: `"PAHNAPLSIIGYIR"`

Write the code that will take a string and make this conversion given a number of rows:

> string convert(string s, int numRows);

Example 1:

> Input: s = "PAYPALISHIRING", numRows = 3\
> Output: "PAHNAPLSIIGYIR"

Example 2:

> Input: s = "PAYPALISHIRING", numRows = 4\
> Output: "PINALSIGYAHRPI"\
> Explanation:\
> P     I    N\
> A   L S  I G\
> Y A   H R\
> P     I

Example 3:

> Input: s = "A", numRows = 1\
> Output: "A"

Constraints:

> 1 <= s.length <= 1000\
> s consists of English letters (lower-case and upper-case), ',' and '.'.\
> 1 <= numRows <= 1000

## Notes

I actually couldn't solve this problem at first. Got trapped thinking in an intuitive but wrong solution and couldn't think in any other approach. Then, I read [this](https://leetcode.com/problems/zigzag-conversion/solutions/817306/very-simple-and-intuitive-o-n-python-solution-with-explanation) explanation and it's actually quite easy to solve.

Basically the idea is to have a string for each row then loop the input string character by character alternating between which row string the character should be concatenated. Read the leet code explanation to understand it since it has some examples.

> P  A  Y  P  A  L  I  S  H  I  R  I  N  G\
> -----------------------------------------\
> 0  1  2  3  2  1  0  1  2  3  2  1  0  1