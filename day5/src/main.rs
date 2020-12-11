use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn proc_bsp(bsp: &str, mut min: i32, mut max: i32) -> i32 {
    let mut val = 0;
    for c in bsp.chars() {
        let mid = min + (max - min) / 2;
        match c {
            'F' | 'L' => {
                if max - min == 1 {
                    val = min;
                } else {
                    max = mid;
                }
            }
            'B' | 'R' => {
                if max - min == 1 {
                    val = max;
                } else {
                    min = mid + 1;
                }
            }
            _ => {
                panic!("Unexpected BSP character");
            }
        }
    }

    return val;
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"(?P<row>(F|B){7})(?P<col>(L|R){3})").unwrap();
    let mut ids: Vec<i32> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let row = proc_bsp(&caps["row"], 0, 127);
        let col = proc_bsp(&caps["col"], 0, 7);
        let id = row * 8 + col;
        ids.push(id);
    }

    ids.sort();
    let mut prev_id = 0;
    for id in ids {
        if prev_id != 0 && id != prev_id + 1 {
            break;
        }
        prev_id = id;
    }

    println!("{}", prev_id + 1);
}
