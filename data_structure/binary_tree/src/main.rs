#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn main() {
    let tree = self::generate_tree_recu(3, &mut 0);
    print_tree_recu(&tree, 0);
    println!("--------------------------------------");
    let inv_tree = self::invert_tree_recu(tree);
    print_tree_recu(&inv_tree, 0);
}

fn invert_tree_recu(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    if let Some(node) = root {
        return Some(Box::new(TreeNode {
            val: node.val.clone(),
            left: self::invert_tree_recu(node.right),
            right: self::invert_tree_recu(node.left),
        }));
    } else {
        return None;
    }
}

fn generate_tree_recu(level: usize, counter: &mut i32) -> Option<Box<TreeNode>> {
    if level == 0 {
        return None;
    }
    let mut node = Box::new(TreeNode {
        val: *counter as i32,
        left: None,
        right: None,
    });

    *counter += 1;
    let left_node = self::generate_tree_recu(level - 1, counter);
    let right_node = self::generate_tree_recu(level - 1, counter);
    node.left = left_node;
    node.right = right_node;
    Some(node)
}

fn print_tree_recu(root: &Option<Box<TreeNode>>, level: usize) {
    if let Some(node) = root {
        self::print_tree_recu(&node.right, level + 1);
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", node.val);
        self::print_tree_recu(&node.left, level + 1);
    }
}

// TODO:
fn generate_tree_nonrecu() -> TreeNode {
    todo!();
}

fn invert_tree_nonrecu() -> Option<Box<TreeNode>> {
    todo!();
}

fn print_tree_nonrecu() {
    todo!();
}
