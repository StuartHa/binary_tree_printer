pub struct Node<'a> {
  pub left: Option<&'a Node<'a>>,
  pub right: Option<&'a Node<'a>>,
  pub value: char
}


