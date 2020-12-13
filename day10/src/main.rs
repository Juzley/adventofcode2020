use std::fs::File;
use std::io::{BufRead, BufReader};

fn pt1(mut adapters: Vec<i32>) -> i32 {
    adapters.sort();
    let mut cur = 0;
    let mut ones = 0;
    let mut threes = 0;
    for a in adapters {
        if a - cur == 1 {
            ones += 1;
        }

        if a - cur == 3 {
            threes += 1;
        }

        cur = a;
    }

    return ones * (threes + 1);
}

fn find_combinations(adapters: &[i32]) -> i64 {
    if adapters.len() <= 1 {
        return 1;
    }

    let mut result = 0;
    let mut cur_idx = 1;
    while cur_idx < adapters.len() {
        if adapters[cur_idx] - adapters[0] <= 3 {
            result += find_combinations(&adapters[cur_idx..])
        } else {
            break;
        }

        cur_idx += 1;
    }

    return result;
}

fn find_run(adapters: &Vec<i32>, mut cur_idx: usize) -> usize {
    cur_idx += 1;
    while cur_idx < adapters.len() {
        if adapters[cur_idx] - adapters[cur_idx - 1] > 2 {
            break;
        }

        cur_idx += 1;
    }

    return cur_idx;
}

fn pt2(mut adapters: Vec<i32>) -> i64 {
    adapters.push(0);
    adapters.sort();
    let mut cur_idx = 0;
    let mut combinations: i64 = 1;

    while cur_idx < adapters.len() {
        let next_idx = find_run(&adapters, cur_idx);
        println!("Found run {:?}", &adapters[cur_idx..next_idx]);
        combinations *= find_combinations(&adapters[cur_idx..next_idx]);
        cur_idx = next_idx;
    }

    return combinations;
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let adapters: Vec<i32> = reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();
    let pt1 = pt1(adapters.clone());
    let pt2 = pt2(adapters);
    println!("Pt 1: {}, Pt2: {}", pt1, pt2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_example1() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let result = pt1(adapters);
        assert_eq!(result, 7 * 5);
    }

    #[test]
    fn pt1_example2() {
        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let result = pt1(adapters);
        assert_eq!(result, 22 * 10);
    }

    #[test]
    fn pt2_example1() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let result = pt2(adapters);
        assert_eq!(result, 8);
    }

    #[test]
    fn pt2_example2() {
        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let result = pt2(adapters);
        assert_eq!(result, 19208);
    }
}
