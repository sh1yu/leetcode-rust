// 84. 柱状图中最大的矩形
// 给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。
//
// 求在该柱状图中，能够勾勒出来的矩形的最大面积。

use super::Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut max_area = 0;
        for i in 0..=heights.len() {
            let cur_height = if i == heights.len() { 0 } else { heights[i] };

            while !stack.is_empty() && cur_height < heights[*stack.last().unwrap()] {
                let h = stack.pop().unwrap();
                if stack.is_empty() {
                    //柱子h高度的右边界为i，左边界为0
                    max_area = std::cmp::max(max_area, i as i32 * heights[h]);
                } else {
                    //柱子h高度的右边界为i，左边界为 *stack.last().unwrap() + 1
                    max_area = std::cmp::max(max_area, (i - *stack.last().unwrap() -1) as i32 * heights[h]);
                }
            }
            stack.push(i);
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
        println!("{}", Solution::largest_rectangle_area(vec![2, 4]));
        println!("{}", Solution::largest_rectangle_area(vec![1, 4, 4, 2, 4, 4]));
    }
}