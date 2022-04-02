fn build_array(nums: Vec<i32>) -> Vec<i32> {
  nums.iter().map(|num| nums[*num as usize]).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(build_array(vec![0, 2, 1, 5, 3, 4]), vec![0, 1, 2, 4, 5, 3]);
  }

  #[test]
  fn example2() {
    assert_eq!(build_array(vec![5, 0, 1, 2, 3, 4]), vec![4, 5, 0, 1, 2, 3]);
  }
}
