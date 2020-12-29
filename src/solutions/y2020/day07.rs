use std::collections::HashMap;

type Rule = String;

pub fn a(input: String) -> String {
    let mut rules: HashMap<Rule, Vec<Rule>> = HashMap::new();
    let lines = input
        .lines()
        .map(|l| l.replace(" bags", "").replace(" bag", "").replace(".", ""));

    for l in lines {
        let mut parts = l.split(" contain ");
        let rule = parts.next().unwrap();
        let entry = rules.entry(rule.to_owned()).or_insert_with(|| Vec::new());

        for r in parts.next().unwrap().split(", ") {
            let mut split = r.splitn(2, " ");
            if let Ok(_) = split.next().unwrap().parse::<u32>() {
                entry.push(split.next().unwrap().to_owned());
            }
        }
    }
    let mut count = 0;
    for bag in rules.keys() {
        let mut found = false;
        traverse_fn(bag, &mut count, &rules, &mut found);
    }

    count.to_string()
}

fn traverse_fn(
    key: &String,
    count: &mut usize,
    rules: &HashMap<Rule, Vec<Rule>>,
    found: &mut bool,
) {
    let bags = rules.get(key).unwrap();
    let k = "shiny gold".to_owned();
    if bags.contains(&k) && !*found {
        *count += 1;
        *found = true;
    } else {
        for bag in bags {
            traverse_fn(&bag, count, &rules, found);
        }
    }
}

pub fn b(input: String) -> String {
    let mut rules: HashMap<Rule, Vec<(u32, Rule)>> = HashMap::new();
    let lines = input
        .lines()
        .map(|l| l.replace(" bags", "").replace(" bag", "").replace(".", ""));

    for l in lines {
        let mut parts = l.split(" contain ");
        let rule = parts.next().unwrap();
        let entry = rules.entry(rule.to_owned()).or_insert_with(|| Vec::new());

        for r in parts.next().unwrap().split(", ") {
            let mut split = r.splitn(2, " ");
            if let Ok(val) = split.next().unwrap().parse::<u32>() {
                entry.push((val, split.next().unwrap().to_owned()));
            }
        }
    }
    let count = count_bags("shiny gold".to_owned(), &rules);

    count.to_string()
}

fn count_bags(bag: String, rules: &HashMap<Rule, Vec<(u32, Rule)>>) -> u32 {
    rules
        .get(&bag)
        .unwrap()
        .iter()
        .map(|(count, rule)| count + count * count_bags(rule.clone(), rules))
        .sum()
}
