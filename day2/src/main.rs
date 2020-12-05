use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<char>\w): (?P<password>\S+)(\s*)?$").unwrap();

    let mut pt1_correct = 0;
    let mut pt2_correct = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let min = caps.name("min").unwrap().as_str().parse::<usize>().unwrap();
        let max = caps.name("max").unwrap().as_str().parse::<usize>().unwrap();
        let req_char = caps.name("char").unwrap().as_str();
        let password = caps.name("password").unwrap().as_str();
        
        // Part 1
        let count = password.matches(req_char).count();
        if count >= min && count <= max {
            pt1_correct += 1;
        }

        // Part 2
        let first = min - 1;
        let second = max - 1;

        let first_match = &password[first..first+1] == req_char;
        let second_match = &password[second..second+1] == req_char;
        if first_match != second_match {
            pt2_correct += 1;
        }
    }

    println!("Part 1: {}, Part 2: {}", pt1_correct, pt2_correct);
}
