#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn build_tree(values: Vec<i32>) -> TreeNode {
    let pivot = values.len() / 2;
    match values.len() {
        1 => TreeNode {
            value: *values.get(0).unwrap(),
            left: None,
            right: None,
        },
        2 => {
            if let [min, max] = values[..] {
                TreeNode {
                    value: max,
                    left: Some(Box::new(TreeNode {
                        value: min,
                        left: None,
                        right: None,
                    })),
                    right: None,
                }
            } else {
                panic!("Invalid match")
            }
        }
        3 => {
            if let [min, pivot, max] = values[..] {
                TreeNode {
                    value: pivot,
                    left: Some(Box::new(TreeNode {
                        value: min,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(TreeNode {
                        value: max,
                        left: None,
                        right: None,
                    })),
                }
            } else {
                panic!("Wrong size");
            }
        }
        _ => TreeNode {
            value: *values.get(pivot).unwrap(),
            left: Some(Box::new(build_tree(values[0..pivot].to_vec()))),
            right: Some(Box::new(build_tree(values[pivot + 1..].to_vec()))),
        },
    }
}

fn main() {
    dbg!(build_tree(vec![
        1, 2, 10, 34, 244, 1000, 2000, 3000, 4000, 4444, 5000, 5555, 7000, 8000, 9000,
    ]));
}
