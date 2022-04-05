fn contains_duplicate(nums: Vec<i32>) -> bool {
  let mut cache: Vec<i32> = vec![];
  let mut iter = nums.iter();

  while let Some(n) = iter.next() {
    if cache.contains(n) {
      return true;
    } else {
      cache.push(*n);
    }
  }

  false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(contains_duplicate(vec![1,2,3,1]), true);
  }

  #[test]
  fn example2() {
    assert_eq!(contains_duplicate(vec![1,2,3,4]), false);
  }

  #[test]
  fn example3() {
    assert_eq!(contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
  }
}

