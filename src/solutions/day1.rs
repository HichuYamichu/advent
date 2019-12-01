use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn index(f: BufReader<File>) {
    let mut total = 0;
    for line in f.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        let mut res = (mass / 3) - 2;
        total += res;
        while (res / 3) - 2 > 0 {
            res = (res / 3) - 2;
            total += res
        }
    }

    println!("{}", total);
}
