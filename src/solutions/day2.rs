use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

pub fn index(_f: BufReader<File>) {
  a();
  b();
}

fn a() {
  let res = find(12, 2);
  println!("solution A: {:?}", res[0])
}

fn b() {
  for i in 0..99 {
    for j in i..99 {
      let res = find(i, j);
      if res[0] == 19690720 {
        println!("solution B: {}", 100 * res[1] + res[2]);
      }
    }
  }
}

fn find(replace1: usize, replace2: usize) -> Vec<usize> {
  let foo: String = fs::read_to_string("input/day2.txt")
    .unwrap()
    .parse()
    .unwrap();

  let mut program = foo
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
