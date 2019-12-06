pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn get_solver(day: u8, problem: char) -> Option<fn(&str) -> String> {
  struct Solver(fn(&str) -> String, fn(&str) -> String);

  let solutions: Solver = match day {
    1 => Solver(day01::a, day01::b),
    2 => Solver(day02::a, day02::b),
    3 => Solver(day03::a, day03::b),
    4 => Solver(day04::a, day04::b),
    5 => Solver(day05::a, day05::b),
    6 => Solver(day06::a, day06::b),
    7 => Solver(day07::a, day07::b),
    8 => Solver(day08::a, day08::b),
    9 => Solver(day09::a, day09::b),
    10 => Solver(day10::a, day10::b),
    11 => Solver(day11::a, day11::b),
    12 => Solver(day12::a, day12::b),
    13 => Solver(day13::a, day13::b),
    14 => Solver(day14::a, day14::b),
    15 => Solver(day15::a, day15::b),
    16 => Solver(day16::a, day16::b),
    17 => Solver(day17::a, day17::b),
    18 => Solver(day18::a, day18::b),
    19 => Solver(day19::a, day19::b),
    20 => Solver(day20::a, day20::b),
    21 => Solver(day21::a, day21::b),
    22 => Solver(day22::a, day22::b),
    23 => Solver(day23::a, day23::b),
    24 => Solver(day24::a, day24::b),
    25 => Solver(day25::a, day25::b),
    _ => {
      return None;
    }
  };

  match problem {
    'a' => Some(solutions.0),
    'b' => Some(solutions.1),
    _ => None,
  }
}
