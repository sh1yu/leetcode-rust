// 263. 丑数
// 给你一个整数 n ，请你判断 n 是否为 丑数 。如果是，返回 true ；否则，返回 false 。
//
// 丑数 就是只包含质因数2、3 和/或5的正整数。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/ugly-number

use super::Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 { return false }
        let mut x = n;
        while x != 1 && x % 2 == 0 { x = x / 2 }
        if x == 1 { return true; }
        while x != 1 && x % 3 == 0 { x = x / 3 }
        if x == 1 { return true; }
        while x != 1 && x % 5 == 0 { x = x / 5 }
        return x == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::is_ugly(1));
        println!("{:?}", Solution::is_ugly(6));
        println!("{:?}", Solution::is_ugly(7));
        println!("{:?}", Solution::is_ugly(8));
        println!("{:?}", Solution::is_ugly(14));
    }
}