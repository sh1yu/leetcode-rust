// 144. 二叉树的前序遍历
// 给你二叉树的根节点 root ，返回它节点值的 前序 遍历。

use super::{Solution, TreeNode};

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if let Some(node) = root.as_ref() {
            ans.push(node.borrow().val);
            if let Some(left) = node.borrow().left.as_ref() {
                ans.append(&mut Solution::preorder_traversal(Some(Rc::clone(left))));
            }
            if let Some(right) = node.borrow().right.as_ref() {
                ans.append(&mut Solution::preorder_traversal(Some(Rc::clone(right))));
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
        println!("{:?}", Solution::preorder_traversal(Some(node1)));
    }
}