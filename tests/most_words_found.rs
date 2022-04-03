fn most_words_found(sentences: Vec<String>) -> i32 {
  sentences.into_iter().fold(0, |acc, s| {
    match s.split(" ").collect::<Vec<&str>>().len() as i32 {
      len if acc < len => len,
      _ => acc,
    }
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    let sentences = vec!["alice and bob love leetcode", "i think so too", "this is great thanks very much"].iter().map(|s| s.to_string()).collect();
  
    assert_eq!(most_words_found(sentences), 6);
  }

  #[test]
  fn example2() {
    let sentences = vec!["please wait", "continue to fight", "continue to win"].iter().map(|s| s.to_string()).collect();
  
    assert_eq!(most_words_found(sentences), 3);
  }
}
