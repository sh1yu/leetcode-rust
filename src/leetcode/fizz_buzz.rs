// 412. Fizz Buzz
// 写一个程序，输出从 1 到 n 数字的字符串表示。
//
// 1. 如果n是3的倍数，输出“Fizz”；
//
// 2. 如果n是5的倍数，输出“Buzz”；
//
// 3.如果n同时是3和5的倍数，输出 “FizzBuzz”。
//

use super::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res = vec!["".into(); n as usize];
        let mut i: usize = 0;
        while i < n as usize {
            let curr = i + 1;
            if curr % 3 == 0 && curr % 5 == 0 {
                res[i] = "FizzBuzz".into();
            } else if curr % 3 == 0 {
                res[i] = "Fizz".into();
            } else if curr % 5 == 0 {
                res[i] = "Buzz".into();
            } else {
                res[i] = curr.to_string();
            }
            i += 1;
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expect = vec!["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"];
        assert_eq!(expect, Solution::fizz_buzz(15));
    }
}