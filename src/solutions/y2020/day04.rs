pub fn a(input: String) -> String {
    let count = input
        .split("\n\n")
        .filter(|pass| {
            pass.contains("byr:")
                && pass.contains("iyr:")
                && pass.contains("eyr:")
                && pass.contains("hgt:")
                && pass.contains("hcl:")
                && pass.contains("ecl:")
                && pass.contains("pid:")
        })
        .count();

    format!("{}", count)
}

pub fn b(input: String) -> String {
    let count = input
        .split("\n\n")
        .map(|p| p.split_ascii_whitespace().collect::<Vec<_>>())
        .filter_map(|pass| {
            let find_and_map = |search: &str| -> Option<&str> {
                pass.iter()
                    .find(|field| field.starts_with(search))?
                    .get(4..)
            };

            find_and_map("byr:").filter(|byr| *byr >= "1920" && *byr <= "2002")?;
            find_and_map("iyr:").filter(|iyr| *iyr >= "2010" && *iyr <= "2020")?;
            find_and_map("eyr:").filter(|eyr| *eyr >= "2020" && *eyr <= "2030")?;

            let hgt = find_and_map("hgt:")?;
            let h = hgt.get(0..hgt.len() - 2)?;
            if hgt.ends_with("cm") {
                if h < "150" || h > "193" {
                    return None;
                }
            } else if hgt.ends_with("in") {
                if h < "59" || h > "76" {
                    return None;
                }
            } else {
                return None;
            }

            let mut hcl = find_and_map("hcl:")?.chars();
            let pound = hcl.next()?;
            if pound != '#' {
                return None;
            }
            let counter = hcl
                .filter(|c| !('a'..='f').contains(&c) || !('0'..='9').contains(&c))
                .count();
            if counter != 6 {
                return None;
            }

            let ecl = find_and_map("ecl:")?;
            if !&["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl) {
                return None;
            }

            let pid = find_and_map("pid:")?;
            let digits = pid.chars().filter(|c| c.is_digit(10)).count();
            if digits != 9 {
                return None;
            }

            Some(())
        })
        .count();

    format!("{}", count)
}
