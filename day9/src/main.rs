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

fn find_encryption_weakness(input: &[i64], target: i64) -> i64 {
    let mut start = 0;
    let mut end = 1;

    loop {
        let sum: i64 = input[start..end].iter().sum();

        if sum == target {
            let min: &i64 = input[start..end].iter().min().unwrap();
            let max: &i64 = input[start..end].iter().max().unwrap();
            return *min + *max;
        }

        if sum < target || start == end {
            end += 1;
        }

        if sum > target {
            start += 1;
        }
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let input: Vec<i64> = reader
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();
    let invalid = find_invalid(&input, 25);
    let weakness = find_encryption_weakness(&input, invalid);
    println!("Pt1: {}, Pt2: {}", invalid, weakness);
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

    #[test]
    fn pt2_example() {
        let input: Vec<i64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let invalid = find_invalid(&input, 5);
        let result = find_encryption_weakness(&input, invalid);
        assert_eq!(result, 62);
    }
}
