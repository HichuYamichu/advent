use std::str::FromStr;

pub fn a(input: &str) -> String {
  let res = find(input, 12, 2);
  res[0].to_string()
}

pub fn b(input: &str) -> String {
  let mut sol = 0;
  for i in 0..99 {
    for j in i..99 {
      let res = find(input, i, j);
      if res[0] == 19690720 {
        sol = 100 * res[1] + res[2];
      }
    }
  }
  sol.to_string()
}

fn find(input: &str, replace1: usize, replace2: usize) -> Vec<usize> {
  let mut program = input
    .split(",")
    .map(|s| FromStr::from_str(s).unwrap())
    .collect::<Vec<usize>>();

  program[1] = replace1;
  program[2] = replace2;
  let mut currnet_position = 0;
  loop {
    if program[currnet_position] == 99 {
      break;
    }
    let input_one_position = program[currnet_position + 1];
    let input_two_position = program[currnet_position + 2];
    let save_position = program[currnet_position + 3];
    if program[currnet_position] == 1 {
      program[save_position] = program[input_one_position] + program[input_two_position];
    } else if program[currnet_position] == 2 {
      program[save_position] = program[input_one_position] * program[input_two_position];
    }

    currnet_position += 4;
  }
  return program;
}
