// Category: algorithms
// Level: Easy
// Percent: 43.85888%

// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
//
// You must write an algorithm with O(log n) runtime complexity.
//
//
// Example 1:
//
// Input: nums = [1,3,5,6], target = 5
// Output: 2
//
//
// Example 2:
//
// Input: nums = [1,3,5,6], target = 2
// Output: 1
//
//
// Example 3:
//
// Input: nums = [1,3,5,6], target = 7
// Output: 4
//
//
//
// Constraints:
//
//
// 	1 <= nums.length <= 10⁴
// 	-10⁴ <= nums[i] <= 10⁴
// 	nums contains distinct values sorted in ascending order.
// 	-10⁴ <= target <= 10⁴
//

#![allow(dead_code)]
struct Solution {}
// @leetcode_start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut beg = 0;
        let mut end = nums.len() - 1;

        if target < nums[beg] {
            return 0;
        }

        if target > nums[end] {
            return (end + 1) as i32;
        }

        loop {
            let mid = (beg + end) / 2;
            let num = nums[mid];
            if target == num {
                return mid as i32;
            }
            if mid == beg {
                return (mid + 1) as i32;
            }
            if target > num {
                beg = mid
            }
            if target < num {
                end = mid
            }
        }
    }
}
// @leetcode_end
fn main() {}
