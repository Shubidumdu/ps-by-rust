fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
  let mut start = 0;
  let mut end = nums.len() - 1;
  let mut index = nums.len() - 1;
  let mut result = vec![0; nums.len()];

  while start <= end {
    let left_square = nums[start].pow(2);
    let right_square = nums[end].pow(2);

    if (start == end) {
      result[index] = left_square;
      break;
    }

    if (left_square < right_square) {
      result[index] = right_square;
      end -= 1;
      index -= 1;
    } else {
      result[index] = left_square;
      start += 1;
      index -= 1;
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(sorted_squares(vec![-4, -1, 0, 3, 10]), vec![0, 1, 9, 16, 100]);
  }

  #[test]
  fn example2() {
    assert_eq!(sorted_squares(vec![-7, -3, 2, 3, 11]), vec![4, 9, 9, 49, 121]);
  }
}
