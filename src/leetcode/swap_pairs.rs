//24. 两两交换链表中的节点
//给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。
// 你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        let mut remain = head;

        while let Some(mut n1) = remain {
            remain = n1.next.take();
            if let Some(mut n2) = remain {
                remain = n2.next.take();

                n2.next = Some(n1);
                tail.next = Some(n2);
                tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                tail.next = Some(n1);
            }
        };

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut l1 = Option::Some(Box::new(ListNode::new(10)));
        l1.as_mut().unwrap().next = Option::Some(Box::new(ListNode::new(20)));
        l1.as_mut().unwrap().next.as_mut().unwrap().next = Option::Some(Box::new(ListNode::new(30)));
        l1.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Option::Some(Box::new(ListNode::new(40)));
        l1.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Option::Some(Box::new(ListNode::new(50)));

        println!("{:?}", Solution::swap_pairs(l1));
    }
}