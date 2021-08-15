use std::fmt::{Debug, Display, Formatter, Result};
use std::rc::Rc;
use std::cell::RefCell;

mod merge_two_lists;
mod move_zeros;
mod swap_pairs;
mod three_sum;
mod reverse_list;
mod has_cycle;
mod detect_cycle;
mod circular_deque;
mod remove_duplicates;
mod rotate;
mod merge;
mod two_sum;
mod plus_one;
mod trap;
mod valid_parentheses;
mod min_stack;
mod largest_rectangle_area;
mod valid_anagram;
mod group_anagrams;
mod intersection_of_two_arrays_ii;
mod max_sliding_window;
mod reverse_k_group;
mod inorder_traversal;
mod preorder_traversal;
mod remove_outer_parentheses;
mod fizz_buzz;
mod add_digits;
mod max_depth;
mod get_least_numbers;
mod is_ugly;
mod nth_ugly_number;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
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

impl Solution {
    pub fn format_list(head: &Option<Box<ListNode>>) -> String {
        return match head {
            None => String::from("None"),
            Some(n) => format!("{} -> {}", n.val, Solution::format_list(&n.next))
        };
    }
}
