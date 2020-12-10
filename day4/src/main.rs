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

fn is_valid_passport(passport: &HashMap<String, String>, req_keys: &HashSet<String>) -> bool {
    for key in req_keys {
        if !passport.contains_key(key) {
            println!("Missing {}", key);
            return false;
        }
    }
    return true;
}

fn count_valid_passports(passports: &Vec<HashMap<String, String>>) -> i32 {
    let req_keys: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|k| k.to_string())
        .collect();
    let mut valid_count = 0;
    for passport in passports {
        if is_valid_passport(&passport, &req_keys) {
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
