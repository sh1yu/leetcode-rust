// 264. 丑数 II
// 给你一个整数 n ，请你找出并返回第 n 个 丑数 。
//
// 丑数 就是只包含质因数 2、3 和/或 5 的正整数。
// https://leetcode-cn.com/problems/ugly-number-ii/

use super::Solution;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut set = HashSet::new();
        let mut heap = BinaryHeap::new();
        set.insert(1);
        heap.push(Reverse(1));
        let mut curr = 0;
        for _ in 0..n {
            curr = heap.pop().unwrap().0;
            let mut next = curr as i64 * 2;
            if next < i32::MAX as i64 && set.insert(next) {
                heap.push(Reverse(next as i32));
            }
            next = curr as i64 * 3;
            if next < i32::MAX as i64 && set.insert(next) {
                heap.push(Reverse(next as i32));
            }
            next = curr as i64 * 5;
            if next < i32::MAX as i64 && set.insert(next) {
                heap.push(Reverse(next as i32));
            }
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::nth_ugly_number(1));
        println!("{:?}", Solution::nth_ugly_number(10));
        println!("{:?}", Solution::nth_ugly_number(1407));
    }
}
