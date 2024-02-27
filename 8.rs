
struct TreeNode {
    val: i32, 
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            // Recursively calculate the maximum depth of the left and right subtrees
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            
            // Return the maximum depth of the left and right subtrees plus 1 (for the current node)
            1 + left_depth.max(right_depth)
        }
        None => 0, // Return 0 for an empty tree
    }
}

fn main() {
    // Example binary tree
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode::new(15))),
            right: Some(Box::new(TreeNode::new(7))),
        })),
    }));

    let depth = max_depth(root);
    println!("Maximum depth of the tree: {}", depth);
}
