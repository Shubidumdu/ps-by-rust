fn roman_to_int(s: String) -> i32 {
  let mut chars: Vec<char> = s.chars().collect();
  let mut index = 0;
  let mut result = 0;

  while let Some(c) = chars.get(index) {
    
    let num = match c {
      'I' => {
        match chars.get(index + 1) {
          Some('V') => { index += 1; 4 },
          Some('X') => { index += 1; 9 },
          _ => 1,
        }
      },
      'X' => {
        match chars.get(index + 1) {
          Some('L') => { index += 1; 40 },
          Some('C') => { index += 1; 90 },
          _ => 10,
        }
      },
      'C' => {
        match chars.get(index + 1) {
          Some('D') => { index += 1; 400 },
          Some('M') => { index += 1; 900 },
          _ => 100,
        }
      },
      'V' => 5,
      'L' => 50,
      'D' => 500,
      'M' => 1000,
      _ => 0,
    };

    index += 1;
    result += num;
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(roman_to_int("III".to_owned()), 3);
  }

  #[test]
  fn example2() {
    assert_eq!(roman_to_int("LVIII".to_owned()), 58);
  }

  #[test]
  fn example3() {
    assert_eq!(roman_to_int("MCMXCIV".to_owned()), 1994);
  }
}
