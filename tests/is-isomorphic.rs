use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
  let mut s_map = HashMap::<char, char>::new();
  let mut t_map = HashMap::<char, char>::new();
  let mut s_chars = s.chars();
  let mut t_chars = t.chars();

  for i in 0..s.len() {
    match (s_chars.next(), t_chars.next()) {
      (Some(s_char), Some(t_char)) => {
        match s_map.get(&s_char) {
          Some(mapped) => {
            if *mapped != t_char {
              return false;
            }
          },
          None => {
            s_map.insert(s_char, t_char);
          }
        }
        match t_map.get(&t_char) {
          Some(mapped) => {
            if *mapped != s_char {
              return false;
            }
          },
          None => {
            t_map.insert(t_char, s_char);
          }
        }
      },
      (_, _) => ()
    }
  }
  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(is_isomorphic("egg".to_string(), "add".to_string()), true);
  }

  #[test]
  fn example2() {
    assert_eq!(is_isomorphic("foo".to_string(), "bar".to_string()), false);
  }

  #[test]
  fn example3() {
    assert_eq!(is_isomorphic("paper".to_string(), "title".to_string()), true);
  }

  #[test]
  fn example4() {
    assert_eq!(is_isomorphic("badc".to_string(), "baba".to_string()), false);
  }
}