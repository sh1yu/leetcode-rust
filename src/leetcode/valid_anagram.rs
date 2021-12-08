// 242. 有效的字母异位词
// 给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
// 注意：若 s 和 t 中每个字符出现的次数都相同，则称 s 和 t 互为字母异位词。

use super::Solution;
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {

    // 常规hash思路
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut ht = HashMap::new();
        for c in s.chars() {
            *ht.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            let count = ht.entry(c).or_insert(0);
            if *count == 0 {
                return false;
            }
            *count -= 1;
        }
        ht.iter().all(|(_, &v)| v == 0)
    }

    // 方法2，直接使用for_each，更为简短；而且直接将字符映射到数组上，更有效率
    pub fn is_anagram2(s: String, t: String) -> bool {
        let mut hash = [0; 26];
        s.chars().for_each(|ch| hash[(ch as u8 - 'a' as u8) as usize] += 1);
        t.chars().for_each(|ch| hash[(ch as u8 - 'a' as u8) as usize] -= 1);
        !hash.iter().any(|&x| x != 0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, Solution::is_anagram(String::from("anagram"), String::from("nagaram")));
        assert_eq!(false, Solution::is_anagram(String::from("cat"), String::from("car")));
        assert_eq!(false, Solution::is_anagram(String::from("ca"), String::from("car")));
        assert_eq!(false, Solution::is_anagram(String::from("cat"), String::from("ca")));
        assert_eq!(true, Solution::is_anagram2(String::from("anagram"), String::from("nagaram")));
        assert_eq!(false, Solution::is_anagram2(String::from("cat"), String::from("car")));
        assert_eq!(false, Solution::is_anagram2(String::from("ca"), String::from("car")));
        assert_eq!(false, Solution::is_anagram2(String::from("cat"), String::from("ca")));
    }
}