// 25. K 个一组翻转链表
// 给你一个链表，每k个节点一组进行翻转，请你返回翻转后的链表。
//
// k是一个正整数，它的值小于或等于链表的长度。
//
// 如果节点总数不是k的整数倍，那么请将最后剩余的节点保持原有顺序。
//
// 进阶：
//
// 你可以设计一个只使用常数额外空间的算法来解决此问题吗？
// 你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。
//

use super::{Solution, ListNode};

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut remain = head;
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        while remain.is_some() {
            let (new_head, new_remain) = Solution::reverse_one(remain, k);
            remain = new_remain;
            tail.next = new_head;
            while tail.next.as_ref().is_some() {
                tail = tail.next.as_mut().unwrap();
            }
        }

        dummy.next
    }

    // 反转一次，返回反转后的head和remain
    // 如果为最后一次不足以反转，remain为None
    fn reverse_one(head: Option<Box<ListNode>>, k: i32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut pre = head.as_ref();
        for _ in 0..k {
            if pre.is_none() {
                return (head, None);
            }
            pre = pre.unwrap().next.as_ref();
        }

        let mut remain = head;
        let mut dummy = ListNode::new(0);
        for _ in 0..k {
            if let Some(mut n) = remain {
                remain = n.next.take();
                n.next = dummy.next.take();
                dummy.next = Some(n);
            }
        }

        (dummy.next, remain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut l1 = Option::Some(Box::new(ListNode::new(10)));
        let mut next = l1.as_mut().unwrap();
        next.next = Option::Some(Box::new(ListNode::new(20)));
        next = next.next.as_mut().unwrap();
        next.next = Option::Some(Box::new(ListNode::new(30)));
        next = next.next.as_mut().unwrap();
        next.next = Option::Some(Box::new(ListNode::new(40)));
        next = next.next.as_mut().unwrap();
        next.next = Option::Some(Box::new(ListNode::new(50)));

        println!("{}", Solution::format_list(&Solution::reverse_k_group(l1, 3)));
    }
}