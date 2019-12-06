use advent::solutions;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<u8>().unwrap();
    let problem = args[2].parse::<char>().unwrap();
    if let Some(solver) = solutions::get_solver(day, problem) {
        let mut input = String::new();
        File::open(format!("input/day{}.txt", day))
            .expect("Error missing input file")
            .read_to_string(&mut input)
            .expect("Error reading input file");

        let solution = solver(input.as_ref());
        println!("{}", solution);
    } else {
        unimplemented!()
    }
}
