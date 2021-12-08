// 49. 字母异位词分组
// 给定一个字符串数组，将字母异位词组合在一起。可以按任意顺序返回结果列表。
// 字母异位词指字母相同，但排列不同的字符串。

use super::Solution;
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {

        let mut ht = HashMap::new();
        strs.into_iter().for_each(|str| {
            let mut hash_key = [0; 26];
            str.chars().for_each(|ch| hash_key[(ch as u8 - 'a' as u8) as usize] += 1);
            ht.entry(hash_key).or_insert(vec![]).push(str);
        });
        // .cloned() 类似于 .map(|i| i.clone())
        ht.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let mut expected = vec![vec!["eat", "tea", "ate"], vec!["tan", "nat"], vec!["bat"]];
        let mut actual = Solution::group_anagrams(vec!["eat".into(), "tea".into(), "tan".into(), "ate".into(), "nat".into(), "bat".into()]);
        assert_eq!(expected.sort(), actual.sort());

        assert_eq!(vec![vec![""]], Solution::group_anagrams(vec!["".into()]));

        assert_eq!(vec![vec!["a"]], Solution::group_anagrams(vec!["a".into()]));
    }
}