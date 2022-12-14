use std::fmt::Display;

type NodeRef<T> = Option<Box<TreeNode<T>>>;

enum Action<T, U> {
    Call(T),
    Handler(U),
}

#[derive(Debug, Clone)]
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

fn main() {
    let tree = self::generate_tree_nonrecu(3);
    print_tree_nonrecu(&tree);
    println!("--------------------------------------");
    let inv_tree = invert_tree_nonrecu(&tree);
    print_tree_nonrecu(&inv_tree);
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

fn generate_tree_nonrecu(level: usize) -> NodeRef<i32> {
    let mut counter = 1;
    let mut arg_stack = Vec::<Action<usize, i32>>::new();
    let mut ret_stack = Vec::<NodeRef<i32>>::new();

    arg_stack.push(Action::Call(level));

    while let Some(action) = arg_stack.pop() {
        match action {
            Action::Call(level) => {
                if level > 0 {
                    arg_stack.push(Action::Handler(counter));
                    counter += 1;
                    arg_stack.push(Action::Call(level - 1)); // right
                    arg_stack.push(Action::Call(level - 1)); // left
                } else {
                    ret_stack.push(None);
                }
            }
            Action::Handler(val) => {
                let right = ret_stack.pop().unwrap();
                let left = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(TreeNode { val, left, right })))
            }
        }
    }

    return ret_stack.pop().unwrap();
}

fn invert_tree_nonrecu<T: Clone>(root: &NodeRef<T>) -> NodeRef<T> {
    let mut arg_stack = Vec::<Action<&NodeRef<T>, &T>>::new();
    let mut ret_stack = Vec::<NodeRef<T>>::new();

    arg_stack.push(Action::Call(root));
    while let Some(action) = arg_stack.pop() {
        match action {
            Action::Call(node) => {
                if let Some(n) = node {
                    arg_stack.push(Action::Handler(&n.val));
                    arg_stack.push(Action::Call(&n.right));
                    arg_stack.push(Action::Call(&n.left));
                } else {
                    ret_stack.push(None);
                }
            }
            Action::Handler(val) => {
                let right = ret_stack.pop().unwrap();
                let left = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(TreeNode {
                    val: val.clone(),
                    left: right,
                    right: left,
                })))
            }
        }
    }

    ret_stack.pop().unwrap()
}

fn print_tree_nonrecu<T: Display>(root: &Option<Box<TreeNode<T>>>) {
    let mut stack = Vec::<Action<(&NodeRef<T>, usize), (&T, usize)>>::new();
    stack.push(Action::Call((&root, 0)));

    while let Some(action) = stack.pop() {
        match action {
            Action::Call((root, level)) => {
                if let Some(node) = root {
                    stack.push(Action::Call((&node.left, level + 1)));
                    stack.push(Action::Handler((&node.val, level)));
                    stack.push(Action::Call((&node.right, level + 1)));
                }
            }
            Action::Handler((val, level)) => {
                for _ in 0..level {
                    print!("  ");
                }
                println!("{}", val);
            }
        }
    }
}
