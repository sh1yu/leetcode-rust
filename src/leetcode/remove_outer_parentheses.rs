// 1021. 删除最外层的括号
// 有效括号字符串为空 ""、"(" + A + ")"或A + B ，其中A 和B都是有效的括号字符串，+代表字符串的连接。
// 
// 例如，""，"()"，"(())()"和"(()(()))"都是有效的括号字符串。
// 如果有效字符串 s 非空，且不存在将其拆分为 s = A + B的方法，我们称其为原语（primitive），其中A 和B都是非空有效括号字符串。
// 
// 给出一个非空有效字符串 s，考虑将其进行原语化分解，使得：s = P_1 + P_2 + ... + P_k，其中P_i是有效括号字符串原语。
// 
// 对 s 进行原语化分解，删除分解中每个原语字符串的最外层括号，返回 s 。
use super::Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut stack = vec![];
        let mut res = String::new();
        for i in s.chars() {
            match i {
                '(' => {
                    if !stack.is_empty() {
                        res.push('(');
                    }
                    stack.push(')');
                }
                _ => {
                    stack.pop();
                    if !stack.is_empty() {
                        res.push(')');
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("()()()", Solution::remove_outer_parentheses("(()())(())".into()));
        assert_eq!("()()()()(())", Solution::remove_outer_parentheses("(()())(())(()(()))".into()));
        assert_eq!("", Solution::remove_outer_parentheses("()()".into()));
    }
}