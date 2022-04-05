fn max_sub_array(nums: Vec<i32>) -> i32 {
  let mut max = nums[0];
  let mut sum = 0;
  let mut iter = nums.iter();

  while let Some(n) = iter.next() {
    sum += *n;

    if sum > 0 {
      max = if sum > max { sum } else { max };
    } else {
      max = if *n > max { *n } else { max };
      sum = 0;
    }
  }

  max
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
  }

  #[test]
  fn example2() {
    assert_eq!(max_sub_array(vec![1]), 1);
  }

  #[test]
  fn example3() {
    assert_eq!(max_sub_array(vec![5,4,-1,7,8]), 23);
  }
}

