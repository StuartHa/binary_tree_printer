pub struct Node<'a> {
  pub left: Option<&'a Node<'a>>,
  pub right: Option<&'a Node<'a>>,
  pub value: char
}

// Compute the depth of a tree. The minimum depth is 1 for a single-node tree.
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

fn print_tree_rec(node: &Node, level: usize, pos: usize, arr: &mut Vec<Vec<char>>) {
  let tree_depth = depth(&node) as usize;
  arr[level][pos] = node.value;
  if let Some(ref n) = node.left {
    let edge_len = tree_depth - 1;
    for i in 0..edge_len {
      arr[level + i + 1][pos - i - 1] = '/';
    }
    print_tree_rec(&n, level + tree_depth, pos - edge_len, arr);
  }
  if let Some(ref n) = node.right {
    let edge_len = tree_depth - 1;
    for i in 0..edge_len {
      arr[level + i + 1][pos + i + 1] = '\\';
    }
    print_tree_rec(&n, level + tree_depth, pos + edge_len, arr);
  }
}

pub fn print_tree(root: &Node) {
  let tree_depth = depth(&root);
  // Data grid needs to be tree_depth*(tree_depth + 1)/2 rows, and
  // 2^tree_depth - 1 columns.
  let mut data = vec![vec![' '; ((2 as i32).pow(tree_depth) - 1) as usize ]; (tree_depth*(tree_depth+1)/2) as usize];
  print_tree_rec(&root, 0, ((2 as i32).pow(depth(&root) - 1)  - 1) as usize, &mut data);
  for i in 0..data.len() {
    for j in 0..data[i].len() {
      print!("{}", data[i][j]);
    }
    println!("");
  }
}
