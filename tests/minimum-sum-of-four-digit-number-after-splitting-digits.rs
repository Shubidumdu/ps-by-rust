fn minimum_sum(num: i32) -> i32 {
  let mut digits: Vec<_> = num.to_string().chars().collect();
  digits.sort();
  let mut iter = digits.into_iter().enumerate();
  let mut new1 = "".to_owned();
  let mut new2 = "".to_owned();
  while let Some((i, digit)) = iter.next() {
    if i % 2 == 0 {
      new1.push(digit);
    } else {
      new2.push(digit);
    }
  }
  new1.parse::<i32>().unwrap() + new2.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(minimum_sum(2932), 52);
  }

  #[test]
  fn example2() {
    assert_eq!(minimum_sum(4009), 13);
  }
}
