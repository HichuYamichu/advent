pub fn a(input: &str) -> String {
  let (start, end) = parse_input(input);

  let mut viable_passwords = 0;
  'outer: for i in start..=end {
    let digits = number_to_vec(i);
    let mut has_same_digits = false;
    for (index, digit) in digits.iter().enumerate() {
      if index != 0 {
        if digits[index - 1] > *digit {
          continue 'outer;
        } else if digits[index - 1] == *digit {
          has_same_digits = true;
        }
      }
    }
    if has_same_digits {
      viable_passwords += 1;
    }
  }

  viable_passwords.to_string()
}

pub fn b(input: &str) -> String {
  let (start, end) = parse_input(input);

  let mut viable_passwords = 0;
  'outer: for i in start..=end {
    let digits = number_to_vec(i);
    let mut same_digit_count = 1;
    let mut has_same_digits = false;
    for (index, digit) in digits.iter().enumerate() {
      if index != 0 {
        if digits[index - 1] > *digit {
          continue 'outer;
        } else if digits[index - 1] == *digit {
          same_digit_count += 1
        } else {
          if same_digit_count == 2 {
            has_same_digits = true;
          }
          same_digit_count = 1;
        }
      }
    }
    if same_digit_count == 2 {
      has_same_digits = true;
    }
    if has_same_digits {
      viable_passwords += 1;
    }
  }

  viable_passwords.to_string()
}

fn parse_input(input: &str) -> (u32, u32) {
  let mut parts = input.trim().split('-');
  let start = parts.next().unwrap().parse::<u32>().unwrap();
  let end = parts.next().unwrap().parse::<u32>().unwrap();
  (start, end)
}

fn number_to_vec(n: u32) -> Vec<u32> {
  n.to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect()
}
