// 20. 有效的括号
// 给定一个只包括 '('，')'，'{'，'}'，'['，']'的字符串 s ，判断字符串是否有效。
//
// 有效字符串需满足：
//
// 左括号必须用相同类型的右括号闭合。
// 左括号必须以正确的顺序闭合。
//
use super::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for i in s.chars() {
            match i {
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '(' => stack.push(')'),
                c => {
                    match stack.pop() {
                        Some(d) => { if c != d { return false; } }
                        None => { return false; }
                    }
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, Solution::is_valid(String::from("{{}}")));
        assert_eq!(true, Solution::is_valid(String::from("{{[]}}")));
        assert_eq!(false, Solution::is_valid(String::from("{{[}]}")));
    }
}