// 剑指 Offer 06. 从尾到头打印链表
// 输入一个链表的头节点，从尾到头反过来返回每个节点的值（用数组返回）。
//
// https://leetcode-cn.com/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/

use super::{Solution, ListNode};

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = vec![];
        match head {
            Some(n) => {
                ans.append(&mut Solution::reverse_print(n.next));
                ans.push(n.val);
                return ans;
            }
            None => ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut head = Option::Some(Box::new(ListNode::new(1)));
        let mut tail = head.as_mut().unwrap();
        tail.next = Option::Some(Box::new(ListNode::new(2)));
        tail = tail.next.as_mut().unwrap();
        tail.next = Option::Some(Box::new(ListNode::new(3)));
        tail = tail.next.as_mut().unwrap();
        tail.next = Option::Some(Box::new(ListNode::new(4)));
        tail = tail.next.as_mut().unwrap();
        tail.next = Option::Some(Box::new(ListNode::new(5)));

        println!("{:?}", Solution::reverse_print(head));
    }
}