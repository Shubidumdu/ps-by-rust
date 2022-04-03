fn final_value_after_operations(operations: Vec<String>) -> i32 {
  operations.into_iter().fold(0, |acc, x| {
    match x.as_str() {
      "--X" | "X--" => acc - 1,
      "++X" | "X++" => acc + 1,
      _ => acc,
    }
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(final_value_after_operations(vec!["--X".to_string(),"X++".to_string(),"X++".to_string()]), 1);
  }

  #[test]
  fn example2() {
    assert_eq!(final_value_after_operations(vec!["++X".to_string(),"++X".to_string(),"X++".to_string()]), 3);
  }

  #[test]
  fn example3() {
    assert_eq!(final_value_after_operations(vec!["X++".to_string(),"++X".to_string(),"--X".to_string(),"X--".to_string()]), 0);
  }
}
