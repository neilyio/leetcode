// Category: algorithms
// Level: Easy
// Percent: 52.23365%

// You are climbing a staircase. It takes n steps to reach the top.
//
// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
//
//
// Example 1:
//
// Input: n = 2
// Output: 2
// Explanation: There are two ways to climb to the top.
// 1. 1 step + 1 step
// 2. 2 steps
//
//
// Example 2:
//
// Input: n = 3
// Output: 3
// Explanation: There are three ways to climb to the top.
// 1. 1 step + 1 step + 1 step
// 2. 1 step + 2 steps
// 3. 2 steps + 1 step
//
//
//
// Constraints:
//
//
// 	1 <= n <= 45
//

#![allow(dead_code)]
struct Solution {}
// @leetcode_start
impl Solution {
    fn climb_stairs_with_cache(n: usize, cache: &mut [usize; 46]) -> usize {
        if n == 1 {
            return 1;
        }

        if n == 2 {
            return 2;
        }

        if cache[n] != 0 {
            return cache[n];
        }

        let result = Solution::climb_stairs_with_cache(n - 1, cache)
            + Solution::climb_stairs_with_cache(n - 2, cache);

        cache[n] = result;

        return result;
    }

    fn climb_stairs(n: i32) -> i32 {
        let mut cache: [usize; 46] = [0; 46];

        return Solution::climb_stairs_with_cache(n as usize, &mut cache) as i32;
    }
}

// @leetcode_end
fn main() {}
