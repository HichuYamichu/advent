use advent::solutions;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    match day {
        1 => solutions::day1::index(load_input(day)),
        2 => solutions::day2::index(load_input(day)),
        _ => setup_day(day),
    };
}

fn load_input(day: usize) -> BufReader<File> {
    let f = File::open(format!("input/day{}.txt", day)).expect("missing input file");
    BufReader::new(f)
}

fn setup_day(day: usize) {
    fs::copy(
        "src/solutions/boilerplate.rs",
        format!("src/solutions/day{}.rs", day),
    )
    .expect("failed to create solution file");

    File::create(format!("input/day{}.txt", day)).expect("failed to create input file");

    let mut solutions = OpenOptions::new()
        .append(true)
        .open("src/solutions.rs")
        .expect("failed to open solutions file");

    let to_write = format!("\npub mod day{};", day);
    solutions
        .write_all(to_write.as_bytes())
        .expect("failed to append solutions file");

    let main = File::open("src/main.rs").expect("failed to open main file");
    let reader = BufReader::new(main);
    let mut line_count = 0;
    let mut first_part = String::new();
    let mut second_part = String::new();
    for line in reader.lines() {
        if line_count < 11 + day {
            first_part = format!(
                "{first_part}{line}\n",
                first_part = first_part,
                line = line.expect("failed read line")
            )
        } else {
            second_part = format!(
                "{second_part}{line}\n",
                second_part = second_part,
                line = line.expect("failed read line")
            )
        }
        line_count += 1;
    }
    let to_write = format!(
        "{first_part}        {day} => solutions::day{day}::index(load_input(day)),\n{second_part}",
        first_part = first_part,
        day = day,
        second_part = second_part
    );
    let mut test = OpenOptions::new()
        .write(true)
        .open("src/main.rs")
        .expect("failed to open main file");

    test.write_all(to_write.as_bytes())
        .expect("failed to modify main file");
}
