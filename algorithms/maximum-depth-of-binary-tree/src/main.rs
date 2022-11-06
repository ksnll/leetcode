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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        fn traverse_helper(
            node: Option<&Rc<RefCell<TreeNode>>>,
            current_depth: i32,
            max_depth: &mut i32,
        ) {
            if let Some(node) = node {
                *max_depth = i32::max(*max_depth, current_depth + 1);
                traverse_helper(node.borrow().left.as_ref(), current_depth + 1, max_depth);
                traverse_helper(node.borrow().right.as_ref(), current_depth + 1, max_depth);
            }
        }
        traverse_helper(root.as_ref(), 0, &mut max_depth);
        max_depth
    }
}
fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    tree.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    tree.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    tree.as_ref()
        .unwrap()
        .borrow()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    tree.as_ref()
        .unwrap()
        .borrow()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    println!("{:?}", Solution::max_depth(tree));
}
