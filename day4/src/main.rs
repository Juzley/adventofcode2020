use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_passports() -> Vec<HashMap<String, String>> {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"(?P<key>\S+):(?P<value>\S+)(\s+|$)").unwrap();

    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut current_passport: HashMap<String, String> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            passports.push(current_passport.clone());
            current_passport.clear();
        } else {
            for caps in re.captures_iter(&line) {
                current_passport.insert(caps["key"].to_string(), caps["value"].to_string());
            }
        }
    }

    if !current_passport.is_empty() {
        passports.push(current_passport);
    }

    return passports;
}

fn is_year_valid(passport: &HashMap<String, String>, key: &str, min: i32, max: i32) -> bool {
    if !passport.contains_key(key) {
        return false;
    }

    let parse_result = passport[key].parse::<i32>();
    if parse_result.is_err() {
        return false;
    }

    let value = parse_result.unwrap();
    if value < min || value > max {
        return false;
    }

    return true;
}

fn is_height_valid(passport: &HashMap<String, String>) -> bool {
    if !passport.contains_key("hgt") {
        return false;
    }

    let re = Regex::new(r"^(?P<value>\d+)(?P<units>in|cm)$").unwrap();
    let caps = re.captures(&passport["hgt"]);
    if caps.is_none() {
        return false;
    }
    let caps = caps.unwrap();

    let parse_result = caps["value"].parse::<i32>();
    if parse_result.is_err() {
        return false;
    }
    let value = parse_result.unwrap();
    let units = &caps["units"];
    if units == "in" {
        if value < 59 || value > 76 {
            return false;
        }
        return true;
    } else if units == "cm" {
        if value < 150 || value > 193 {
            return false;
        }
        return true;
    }

    return false;
}

fn is_hair_colour_valid(passport: &HashMap<String, String>) -> bool {
    if !passport.contains_key("hcl") {
        return false;
    }

    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    if re.is_match(&passport["hcl"]) {
        return true;
    }
    return false;
}

fn is_eye_colour_valid(passport: &HashMap<String, String>) -> bool {
    if !passport.contains_key("ecl") {
        return false;
    }

    let allowed: HashSet<String> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .map(|k| k.to_string())
        .collect();
    if allowed.contains(&passport["ecl"]) {
        return true;
    }

    return false;
}

fn is_pid_valid(passport: &HashMap<String, String>) -> bool {
    if !passport.contains_key("pid") {
        return false;
    }

    let re = Regex::new("^[0-9]{9}$").unwrap();
    if re.is_match(&passport["pid"]) {
        return true;
    }

    return false;
}

fn is_valid_passport(passport: &HashMap<String, String>) -> bool {
    if !is_year_valid(passport, "byr", 1920, 2002) {
        return false;
    }

    if !is_year_valid(passport, "iyr", 2010, 2020) {
        return false;
    }

    if !is_year_valid(passport, "eyr", 2020, 2030) {
        return false;
    }

    if !is_height_valid(passport) {
        return false;
    }

    if !is_hair_colour_valid(passport) {
        return false;
    }

    if !is_eye_colour_valid(passport) {
        return false;
    }

    if !is_pid_valid(passport) {
        return false;
    }

    return true;
}

fn count_valid_passports(passports: &Vec<HashMap<String, String>>) -> i32 {
    let mut valid_count = 0;
    for passport in passports {
        if is_valid_passport(&passport) {
            valid_count += 1;
        }
    }

    return valid_count;
}

fn main() {
    let passports = read_passports();
    let valid_count = count_valid_passports(&passports);
    println!("{} of {} valid", valid_count, passports.len());
}
