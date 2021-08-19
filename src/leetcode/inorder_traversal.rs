// 94. 二叉树的中序遍历
// 给定一个二叉树的根节点 root ，返回它的 中序 遍历。

use super::{Solution, TreeNode};

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if let Some(node) = root {
            ans.append(&mut Solution::inorder_traversal(node.borrow_mut().left.take()));
            ans.push(node.borrow().val);
            ans.append(&mut Solution::inorder_traversal(node.borrow_mut().right.take()));
        }
        ans
    }

    pub fn inorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if let Some(node) = root.as_ref() {
            if let Some(left) = node.borrow().left.as_ref() {
                ans.append(&mut Solution::inorder_traversal(Some(Rc::clone(left))));
            }
            ans.push(node.borrow().val);
            if let Some(right) = node.borrow().right.as_ref() {
                ans.append(&mut Solution::inorder_traversal(Some(Rc::clone(right))));
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        node2.borrow_mut().left = Some(node3);
        node1.borrow_mut().right = Some(node2);
        assert_eq!(vec![1, 3, 2], Solution::inorder_traversal(Some(node1)));
    }

    #[test]
    fn it_works2() {
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        node2.borrow_mut().left = Some(node3);
        node1.borrow_mut().right = Some(node2);
        assert_eq!(vec![1, 3, 2], Solution::inorder_traversal2(Some(node1)));
    }
}