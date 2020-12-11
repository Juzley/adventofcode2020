use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut pt1_count = 0;
    let mut pt2_count = 0;
    let mut current_group: HashMap<char, i32> = HashMap::new();
    let mut current_group_size = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            for count in current_group.values() {
                pt1_count += 1;
                if *count == current_group_size {
                    pt2_count += 1;
                }
            }

            current_group_size = 0;
            current_group.clear();
        } else {
            current_group_size += 1;

            for c in line.chars() {
                let count = current_group.entry(c).or_insert(0);
                *count += 1;
            }
        }
    }

    for count in current_group.values() {
        pt1_count += 1;
        if *count == current_group_size {
            pt2_count += 1;
        }
    }

    println!("Pt1 {}, Pt2 {}", pt1_count, pt2_count);
}
