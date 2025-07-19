#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(
        list_1: Option<Box<ListNode>>,
        list_2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        return Solution::method_01(list_1, list_2);
    }

    pub fn method_01(
        list_1: Option<Box<ListNode>>,
        list_2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list_1, list_2) {
            // if both list are empty return None
            (None, None) => return None,

            // if either of the list is empty return the other non empty list
            (Some(node), None) => return Some(node),
            (None, Some(node)) => return Some(node),

            // Both are non-empty list so merge these non-empty list
            // create another Linked List to be the sorted list.
            // init left_node & right_node from list_1 & list_2 unwapping the value
            (left_node, right_node) => {
                let mut head = Box::new(ListNode::new(0));
                let mut traverse = &mut head;

                let mut left_node = &left_node;
                let mut right_node = &right_node;

                while left_node.is_some() || right_node.is_some() {
                    traverse.next = Some(Box::new(ListNode::new(0)));
                    traverse = traverse.next.as_mut().unwrap();

                    if left_node.is_some()
                        && (right_node.is_none()
                            || left_node.as_ref().unwrap().val <= right_node.as_ref().unwrap().val)
                    {
                        traverse.val = left_node.as_ref().unwrap().val;
                        left_node = &left_node.as_ref().unwrap().next;
                    } else {
                        traverse.val = right_node.as_ref().unwrap().val;
                        right_node = &right_node.as_ref().unwrap().next;
                    }
                }

                return head.next;
            }
        }
    }

    #[allow(unused)]
    pub fn method_02(
        list_1: Option<Box<ListNode>>,
        list_2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let head = Box::new(ListNode::new(0));

        return None;
    }
}
