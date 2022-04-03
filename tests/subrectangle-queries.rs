struct SubrectangleQueries {
  rectangle: Vec<Vec<i32>>
}

impl SubrectangleQueries {

  fn new(rectangle: Vec<Vec<i32>>) -> Self {
      SubrectangleQueries {
        rectangle
      }
  }
  
  fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
      for r in row1..row2 + 1 {
        for c in col1..col2 + 1 {
          self.rectangle[r as usize][c as usize] = new_value;
        }
      }
  }
  
  fn get_value(&self, row: i32, col: i32) -> i32 {
    self.rectangle[row as usize][col as usize]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    let mut queries = SubrectangleQueries::new(vec![
      vec![1, 2, 1],
      vec![4, 3, 4],
      vec![3, 2, 1],
      vec![1, 1, 1],
    ]);
    assert_eq!(queries.get_value(0, 2), 1);
    queries.update_subrectangle(0, 0, 3, 2, 5);
    assert_eq!(queries.get_value(0, 2), 5);
    assert_eq!(queries.get_value(3, 1), 5);
    queries.update_subrectangle(3, 0, 3, 2, 10);
    assert_eq!(queries.get_value(3, 1), 10);
    assert_eq!(queries.get_value(0, 2), 5);
  }

  #[test]
  fn example2() {
    let mut queries = SubrectangleQueries::new(vec![
      vec![1, 1, 1],
      vec![2, 2, 2],
      vec![3, 3, 3],
    ]);
    assert_eq!(queries.get_value(0, 0), 1);
    queries.update_subrectangle(0, 0, 2, 2, 100);
    assert_eq!(queries.get_value(0, 0), 100);
    assert_eq!(queries.get_value(2, 2), 100);
    queries.update_subrectangle(1, 1, 2, 2, 20);
    assert_eq!(queries.get_value(2, 2), 20);
  }
}
