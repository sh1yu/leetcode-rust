//42. 接雨水
// 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。

use super::Solution;

#[allow(dead_code)]
impl Solution {
    //单调栈，单调降
    //可以考虑括号匹配，每次遇到有上升的柱子时，就知道可能有积水了，这时就应该关闭括号
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut ans = 0;

        for i in 0..height.len() {
            while stack.len() > 0 && height[i] > height[stack[stack.len() - 1]] {
                let t = stack.pop().unwrap();
                if stack.len() == 0 {
                    break;
                }
                let l = stack[stack.len() - 1];
                let w = i - l - 1;
                let h = std::cmp::min(height[i], height[l]) - height[t];
                ans += (w as i32) * h;
            }
            stack.push(i);
        }
        ans
    }

    //朴素法，动态规划，求leftMax和rightMax
    //leftMax = max(leftMax[i-1], height[i])
    //rightMax = max(rightMax[i+1], height[i])
    //trap[i] = min(leftMax[i], rightMax[i]) - height[i]
    pub fn trap2(height: Vec<i32>) -> i32 {
        let mut leftMax: Vec<i32> = vec![0; height.len()];
        let mut rightMax: Vec<i32> = vec![0; height.len()];
        leftMax[0] = height[0];
        rightMax[height.len() - 1] = height[height.len() - 1];
        for i in 1..height.len() {
            leftMax[i] = std::cmp::max(leftMax[i - 1], height[i])
        }
        for i in (0..height.len() - 1).rev() {
            rightMax[i] = std::cmp::max(rightMax[i + 1], height[i])
        }

        let mut res = 0;
        for i in 0..height.len() {
            res += std::cmp::min(leftMax[i], rightMax[i]) - height[i]
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(6, Solution::trap2(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
        assert_eq!(9, Solution::trap2(vec![4, 2, 0, 3, 2, 5]));
    }
}
