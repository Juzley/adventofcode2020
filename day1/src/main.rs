use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut entries = Vec::new();
    for line in reader.lines() {
        let entry = line.unwrap().parse::<i64>().unwrap();
        entries.push(entry);
    }

    // Pt 1
    let entries = entries;
    'outer1: for entry1 in &entries {
        for entry2 in &entries {
            if entry1 + entry2 == 2020 {
                println!("Part 1: {}", entry1 * entry2);
                break 'outer1;
            }
        }
    }

    // Pt 2
    let entries_set: HashSet<i64> = HashSet::from_iter(entries.iter().cloned());
    'outer2: for entry1 in &entries {
        for entry2 in &entries {
            let candidate = 2020 - entry1 - entry2;
            if entries_set.contains(&candidate) {
                println!("Part 2: {}", entry1 * entry2 * candidate);
                break 'outer2;
            }
        }
    }
}
