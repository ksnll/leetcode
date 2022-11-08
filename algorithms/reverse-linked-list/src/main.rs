#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            cur = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

fn main() {
    dbg!(Solution::reverse_list(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(3))),
        })),
    }))));
}
