struct Solution;
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

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut string = String::new();
        let mut cur = head;
        while let Some(node) = cur {
            string.push_str(&node.val.to_string().chars().collect::<String>());
            cur = node.next;
        }
        string.clone() == string.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_palindrome(Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        })));
        assert_eq!(result, false);
    }
}
