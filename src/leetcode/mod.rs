use std::fmt::{Debug, Display, Formatter, Result};

mod merge_two_lists;
mod move_zeros;
mod swap_pairs;
mod three_sum;
mod reverse_list;

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

pub struct Solution {}

impl Solution {
    pub fn format_list(head: &Option<Box<ListNode>>) -> String {
        return match head {
            None => String::from("None"),
            Some(n) => format!("{} -> {}", n.val, Solution::format_list(&n.next))
        };
    }
}
