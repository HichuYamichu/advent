pub fn a(input: String) -> String {
    let mut adapters = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    adapters.sort();
    let highest_adapter = adapters.last().unwrap() + 3;
    adapters.push(highest_adapter);

    let mut current_joltage = 0;
    let mut diff1 = 0;
    let mut diff3 = 0;
    for adapter in adapters {
        let differece = adapter - current_joltage;
        current_joltage = adapter;

        if differece == 1 {
            diff1 += 1;
        } else if differece == 3 {
            diff3 += 1;
        }
    }

    (diff1 * diff3).to_string()
}

pub fn b(input: String) -> String {
    let mut adapters = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    adapters.sort();
    adapters.insert(0, 0);
    let highest_adapter = adapters.last().unwrap() + 3;
    adapters.push(highest_adapter);

    let mut combinations: Vec<u64> = Vec::new();
    combinations.push(1);
    for i in 1..adapters.len() {
        combinations.push(0);
        for j in (0..i).rev() {
            if adapters[i] - adapters[j] > 3 {
                break;
            }
            combinations[i] += combinations[j];
        }
    }

    combinations.last().unwrap().to_string()
}
