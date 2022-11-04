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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 1 {
            Some(Rc::new(RefCell::new(TreeNode::new(
                nums.first().unwrap().to_owned(),
            ))))
        } else if nums.len() == 2 {
            let node_a = nums.get(0).unwrap();
            let node_b = nums.get(1).unwrap();
            let mut tree_node = TreeNode::new(node_b.to_owned());
            tree_node.left = Some(Rc::new(RefCell::new(TreeNode::new(node_a.to_owned()))));
            Some(Rc::new(RefCell::new(tree_node)))
        } else if nums.len() == 3 {
            let node_a = nums.get(0).unwrap();
            let node_b = nums.get(1).unwrap();
            let node_c = nums.get(2).unwrap();
            let mut tree_node = TreeNode::new(node_b.to_owned());
            tree_node.left = Some(Rc::new(RefCell::new(TreeNode::new(node_a.to_owned()))));
            tree_node.right = Some(Rc::new(RefCell::new(TreeNode::new(node_c.to_owned()))));
            Some(Rc::new(RefCell::new(tree_node)))
        } else {
            let pivot = nums.len() / 2;
            let mut tree_node = TreeNode::new(nums[pivot]);
            tree_node.left =
                Some(Solution::sorted_array_to_bst(nums[..=pivot - 1].to_vec()).unwrap());
            tree_node.right =
                Some(Solution::sorted_array_to_bst(nums[pivot + 1..].to_vec()).unwrap());
            Some(Rc::new(RefCell::new(tree_node)))
        }
    }
}

fn main() {
    Solution::sorted_array_to_bst(vec![0, 2, 3, 4]);
}
