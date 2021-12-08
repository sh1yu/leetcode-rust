//有些数的素因子只有 3，5，7，请设计一个算法找出第 k 个数。
//注意，不是必须有这些素因子，而是必须不包含其他的素因子。例如，前几个数按顺序应该是 1，3，5，7，9，15，21。

use super::Solution;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

#[allow(dead_code)]
impl Solution {
    pub fn get_kth_magic_number(k: i32) -> i32 {
        if k == 1 {
            return 1;
        }
        let muti_factors = vec![3, 5, 7];
        let mut count = 1;
        let mut heap = BinaryHeap::new();
        let mut duplicate = HashSet::new();
        heap.push(Reverse(1));
        duplicate.insert(1);
        loop {
            let curr = heap.pop().unwrap().0;
            if count == k {
                return curr as i32;
            }
            let next_values: Vec<i64> = muti_factors.iter().map(|x| x * curr).collect();
            for val in next_values {
                if duplicate.insert(val) {
                    heap.push(Reverse(val));
                }
            }
            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, Solution::get_kth_magic_number(1));
        assert_eq!(9, Solution::get_kth_magic_number(5));
        assert_eq!(3215625, Solution::get_kth_magic_number(251));
    }
}
