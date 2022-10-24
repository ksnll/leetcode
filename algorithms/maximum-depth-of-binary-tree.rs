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
    pub fn new<MaybeTreeNode>(val: i32, left: MaybeTreeNode, right: MaybeTreeNode) -> Self
    where
        MaybeTreeNode: Into<Option<TreeNode>>,
    {
        TreeNode {
            val,
            left: left.into().map(|l| (Rc::new(RefCell::new(l)))),
            right: right.into().map(|r| (Rc::new(RefCell::new(r)))),
        }
    }
}
struct Solution;

impl Solution {
    pub fn max_depth(root: &TreeNode) -> i32 {
        let mut max_depth = 0;
        fn traverse_helper(node: &TreeNode, current_depth: i32, max_depth: &mut i32) {
            *max_depth = i32::max(*max_depth, current_depth + 1);
            if let Some(l) = &node.left {
                traverse_helper(&l.borrow(), current_depth + 1, max_depth);
            }
            if let Some(r) = &node.right {
                traverse_helper(&r.borrow(), current_depth + 1, max_depth);
            }
        }
        traverse_helper(root, 0, &mut max_depth);
        max_depth
    }
}
fn main() {
    let tree = TreeNode::new(
        1,
        TreeNode::new(9, None, None),
        TreeNode::new(
            20,
            TreeNode::new(20, None, None),
            TreeNode::new(7, None, None),
        ),
    );

    println!("{:?}", Solution::max_depth(&tree));
}
