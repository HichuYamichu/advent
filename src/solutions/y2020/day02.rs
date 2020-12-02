struct Entry<'a> {
    position1: u32,
    position2: u32,
    letter: char,
    pass: &'a str,
}

pub fn a(input: String) -> String {
    let res = input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut i| {
            let mut split = i.next().unwrap().split('-');

            Entry {
                position1: split.next().unwrap().parse().unwrap(),
                position2: split.next().unwrap().parse().unwrap(),
                letter: i.next().unwrap().chars().next().unwrap(),
                pass: i.next().unwrap(),
            }
        })
        .map(|entry| {
            let count = entry.pass.matches(entry.letter).count();
            count >= entry.position1 as usize && count <= entry.position2 as usize
        })
        .filter(|p| *p)
        .count();

    format!("{}", res)
}

pub fn b(input: String) -> String {
    let res = input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut i| {
            let mut split = i.next().unwrap().split('-');

            Entry {
                position1: split.next().unwrap().parse().unwrap(),
                position2: split.next().unwrap().parse().unwrap(),
                letter: i.next().unwrap().chars().next().unwrap(),
                pass: i.next().unwrap(),
            }
        })
        .map(|entry| {
            let mut chars = entry.pass.chars();
            let first = chars.nth((entry.position1 - 1) as usize);
            let second = chars.nth((entry.position2 - entry.position1 - 1) as usize);
            first.unwrap() == entry.letter && second.unwrap() != entry.letter
                || second.unwrap() == entry.letter && first.unwrap() != entry.letter
        })
        .filter(|p| *p)
        .count();

    format!("{}", res)
}
