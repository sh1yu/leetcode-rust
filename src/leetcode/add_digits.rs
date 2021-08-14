// 258. 各位相加
// 给定一个非负整数 num，反复将各个位上的数字相加，直到结果为一位数。

use super::Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        (num - 1) % 9 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::add_digits(15));
        println!("{:?}", Solution::add_digits(99));
        println!("{:?}", Solution::add_digits(1024));
        println!("{:?}", Solution::add_digits(56));
    }
}