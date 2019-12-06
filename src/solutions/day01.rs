pub fn a(_input: &str) -> String {
    format!("aaa")
}

pub fn b(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let mass = line.parse::<i32>().unwrap();
        let mut res = (mass / 3) - 2;
        total += res;
        while (res / 3) - 2 > 0 {
            res = (res / 3) - 2;
            total += res
        }
    }
    total.to_string()
}
