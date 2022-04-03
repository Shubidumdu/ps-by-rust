fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
  let mut result = vec![];
  
  for i in 0..n {
    result.push(nums[i as usize]);
    result.push(nums[(n + i) as usize]);
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(shuffle(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
  }

  #[test]
  fn example2() {
    assert_eq!(shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4), vec![1, 4, 2, 3, 3, 2, 4, 1]);
  }

  #[test]
  fn example3() {
    assert_eq!(shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
  }
}
