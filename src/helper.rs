// find position of one of @dlims.
pub fn strpbrk(msg: &String, delims: &str) -> Option<usize> {
  for (ix, c) in msg.chars().enumerate() {
    if let Some(_) = delims.find(c) {
      return Some(ix);
    }
  }
  None
}

// find all positions of one of @dlims.
pub fn strpbrk_all(msg: &String, delims: &str) -> Vec<usize> {
  let mut ret = vec![];
  for (ix, c) in msg.chars().enumerate() {
    if let Some(p) = delims.find(c) {
      ret.push(ix);
    }
  }
  ret
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_strpbrk_all() {
    let s = String::from("012,45:7:,");
    assert_eq!(super::strpbrk_all(&s, ",:"), vec![3, 6, 8, 9]);
  }
}
