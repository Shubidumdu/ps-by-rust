fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
  accounts.into_iter().map(|account| account.into_iter().sum()).max().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6);
  }

  #[test]
  fn example2() {
    assert_eq!(maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]), 10);
  }

  #[test]
  fn example3() {
    assert_eq!(maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]), 17);
  }
}
