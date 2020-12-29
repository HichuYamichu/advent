#[derive(Debug, Clone, Copy)]
struct Range {
    lower: u32,
    upper: u32,
}

#[derive(Debug, Clone, Copy)]
struct Seat {
    row: Range,
    col: Range,
}

pub fn a(input: String) -> String {
    let max = input
        .lines()
        .map(|l| l.chars())
        .map(|chars| {
            let take_upper = |mut r: &mut Range| r.lower += (r.upper - r.lower) / 2 + 1;
            let take_lower = |mut r: &mut Range| r.upper -= (r.upper - r.lower) / 2 + 1;

            let mut seat = Seat {
                row: Range {
                    lower: 0,
                    upper: 127,
                },
                col: Range { lower: 0, upper: 7 },
            };
            seat = chars.fold(seat, |mut s, c| {
                match c {
                    'F' => take_lower(&mut s.row),
                    'B' => take_upper(&mut s.row),
                    'L' => take_lower(&mut s.col),
                    'R' => take_upper(&mut s.col),
                    _ => unreachable!(),
                };
                s
            });

            8 * seat.row.lower + seat.col.upper
        })
        .max()
        .unwrap();

    format!("{}", max)
}

pub fn b(input: String) -> String {
    let ids = input
        .lines()
        .map(|l| l.chars())
        .map(|chars| {
            let take_upper = |mut r: &mut Range| r.lower += (r.upper - r.lower) / 2 + 1;
            let take_lower = |mut r: &mut Range| r.upper -= (r.upper - r.lower) / 2 + 1;

            let mut seat = Seat {
                row: Range {
                    lower: 0,
                    upper: 127,
                },
                col: Range { lower: 0, upper: 7 },
            };
            seat = chars.fold(seat, |mut s, c| {
                match c {
                    'F' => take_lower(&mut s.row),
                    'B' => take_upper(&mut s.row),
                    'L' => take_lower(&mut s.col),
                    'R' => take_upper(&mut s.col),
                    _ => unreachable!(),
                };
                s
            });

            8 * seat.row.lower + seat.col.upper
        })
        .collect::<Vec<u32>>();

    let max = ids.iter().max().unwrap();
    let seat_id = (1..=*max)
        .find(|id| !ids.contains(&id) && ids.contains(&(id - 1)) && ids.contains(&(id + 1)))
        .unwrap();

    format!("{}", seat_id)
}
