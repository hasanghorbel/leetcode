fn min_operations(logs: Vec<String>) -> i32 {
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

fn main() {
  use std::time::Instant;
  let logs = ["d1/","../","../","../"].map(|s| s.to_string()).to_vec();
  let now = Instant::now();
  let result = min_operations(logs);
  let elapsed = now.elapsed();
  println!("{}", result);
  println!("elapsed: {:?}", elapsed);
}