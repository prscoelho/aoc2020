use std::collections::HashMap;

fn read_passports<'a>(input: &'a str) -> Vec<HashMap<&'a str, &'a str>> {
    let mut passports = Vec::new();

    for groups in input.split("\n\n") {
        let mut passport = HashMap::new();

        for line in groups.split("\n") {
            for word in line.split_ascii_whitespace() {
                let kv = word.split(":").collect::<Vec<_>>();
                if kv.len() == 2 {
                    passport.insert(kv[0], kv[1]);
                }
            }
        }
        passports.push(passport);
    }
    passports
}

const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn has_required_keys(passport: &HashMap<&str, &str>) -> bool {
    REQUIRED_KEYS.iter().all(|key| passport.contains_key(key))
}

fn in_range(num: i32, low: i32, high: i32) -> bool {
    num >= low && num <= high
}

fn valid_byr(byr_unparsed: &str) -> bool {
    if let Ok(val) = byr_unparsed.parse::<i32>() {
        in_range(val, 1920, 2002)
    } else {
        false
    }
}

fn valid_iyr(iyr_unparsed: &str) -> bool {
    if let Ok(val) = iyr_unparsed.parse::<i32>() {
        in_range(val, 2010, 2020)
    } else {
        false
    }
}

fn valid_eyr(eyr_unparsed: &str) -> bool {
    if let Ok(val) = eyr_unparsed.parse::<i32>() {
        in_range(val, 2020, 2030)
    } else {
        false
    }
}

fn valid_hgt(hgt_unparsed: &str) -> bool {
    if let Some(hgt_in_unparsed) = hgt_unparsed.strip_suffix("in") {
        if let Ok(hgt_in) = hgt_in_unparsed.parse::<i32>() {
            return in_range(hgt_in, 59, 76);
        }
    } else if let Some(hgt_cm_unparsed) = hgt_unparsed.strip_suffix("cm") {
        if let Ok(hgt_cm) = hgt_cm_unparsed.parse::<i32>() {
            return in_range(hgt_cm, 150, 193);
        }
    }

    false
}

fn valid_hcl(hcl_unparsed: &str) -> bool {
    if &hcl_unparsed[0..1] == "#" {
        hcl_unparsed[1..].len() == 6 && hcl_unparsed[1..].chars().all(|c| c.is_ascii_hexdigit())
    } else {
        false
    }
}

const VALID_ECLS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn valid_ecl(ecl: &str) -> bool {
    VALID_ECLS.iter().any(|&eye| eye == ecl)
}

fn valid_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
}

fn has_valid_fields(passport: &HashMap<&str, &str>) -> bool {
    valid_byr(passport["byr"])
        && valid_iyr(passport["iyr"])
        && valid_eyr(passport["eyr"])
        && valid_hgt(passport["hgt"])
        && valid_hcl(passport["hcl"])
        && valid_ecl(passport["ecl"])
        && valid_pid(passport["pid"])
}

pub fn part1(input: &str) -> usize {
    let passports = read_passports(input);
    passports.iter().filter(|p| has_required_keys(p)).count()
}

pub fn part2(input: &str) -> usize {
    let passports = read_passports(input);
    passports
        .iter()
        .filter(|p| has_required_keys(p) && has_valid_fields(p))
        .count()
}
