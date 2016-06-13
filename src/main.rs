extern crate binary_tree_visualizer;

use binary_tree_visualizer::node::Node;

fn main() {
  let left_left_node = Node {left: None, right: None, value: 'd'};
  let left_node = Node {left: Some(&left_left_node), right: None, value: 'b'};
  let right_node = Node {left: None, right: None, value: 'c'};
  let root = Node {left: Some(&left_node), right: Some(&right_node), value: 'a'};
  binary_tree_visualizer::print_tree(&root);
}
