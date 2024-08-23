pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
  let mut current_time: u64 = 0;
  let mut average: u64 = 0;
  for i in 0..customers.len() {
      let arrival = customers[i][0] as u64;
      let time = customers[i][1] as u64;

      if current_time >=  arrival {
        current_time += time;
        average += current_time - arrival;
      } else {
        current_time = arrival + time;
        average += time;
      }
  }
  average as f64 / customers.len() as f64
}

fn main() {
  use std::time::Instant;
  let customers = [[5,2],[5,4],[10,3],[20,1]].map(|v| v.to_vec()).to_vec();
  let now = Instant::now();
  let result = average_waiting_time(customers);
  let elapsed = now.elapsed();
  println!("{}", result);
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let customers = [[5,2],[5,4],[10,3],[20,1]].map(|v| v.to_vec()).to_vec();
    assert_eq!(average_waiting_time(customers), 3.25)
  }
}
