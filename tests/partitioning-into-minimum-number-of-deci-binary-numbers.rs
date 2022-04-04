fn min_partitions(n: String) -> i32 {
  n.chars().max().unwrap().to_digit(10).unwrap() as i32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(min_partitions("32".to_owned()), 3);
  }

  #[test]
  fn example2() {
    assert_eq!(min_partitions("82734".to_owned()), 8);
  }

  #[test]
  fn example3() {
    assert_eq!(min_partitions("27346209830709182346".to_owned()), 9);
  }
}
