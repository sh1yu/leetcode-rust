// 剑指 Offer 40. 最小的k个数
// 输入整数数组 arr ，找出其中最小的 k 个数。例如，输入4、5、1、6、2、7、3、8这8个数字，则最小的4个数字是1、2、3、4。
//
// https://leetcode-cn.com/problems/zui-xiao-de-kge-shu-lcof/

use super::Solution;
use std::collections::BinaryHeap;

// 最大堆，堆顶放入最大值，如果还有更小的则弹出并将较小的入堆，O(nlogk)
#[allow(dead_code)]
impl Solution {
    pub fn least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::with_capacity(k as usize);
        for num in arr {
            if heap.len() < k as usize {
                heap.push(num);
            } else {
                if !heap.is_empty() && *heap.peek().unwrap() > num {
                    heap.pop();
                    heap.push(num);
                }
            }
        }
        heap.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![2, 1], Solution::least_numbers(vec![3, 2, 1], 2));
        assert_eq!(vec![0], Solution::least_numbers(vec![0, 1, 2, 1], 1));
    }
}