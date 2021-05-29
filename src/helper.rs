pub fn strpbrk(msg: &String, delims: &String) -> Option<usize> {
  for (ix, c) in msg.chars().enumerate() {
    if let Some(_) = delims.find(c) {
      return Some(ix);
    }
  }
  None
}