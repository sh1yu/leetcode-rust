// 1. 两数之和
// 给定一个整数数组 nums和一个整数目标值 target，请你在该数组中找出 和为目标值 target 的那两个整数，并返回它们的数组下标。
//
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
//
// 你可以按任意顺序返回答案。
//
use super::{Solution};
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ht = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            if ht.contains_key(&num) {
                return vec![*ht.get(&num).unwrap() as i32, i as i32];
            }
            ht.insert(target - num, i);
        }
        vec![]
    }

    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ht = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            match ht.get(&num) {
                Some(&n) => {return vec![n, i as i32];}
                None => {ht.insert(target-num, i as i32);}
            }
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }

    #[test]
    fn it_works2() {
        assert_eq!(vec![0, 1], Solution::two_sum2(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum2(vec![3, 2, 4], 6));
    }
}