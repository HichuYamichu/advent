mod solutions;
use solutions::y2020::*;

type Solution = fn(String) -> String;

pub fn get_solution(day: u8, part: char) -> Option<Solution> {
    struct Pair(Solution, Solution);

    let pair: Pair = match day {
        1 => Pair(day01::a, day01::b),
        2 => Pair(day02::a, day02::b),
        _ => return None,
    };

    match part {
        'a' => Some(pair.0),
        'b' => Some(pair.1),
        _ => None,
    }
}
