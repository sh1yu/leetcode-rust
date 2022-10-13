//13. 罗马数字转整数

//罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。
//字符          数值
//I             1
//V             5
//X             10
//L             50
//C             100
//D             500
//M             1000
//例如， 罗马数字 2 写做 II ，即为两个并列的 1 。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。

//通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。
//数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。
//这个特殊的规则只适用于以下六种情况：

//I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
//X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
//C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
//给定一个罗马数字，将其转换成整数。

//https://leetcode.cn/problems/roman-to-integer
use super::Solution;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref ROMAN_INT_MAP: HashMap<char, i32> = {
        let mut m = HashMap::new();
        m.insert('I', 1);
        m.insert('V', 5);
        m.insert('X', 10);
        m.insert('L', 50);
        m.insert('C', 100);
        m.insert('D', 500);
        m.insert('M', 1000);
        m.insert(' ', 0);
        m
    };
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;

        let cs: Vec<_> = s.chars().collect();
        let mut index = 0;
        while index < cs.len() {
            match cs.get(index) {
                Some('I') => {
                    let next = cs.get(index + 1);
                    if let Some(x) = next {
                        if *x == 'V' {
                            res += 4;
                            index += 1;
                        } else if *x == 'X' {
                            res += 9;
                            index += 1;
                        } else {
                            res += 1;
                        }
                    } else {
                        res = res + 1;
                    }
                }
                Some('V') => res = res + 5,
                Some('X') => {
                    let next = cs.get(index + 1);
                    if let Some(x) = next {
                        if *x == 'L' {
                            res += 40;
                            index += 1;
                        } else if *x == 'C' {
                            res += 90;
                            index += 1;
                        } else {
                            res += 10;
                        }
                    } else {
                        res = res + 10;
                    }
                }
                Some('L') => res = res + 50,
                Some('C') => {
                    let next = cs.get(index + 1);
                    if let Some(x) = next {
                        if *x == 'D' {
                            res += 400;
                            index += 1;
                        } else if *x == 'M' {
                            res += 900;
                            index += 1;
                        } else {
                            res += 100;
                        }
                    } else {
                        res = res + 100;
                    }
                }
                Some('D') => res = res + 500,
                Some('M') => res = res + 1000,
                _ => break,
            }
            index += 1;
        }

        res
    }

    pub fn roman_to_int2(s: String) -> i32 {
        s.chars()
            .fold((0, 0), |acc, cur| {
                let n = match cur {
                    'I' => 1,
                    'V' => 5,
                    'X' => 10,
                    'L' => 50,
                    'C' => 100,
                    'D' => 500,
                    'M' => 1000,
                    _ => 0,
                };
                let new_acc = if acc.1 < n {
                    acc.0 - acc.1 * 2 + n
                } else {
                    acc.0 + n
                };
                (new_acc, n)
            })
            .0
    }

    pub fn roman_to_int3(s: String) -> i32 {
        s.chars()
        .chain(" ".chars())
        .fold((0, 0), |acc, cur| {
            let n = ROMAN_INT_MAP.get(&cur).unwrap();
            let new_acc = if acc.1 < *n {
                acc.0 - acc.1
            } else {
                acc.0 + acc.1
            };
            (new_acc, *n)
        })
        .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, Solution::roman_to_int3("III".into()));
        assert_eq!(4, Solution::roman_to_int3("IV".into()));
        assert_eq!(9, Solution::roman_to_int3("IX".into()));
        assert_eq!(58, Solution::roman_to_int3("LVIII".into()));
        assert_eq!(1994, Solution::roman_to_int3("MCMXCIV".into()));
    }
}
