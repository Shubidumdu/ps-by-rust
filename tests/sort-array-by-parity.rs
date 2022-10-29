pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
  let mut result: Vec<i32> = vec![];
  for num in nums.iter() {
    match num {
      n if n % 2 == 1 => {
        result.push(*n);
      },
      n => {
        result.insert(0, *n);
      }
    }
  }
  result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(sort_array_by_parity(vec![3, 1, 2, 4]), vec![4, 2, 3, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(sort_array_by_parity(vec![0]), vec![0]);
    }
}
