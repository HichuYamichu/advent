mod solutions;
use solutions::y2020::*;

type Solution = fn(String) -> String;

pub fn get_solution(day: u8, part: char) -> Option<Solution> {
    struct Pair(Solution, Solution);

    let pair: Pair = match day {
        1 => Pair(day01::a, day01::b),
        2 => Pair(day02::a, day02::b),
        3 => Pair(day03::a, day03::b),
        4 => Pair(day04::a, day04::b),
        5 => Pair(day05::a, day05::b),
        6 => Pair(day06::a, day06::b),
        7 => Pair(day07::a, day07::b),
        8 => Pair(day08::a, day08::b),
        9 => Pair(day09::a, day09::b),
        10 => Pair(day10::a, day10::b),
        _ => return None,
    };

    match part {
        'a' => Some(pair.0),
        'b' => Some(pair.1),
        _ => None,
    }
}
