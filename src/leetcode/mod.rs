use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter, Result};
use std::rc::Rc;

mod add_digits;
mod circular_deque;
mod detect_cycle;
mod fizz_buzz;
mod get_kth_magic_number;
mod get_least_numbers;
mod group_anagrams;
mod has_cycle;
mod inorder_traversal;
mod intersection_of_two_arrays_ii;
mod is_ugly;
mod largest_rectangle_area;
mod max_depth;
mod max_sliding_window;
mod merge;
mod merge_two_lists;
mod min_stack;
mod move_zeros;
mod nth_ugly_number;
mod plus_one;
mod preorder_traversal;
mod remove_duplicates;
mod remove_outer_parentheses;
mod replace_space;
mod reverse_k_group;
mod reverse_list;
mod reverse_print;
mod rotate;
mod swap_pairs;
mod three_sum;
mod top_k_frequent;
mod trap;
mod two_sum;
mod valid_anagram;
mod valid_parentheses;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.val)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RcListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<RcListNode>>>,
}

#[allow(dead_code)]
impl RcListNode {
    pub fn new(val: i32) -> RcListNode {
        RcListNode {
            val: val,
            next: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn format_list(head: &Option<Box<ListNode>>) -> String {
        return match head {
            None => String::from("None"),
            Some(n) => format!("{} -> {}", n.val, Solution::format_list(&n.next)),
        };
    }
}
