pub fn a(input: String) -> String {
    let mut lines = input.lines().map(|l| l.parse::<i64>().unwrap());
    let mut res = 0;
    while let Some(cursor) = lines.next() {
        for next in lines.clone() {
            if cursor + next == 2020 {
                res = cursor * next;
            }
        }
    }

    format!("{}", res)
}

pub fn b(input: String) -> String {
    let mut lines = input.lines().map(|l| l.parse::<i64>().unwrap());
    let mut res = 0;
    while let Some(cursor) = lines.next() {
        for next in lines.clone() {
            for next2 in lines.clone() {
                if cursor + next + next2 == 2020 {
                    res = cursor * next * next2;
                }
            }
        }
    }

    format!("{}", res)
}
