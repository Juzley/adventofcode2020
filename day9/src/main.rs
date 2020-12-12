use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn find_invalid(input: &[i64], window_size: usize) -> i64 {
    for idx in window_size..input.len() {
        let candidate = input[idx];
        let window_start = idx - window_size;
        let window_end = idx;
        let window = &input[window_start..window_end];
        let window_set: HashSet<&i64> = HashSet::from_iter(window.iter());

        let mut found = false;
        for w in window {
            if window_set.contains(&(candidate - w)) {
                found = true;
                break;
            }
        }

        if !found {
            return candidate;
        }
    }

    panic!("Didn't find invalid number!");
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let input: Vec<i64> = reader
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();
    let result = find_invalid(&input, 25);
    println!("Pt1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_example() {
        let input: Vec<i64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let result = find_invalid(&input, 5);
        assert_eq!(result, 127);
    }
}
