use std::fmt::Display;

#[derive(Debug, Clone)]
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

fn main() {
    let tree = self::generate_tree_recu(5, &mut 0);
    print_tree_recu(&tree, 0);
    println!("--------------------------------------");
    let inv_tree = self::invert_tree_recu(tree);
    print_tree_recu(&inv_tree, 0);
}

fn invert_tree_recu<T>(root: Option<Box<TreeNode<T>>>) -> Option<Box<TreeNode<T>>> {
    if let Some(node) = root {
        return Some(Box::new(TreeNode {
            val: node.val,
            left: self::invert_tree_recu(node.right),
            right: self::invert_tree_recu(node.left),
        }));
    } else {
        return None;
    }
}

fn generate_tree_recu(level: usize, counter: &mut i32) -> Option<Box<TreeNode<i32>>> {
    if level == 0 {
        return None;
    }
    let mut node = Box::new(TreeNode {
        val: *counter as i32,
        left: None,
        right: None,
    });

    *counter += 1;
    node.left = self::generate_tree_recu(level - 1, counter);
    node.right = self::generate_tree_recu(level - 1, counter);
    Some(node)
}

fn print_tree_recu<T: Display>(root: &Option<Box<TreeNode<T>>>, level: usize) {
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
fn generate_tree_nonrecu<T>() -> TreeNode<T> {
    todo!();
}

fn invert_tree_nonrecu<T>() -> Option<Box<TreeNode<T>>> {
    todo!();
}

fn print_tree_nonrecu() {
    todo!();
}
