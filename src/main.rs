use std::str;

struct Node<'a> {
  left: Option<&'a Node<'a>>,
  right: Option<&'a Node<'a>>,
  value: char
}

fn depth(node: &Node) -> u32 {
  let mut max_depth = 1;
  if let Some(ref n) = node.left {
    max_depth = 1 + depth(&n);
  }
  if let Some(ref n) = node.right {
    let right_depth = 1 + depth(&n);
    if right_depth > max_depth {
      max_depth = right_depth;
    }
  }
  max_depth
}

fn print_tree(node: &Node, level: usize, pos: usize, arr: &mut [[char; 60]; 60]) {
  let tree_depth = depth(&node) as usize;
  arr[level][pos] = node.value;
  if let Some(ref n) = node.left {
    // Print the edge to node.
    let edge_len = tree_depth - 1;
    for i in 0..edge_len {
      arr[level + i + 1][pos - i - 1] = '/';
    }
    print_tree(&n, level + tree_depth, pos - edge_len, arr);
  }
  if let Some(ref n) = node.right {
    let edge_len = tree_depth - 1;
    for i in 0..edge_len {
      arr[level + i + 1][pos + i + 1] = '\\';
    }
    print_tree(&n, level + tree_depth, pos + edge_len, arr);
  }
}

fn main() {
  let left_left_node = Node {left: None, right: None, value: 'd'};
  let left_node = Node {left: Some(&left_left_node), right: None, value: 'b'};
  let right_node = Node {left: None, right: None, value: 'c'};
  let root = Node {left: Some(&left_node), right: Some(&right_node), value: 'a'};
  let mut arr = [[' '; 60]; 60];
  print_tree(&root, 0, ((2 as i32).pow(depth(&root) - 1)  - 1) as usize, &mut arr);
  for i in 0..60 {
    for j in 0..60 {
      print!("{}", arr[i][j]);
    }
    println!("");
  }
}
