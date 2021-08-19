// 15. 三数之和
//给你一个包含 n 个整数的数组nums，判断nums中是否存在三个元素 a，b，c ，使得a + b + c = 0 ？
// 请你找出所有和为 0 且不重复的三元组。
//
// 注意：答案中不可以包含重复的三元组。

use super::{Solution};

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if nums.len() < 3 {
            return result;
        }
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len() - 2 {
            if nums[i] > 0 {
                return result;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == 0 {
                    result.push(vec![nums[i], nums[j], nums[k]]);
                }
                if sum <= 0 {
                    let pre_j = nums[j];
                    while pre_j == nums[j] && j < k {
                        j += 1;
                    }
                }
                if sum >= 0 {
                    let pre_k = nums[k];
                    while pre_k == nums[k] && j < k {
                        k -= 1;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![vec![-1, -1, 2], vec![-1, 0, 1]], Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
        assert_eq!(vec![vec![-2, 0, 2]], Solution::three_sum(vec![-2, 0, 0, 2, 2]));
    }
}