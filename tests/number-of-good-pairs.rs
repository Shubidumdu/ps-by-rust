fn num_identical_pairs(nums: Vec<i32>) -> i32 {
  let mut iter = nums.iter();
  let mut count = 0;
  while let Some(n) = iter.next() {
    count += iter.clone().fold(0, |acc, x| {
      match x == n {
        true => acc + 1,
        false => acc,
      }
    });
  }
  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(num_identical_pairs(vec![1,2,3,1,1,3]), 4);
  }

  #[test]
  fn example2() {
    assert_eq!(num_identical_pairs(vec![1,1,1,1]), 6);
  }

  #[test]
  fn example3() {
    assert_eq!(num_identical_pairs(vec![1,2,3]), 0);
  }
}
