const PREAMBLE_SIZE: usize = 25;

pub fn a(input: String) -> String {
    let sq = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<u64>>();

    for (i, next) in sq[PREAMBLE_SIZE..].iter().enumerate() {
        let parts = &sq[i..i + PREAMBLE_SIZE];
        let mut is_made_form_parts = false;
        for (j, num1) in parts.iter().enumerate() {
            for num2 in &parts[j..] {
                if num1 + num2 == *next {
                    is_made_form_parts = true;
                }
            }
        }

        if !is_made_form_parts {
            return next.to_string();
        }
    }
    "not found".to_owned()
}

pub fn b(input: String) -> String {
    let sq = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<u64>>();

    let mut invalid = 0;
    for (i, next) in sq[PREAMBLE_SIZE..].iter().enumerate() {
        let parts = &sq[i..i + PREAMBLE_SIZE];
        let mut is_made_form_parts = false;
        for (j, num1) in parts.iter().enumerate() {
            for num2 in &parts[j..] {
                if num1 + num2 == *next {
                    is_made_form_parts = true;
                }
            }
        }

        if !is_made_form_parts {
            invalid = *next;
        }
    }

    for (i, num1) in sq.iter().enumerate() {
        let mut sum = *num1;
        for (j, num2) in sq[i + 1..].iter().enumerate() {
            sum += num2;

            if sum > invalid {
                break;
            }
            if sum == invalid {
                let mut range = sq[i..=j + i + 1].iter().collect::<Vec<_>>();
                range.sort();
                let res = range[0] + *range.last().unwrap();
                return res.to_string();
            }
        }
    }
    "not found".to_owned()
}
