fn defang_i_paddr(address: String) -> String {
  address.replace(".", "[.]")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(defang_i_paddr("1.1.1.1".to_owned()), "1[.]1[.]1[.]1".to_owned());
  }

  #[test]
  fn example2() {
    assert_eq!(defang_i_paddr("255.100.50.0".to_owned()), "255[.]100[.]50[.]0".to_owned());
  }
}
