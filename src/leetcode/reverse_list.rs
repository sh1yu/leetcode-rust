//206. 反转链表
//给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。

use super::{ListNode, Solution};

#[allow(dead_code)]
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut remain = head;

        while let Some(mut n) = remain {
            remain = n.next.take();
            n.next = dummy.next;
            dummy.next = Some(n);
        }

        dummy.next
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

        assert_eq!(
            "5 -> 4 -> 3 -> 2 -> 1 -> None",
            Solution::format_list(&Solution::reverse_list(head))
        );
    }
}
