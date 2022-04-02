fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
  nums.repeat(2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(get_concatenation(vec![1, 2, 1]), vec![1, 2, 1, 1, 2, 1]);
  }

  #[test]
  fn example2() {
    assert_eq!(get_concatenation(vec![1, 3, 2, 1]), vec![1, 3, 2, 1, 1, 3, 2, 1]);
  }
}