fn search(nums: Vec<i32>, target: i32) -> i32 {
  let mut start: i32 = 0;
  let mut end: i32 = nums.len() as i32 - 1;

  let result = loop {
    let mid = (start + end) / 2;

    let num = nums[mid as usize];

    if num == target {
      break mid;
    }

    if num > target {
      end = mid - 1;
    } else {
      start = mid + 1;
    }

    if start > end {
      break -1;
    }
  };

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(search(vec![-1,0,3,5,9,12], 9), 4);
  }

  #[test]
  fn example2() {
    assert_eq!(search(vec![-1,0,3,5,9,12], 2), -1);
  }
}
