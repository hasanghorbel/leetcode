pub fn min_operations(logs: Vec<String>) -> i32 {
  let mut output = 0;
  for log in logs.iter() {
    match log.as_str() {
      "../" => if output > 0 { output -= 1; },
      "./" => continue,
      _ => output += 1,
    }
  }
  output
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let logs = ["d1/","../","../","../"].map(|s| s.to_string()).to_vec();
    assert_eq!(min_operations(logs), 0)
  }
}


