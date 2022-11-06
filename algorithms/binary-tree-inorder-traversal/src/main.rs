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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut solution = Vec::new();
        fn traverse(root: Option<Rc<RefCell<TreeNode>>>, solution: &mut Vec<i32>) {
            if let Some(n) = root {
                traverse(n.borrow().left.clone(), solution);
                solution.push(n.borrow().val);
                traverse(n.borrow().right.clone(), solution);
            }
        }
        traverse(root, &mut solution);
        solution
    }
}
fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    if let Some(ref tree) = tree {
        tree.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        if let Some(ref right) = tree.borrow().right {
            right.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        }
    }
    println!("{:?}", Solution::inorder_traversal(tree));
}
