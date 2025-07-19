use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Solution::method_2(root);
    }

    // recurssive apporach
    fn method_1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        Solution::method_1_help(
            &root.as_ref().unwrap().borrow().left,
            &root.as_ref().unwrap().borrow().right,
        )
    }

    fn method_1_help(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => return true,
            (Some(left), Some(right)) => {
                return left.borrow().val == right.borrow().val
                    && Solution::method_1_help(&left.borrow().left, &right.borrow().right)
                    && Solution::method_1_help(&left.borrow().right, &right.borrow().left);
            }
            (_, _) => return false,
        }
    }

    // iterative apporach, using stacks
    fn method_2(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // empty tree is symmetric
        if root.is_none() {
            return true;
        }

        // init two stack to store the values
        let mut stack_left = Vec::new();
        let mut stack_right = Vec::new();

        // push root->left to stack_left
        let root = root.take().unwrap();
        stack_left.push(root.borrow().left.clone());
        // push root->right to stack_right
        stack_right.push(root.borrow().right.clone());

        while !stack_left.is_empty() && !stack_right.is_empty() {
            //pop stack_left
            let left = stack_left.pop().unwrap();
            //pop stack_right
            let right = stack_right.pop().unwrap();

            // if both are none continue
            if left.is_none() && right.is_none() {
                continue;
            }

            // if both are different return fasle
            if (left.is_some() && right.is_none()) || (left.is_none() && right.is_some()) {
                return false;
            }

            // if both left and right val are different, not symmetric
            if left.as_ref().unwrap().borrow().val != right.as_ref().unwrap().borrow().val {
                return false;
            }

            // now push into stack_left & stack_right
            // rule:
            // 1. push left->left into stack_left
            // 2. push right->right into stack_right
            //
            // 3. push left->right into stack_left
            // 4. push right->left int stack_right
            // 5. push order should be maintained

            stack_left.push(left.as_ref().unwrap().borrow().left.clone());
            stack_right.push(right.as_ref().unwrap().borrow().right.clone());

            stack_left.push(left.as_ref().unwrap().borrow().right.clone());
            stack_right.push(right.as_ref().unwrap().borrow().left.clone());
        }

        stack_left.is_empty() && stack_right.is_empty()
    }
}
