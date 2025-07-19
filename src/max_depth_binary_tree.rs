use std::cmp;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => return 0,
            Some(node) => return Self::inner_max_depth(&node),
        }
    }

    fn inner_max_depth(node: &Rc<RefCell<TreeNode>>) -> i32 {
        match (&node.borrow().left, &node.borrow().right) {
            (None, None) => {
                return 1;
            }
            (Some(left), None) => {
                return 1 + Self::inner_max_depth(left);
            }
            (None, Some(right)) => {
                return 1 + Self::inner_max_depth(right);
            }
            (Some(left), Some(right)) => {
                return 1 + cmp::max(Self::inner_max_depth(left), Self::inner_max_depth(right));
            }
        }
    }
}
