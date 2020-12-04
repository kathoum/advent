use std::io::{BufRead,Cursor};

fn passport_fields(passport_block: &[String]) -> Vec<(String, String)> {
    passport_block.iter().map(|line| {
        line.split_whitespace().map(|field| {
            let mut iter = field.splitn(2, ':');
            let key = iter.next().unwrap().into();
            let value = iter.next().unwrap().into();
            (key, value)
        })
    }).flatten().collect()
}

fn main() {
    let reader = Cursor::new(include_str!("input04.txt"));
    let input: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();
    let blocks = input.split(|line| line == "");
    let passports: Vec<Vec<(String, String)>> = blocks.map(passport_fields).collect();

    println!("Part One");
    let required_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid_passports = passports.iter().filter(|passport| {
        required_fields.iter().all(|key|
            passport.iter().filter(|entry| key == &entry.0).count() == 1
        )
    }).count();
    println!("{} valid passports", valid_passports);

    println!("Part Two");
    let valid_passports = passports.iter().filter(|passport| {
        // same as part one
        let all_present = required_fields.iter().all(|key|
            passport.iter().filter(|entry| key == &entry.0).count() == 1
        );
        // validate
        let is_year_in_range = |value: &str, min, max| {
            value.len() == 4 && match value.parse::<u32>() {
                Ok(year) => year >= min && year <= max,
                _ => false
            }
        };
        let all_valid = passport.iter().all(|(key, value)| {
            match key.as_str() {
                "byr" => is_year_in_range(value, 1920, 2002),
                "iyr" => is_year_in_range(value, 2010, 2020),
                "eyr" => is_year_in_range(value, 2020, 2030),
                "hgt" => {
                    if let Some(hgt) = value.strip_suffix("cm") {
                        match hgt.parse::<u32>() {
                            Ok(hgt) => hgt >= 150 && hgt <= 193,
                            _ => false
                        }
                    } else if let Some(hgt) = value.strip_suffix("in") {
                        match hgt.parse::<u32>() {
                            Ok(hgt) => hgt >= 59 && hgt <= 76,
                            _ => false
                        }
                    } else {
                        false
                    }
                }
                "hcl" => match value.strip_prefix("#") {
                    Some(value) => value.len() == 6 && value.chars().all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()),
                    None => false
                }
                "ecl" => match value.as_str() {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false
                }
                "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
                "cid" => true,
                _ => false
            }
        });
        all_present && all_valid
    }).count();
    println!("{} really valid passports", valid_passports);
}
