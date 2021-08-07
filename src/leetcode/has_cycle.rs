//141. 环形链表
//给定一个链表，判断链表中是否有环。
//
// 如果链表中有某个节点，可以通过连续跟踪 next 指针再次到达，则链表中存在环。
// 为了表示给定链表中的环，我们使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。
// 如果 pos 是 -1，则在该链表中没有环。注意：pos 不作为参数进行传递，仅仅是为了标识链表的实际情况。
//
// 如果链表中存在环，则返回 true 。 否则，返回 false 。
use std::rc::Rc;
use std::cell::RefCell;
use super::{Solution};

pub struct ListNode {
    _val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    pub fn new(val: i32) -> ListNode {
        ListNode {
            _val: val,
            next: None,
        }
    }
}

impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        return match head {
            Some(ref node) => {
                let mut slow = Rc::clone(node);
                let mut fast = Rc::clone(node);
                loop {
                    if slow.borrow().next.is_none() { return false; }
                    let t = Rc::clone(slow.borrow().next.as_ref().unwrap());
                    slow = t;

                    if fast.borrow().next.is_none() { return false; }
                    let r = Rc::clone(fast.borrow().next.as_ref().unwrap());
                    fast = r;
                    if fast.borrow().next.is_none() { return false; }
                    let r = Rc::clone(fast.borrow().next.as_ref().unwrap());
                    fast = r;

                    if fast.as_ptr() == slow.as_ptr() { return true; }
                }
            }
            None => false
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_cycle() {
        let node1 = Rc::new(RefCell::new(ListNode::new(1)));
        let node2 = Rc::new(RefCell::new(ListNode::new(2)));
        let node3 = Rc::new(RefCell::new(ListNode::new(3)));
        let node4 = Rc::new(RefCell::new(ListNode::new(4)));
        let node2_rc = Rc::clone(&node2);
        let node4_rc = Rc::clone(&node4);
        node3.borrow_mut().next = Some(node4);
        node2.borrow_mut().next = Some(node3);
        node1.borrow_mut().next = Some(node2);
        node4_rc.borrow_mut().next = Some(node2_rc);

        let head = Some(node1);
        println!("{}", Solution::has_cycle(head));
    }

    #[test]
    fn it_not_has_cycle() {
        let node1 = Rc::new(RefCell::new(ListNode::new(1)));
        let node2 = Rc::new(RefCell::new(ListNode::new(2)));
        let node3 = Rc::new(RefCell::new(ListNode::new(3)));
        let node4 = Rc::new(RefCell::new(ListNode::new(4)));
        node3.borrow_mut().next = Some(node4);
        node2.borrow_mut().next = Some(node3);
        node1.borrow_mut().next = Some(node2);

        let head = Some(node1);
        println!("{}", Solution::has_cycle(head));
    }
}