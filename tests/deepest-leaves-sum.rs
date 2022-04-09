use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

struct NodeWithDepth {
    depth: usize,
    node: Rc<RefCell<TreeNode>>,
}

fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  if let Some(start) = root {
      let mut sums: Vec<i32> = vec![];
      let mut stacks: Vec<NodeWithDepth> = vec![
          NodeWithDepth {
              depth: 0,
              node: start,
          }
      ]; 
      while let Some(
          NodeWithDepth {
              depth,
              node,
          }
      ) = stacks.pop() {
          let val = node.borrow().val;
          if let Some(sum) = sums.get_mut(depth) {
              *sum += val;
          } else {
              sums.push(val);
          }
          if let Some(left) = node.borrow_mut().left.take() {
              stacks.push(
                NodeWithDepth {
                    depth: depth + 1,
                    node: left,
                  }  
              );
          }
          if let Some(right) = node.borrow_mut().right.take() {
              stacks.push(
                NodeWithDepth {
                    depth: depth + 1,
                    node: right,
                  }  
              );
          }
      }
      sums.pop().unwrap()
  } else {
      -1
  }
}