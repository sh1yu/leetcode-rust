// 350. 两个数组的交集 II
// 给定两个数组，编写一个函数来计算它们的交集。
// 说明：
//
// 输出结果中每个元素出现的次数，应与元素在两个数组中出现次数的最小值一致。
// 我们可以不考虑输出结果的顺序。
// 进阶：
//
// 如果给定的数组已经排好序呢？你将如何优化你的算法？ //双指针，相同则同时移动，不同则较大者等待
// 如果nums1的大小比nums2小很多，哪种方法更优？
// 如果nums2的元素存储在磁盘上，内存是有限的，并且你不能一次加载所有的元素到内存中，你该怎么办？
//
use super::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut ht = HashMap::new();
        for i in nums1.iter() {
            *ht.entry(*i).or_insert(0) += 1;
        }
        for i in nums2.iter() {
            let account = ht.entry(*i).or_insert(0);
            if *account > 0 {
                ans.push(*i);
                *account -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![1, 2, 2, 5, 5], Solution::intersect(vec![1, 2, 3, 4, 2, 5, 5, 5], vec![1, 2, 2, 5, 5]));
        assert_eq!(vec![2, 2], Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]));
        assert_eq!(vec![9, 4], Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]));
    }
}