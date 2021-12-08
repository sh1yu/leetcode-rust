// 104. 二叉树的最大深度
// 给定一个二叉树，找出其最大深度。
//
// 二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。
//
// 说明:叶子节点是指没有子节点的节点。
//
// 示例：
// 给定二叉树 [3,9,20,null,null,15,7]，返回它的最大深度 3 。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/maximum-depth-of-binary-tree

use std::rc::Rc;
use std::cell::RefCell;
use super::{Solution, TreeNode};

//思路：递归
#[allow(dead_code)]
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root.as_ref() {
            Some(node) => {
                let max_left = match node.borrow().left.as_ref() {
                    Some(left) => Solution::max_depth(Some(Rc::clone(left))),
                    None => 0
                };
                let max_right = match node.borrow().right.as_ref() {
                    Some(right) => Solution::max_depth(Some(Rc::clone(right))),
                    None => 0
                };
                std::cmp::max(max_left, max_right) + 1
            }
            None => 0
        }
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

        assert_eq!(3, Solution::max_depth(Some(node1)));
    }
}