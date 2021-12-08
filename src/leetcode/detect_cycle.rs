//142. 环形链表 II
// 给定一个链表，返回链表开始入环的第一个节点。如果链表无环，则返回null。
//
// 为了表示给定链表中的环，我们使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。
// 如果 pos 是 -1，则在该链表中没有环。注意，pos 仅仅是用于标识环的情况，并不会作为参数传递到函数中。
//
// 说明：不允许修改给定的链表。
use std::rc::Rc;
use std::cell::RefCell;
use super::{RcListNode, Solution};

#[allow(dead_code)]
impl Solution {
    pub fn detect_cycle(head: Option<Rc<RefCell<RcListNode>>>) -> Option<i32> {
        return match head {
            Some(ref node) => {
                let mut slow = Rc::clone(node);
                let mut fast = Rc::clone(node);
                loop {
                    if slow.borrow().next.is_none() { return None; }
                    let s = Rc::clone(slow.borrow().next.as_ref().unwrap());
                    slow = s;

                    if fast.borrow().next.is_none() { return None; }
                    let f = Rc::clone(fast.borrow().next.as_ref().unwrap());
                    fast = f;
                    if fast.borrow().next.is_none() { return None; }
                    let f = Rc::clone(fast.borrow().next.as_ref().unwrap());
                    fast = f;

                    if fast.as_ptr() == slow.as_ptr() {
                        let mut another = Rc::clone(node);
                        loop {
                            if slow.as_ptr() == another.as_ptr() {
                                return Some(another.borrow().val);
                            }
                            let s = Rc::clone(slow.borrow().next.as_ref().unwrap());
                            slow = s;
                            let a = Rc::clone(another.borrow().next.as_ref().unwrap());
                            another = a;
                        }
                    }
                }
            }
            None => None
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_has_cycle() {
        let node1 = Rc::new(RefCell::new(RcListNode::new(1)));
        let node2 = Rc::new(RefCell::new(RcListNode::new(2)));
        let node3 = Rc::new(RefCell::new(RcListNode::new(3)));
        let node4 = Rc::new(RefCell::new(RcListNode::new(4)));
        let node2_rc = Rc::clone(&node2);
        let node4_rc = Rc::clone(&node4);
        node3.borrow_mut().next = Some(node4);
        node2.borrow_mut().next = Some(node3);
        node1.borrow_mut().next = Some(node2);
        node4_rc.borrow_mut().next = Some(node2_rc);

        let head = Some(node1);
        assert_eq!(Some(2), Solution::detect_cycle(head));
    }

    #[test]
    fn detect_has_not_cycle() {
        let node1 = Rc::new(RefCell::new(RcListNode::new(1)));
        let node2 = Rc::new(RefCell::new(RcListNode::new(2)));
        let node3 = Rc::new(RefCell::new(RcListNode::new(3)));
        let node4 = Rc::new(RefCell::new(RcListNode::new(4)));
        node3.borrow_mut().next = Some(node4);
        node2.borrow_mut().next = Some(node3);
        node1.borrow_mut().next = Some(node2);

        let head = Some(node1);
        assert_eq!(None, Solution::detect_cycle(head));
    }
}