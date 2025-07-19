#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut current) = (None, head);

        while let Some(mut node) = current {
            current = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}
