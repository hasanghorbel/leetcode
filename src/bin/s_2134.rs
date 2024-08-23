pub fn min_swaps(nums: Vec<i32>) -> i32 {
  // ones count in vec
  let mut ones = 0;
  for i in 0..nums.len() {
    if nums[i] == 1 {
      ones += 1;
    }
  }

  // initial window
  let mut init = 0;
  for i in 0..ones {
    if nums[i] == 0 {
      init += 1;
    }
  }

  // sliding window
  let mut result = init;

  let mut l = 0;
  let mut r = ones;
  while l < nums.len()-ones {
    if nums[l] == 0 {
      init -= 1; 
    }
    if nums[r] == 0 {
      init += 1;
    }
    l += 1;
    r += 1;

    if init < result {
      result = init;
    }
  }

  // remaining windows
  r = 0;
  while l < nums.len() {
    if nums[l] == 0 {
      init -= 1; 
    }
    if nums[r] == 0 {
      init += 1;
    }
    l += 1;
    r += 1;

    if init < result {
      result = init;
    }
  }
  result 
}

fn main() {
  use std::time::Instant;
  let nums = vec![0,1,0,1,1,0,0];
  let now = Instant::now();
  let result = min_swaps(nums);
  let elapsed = now.elapsed();
  println!("{}", result);
  println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let nums = vec![0,1,0,1,1,0,0];

    assert_eq!(min_swaps(nums), 1);
  }

  #[test]
  fn test_2() {
    let nums = vec![0,1,1,1,0,0,1,1,0];

    assert_eq!(min_swaps(nums), 2);
  }

  #[test]
  fn test_3() {
    let nums = vec![1,1,0,0,1];

    assert_eq!(min_swaps(nums), 0);
  }

}
