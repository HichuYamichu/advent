pub fn a(input: String) -> String {
    let mut grid = input.lines().map(|l| l.chars().cycle());
    let mut trees = 0;
    grid.next();
    let mut y_axis = 3;
    for mut line in grid {
        let c = line.nth(y_axis).unwrap();
        if c == '#' {
            trees += 1;
        }
        y_axis += 3;
    }

    format!("{}", trees)
}

struct Slope {
    right: usize,
    down: usize,
}

pub fn b(input: String) -> String {
    let slopes = &[
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    slopes
        .iter()
        .map(|s| {
            let grid = input.lines().map(|l| l.chars().cycle()).skip(s.down);
            let mut y_axis = (s.right..).step_by(s.right);
            grid.step_by(s.down)
                .map(|mut line| line.nth(y_axis.next().unwrap()).unwrap())
                .filter(|c| *c == '#')
                .count()
        })
        .product::<usize>()
        .to_string()
}
