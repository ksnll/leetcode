use std::cell::RefCell;
use std::rc::Rc;

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
struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_symmetric_helper(
            root.as_ref().and_then(|v| v.borrow().left.clone()),
            root.as_ref().and_then(|v| v.borrow().right.clone()),
        )
    }
    pub fn is_symmetric_helper(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (Some(left), Some(right)) => {
                left.borrow().val == right.borrow().val
                    && Solution::is_symmetric_helper(
                        left.borrow().left.clone(),
                        right.borrow().right.clone(),
                    )
                    && Solution::is_symmetric_helper(
                        left.borrow().right.clone(),
                        right.borrow().left.clone(),
                    )
            }
            (None, None) => true,
            _ => false,
        }
    }
}

fn main() {
    let head = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    head.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    head.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    println!("{}", Solution::is_symmetric(head));
}
