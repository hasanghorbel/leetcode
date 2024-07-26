pub fn reverse_parentheses(s: String) -> String {
  let mut i = 0;
  let mut j = s.len() - 1;
  let mut result = s.chars().collect::<Vec<char>>();
  while i < j {
    if result[i] == '(' {
      while result[j] != ')' {
        j -= 1;
      }

      for k in 1..=(j-i)/2 {
        result.swap(i+k, j-k);

        if result[i+k] == ')' {
          result[i+k] = '(';
        } else if result[i+k] == '(' {
          result[i+k] = ')';
        }

        if result[j-k] == '(' {
          result[j-k] = ')';
        } else if result[j-k] == ')' {
          result[i+k] = '(';
        }
      }
      result.remove(i);
      result.remove(j-1);
      j -= 2;
    } else {
      i += 1;
    }
  }
  
  result.iter().collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let s = "ta()usw((((a))))";
    assert_eq!(reverse_parentheses(s.into()), "tausw)".to_owned())
  }
}
