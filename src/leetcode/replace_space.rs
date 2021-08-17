// 剑指 Offer 05. 替换空格
// 请实现一个函数，把字符串 s 中的每个空格替换成"%20"。
//
// https://leetcode-cn.com/problems/ti-huan-kong-ge-lcof/

use super::Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut res = String::new();
        s.chars().for_each(|ch| {
            if ch == ' ' { res.push_str("%20"); } else { res.push(ch); }
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::replace_space("We are happy.".into()));
    }
}