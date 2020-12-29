use std::collections::{HashMap, HashSet};

pub fn a(input: String) -> String {
    input
        .split("\n\n")
        .map(|g| {
            let mut set = HashSet::new();
            g.chars()
                .filter(|c| *c != '\n')
                .fold(&mut set, |counter, answ| {
                    counter.insert(answ);
                    counter
                });
            set.iter().count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn b(input: String) -> String {
    input
        .split("\n\n")
        .map(|group| {
            let mut answers = HashMap::new();
            let lines = group.lines().collect::<Vec<_>>();
            let expected = lines.len();
            group.lines().fold(&mut answers, |mut answers, l| {
                l.chars().fold(&mut answers, |answers, c| {
                    *answers.entry(c).or_insert(0) += 1;
                    answers
                });
                answers
            });

            answers.values().filter(|count| **count == expected).count()
        })
        .sum::<usize>()
        .to_string()
}
