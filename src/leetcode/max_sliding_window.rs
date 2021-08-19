// 239. 滑动窗口最大值
// 给你一个整数数组 nums，有一个大小为k的滑动窗口从数组的最左侧移动到数组的最右侧。
// 你只可以看到在滑动窗口内的 k个数字。滑动窗口每次只向右移动一位。
//
// 返回滑动窗口中的最大值。
//

use super::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue = VecDeque::new();
        for i in 0..k as usize {
            while !queue.is_empty() && nums[i] > nums[*queue.back().unwrap()] {
                queue.pop_back();
            }
            queue.push_back(i);
        }

        let mut res = vec![nums[*queue.front().unwrap()]];
        for i in k as usize..nums.len() {
            while !queue.is_empty() && nums[i] > nums[*queue.back().unwrap()] {
                queue.pop_back();
            }
            queue.push_back(i);
            while *queue.front().unwrap() <= (i - k as usize) {
                queue.pop_front();
            }
            res.push(nums[*queue.front().unwrap()])
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![3, 3, 5, 5, 6, 7], Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3));
        assert_eq!(vec![1], Solution::max_sliding_window(vec![1], 1));
        assert_eq!(vec![1, -1], Solution::max_sliding_window(vec![1, -1], 1));
        assert_eq!(vec![11], Solution::max_sliding_window(vec![9, 11], 2));
        assert_eq!(vec![4], Solution::max_sliding_window(vec![4, -2], 2));
    }
}