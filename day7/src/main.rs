use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn pt1(lines: &[String]) -> usize {
    let top_re = Regex::new(r"^(?P<colour>.*) bags contain (?P<remainder>.*)$").unwrap();
    let sub_re = Regex::new(r"\s?(?P<count>\d+) (?P<colour>[^,\.]*) bags?(,|\.)").unwrap();

    let mut bags_in: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let top_caps = top_re.captures(&line).unwrap();
        let container_color = top_caps["colour"].to_string();
        let remainder = &top_caps["remainder"];

        println!("{}", remainder);

        for caps in sub_re.captures_iter(remainder) {
            let contained_color = caps["colour"].to_string();
            let entry = bags_in.entry(contained_color).or_insert(vec![]);
            entry.push(container_color.clone());
        }
    }

    println!("{:?}", bags_in);

    let bags_in = bags_in;
    let mut queue: VecDeque<String> = bags_in["shiny gold"].iter().cloned().collect();
    let mut visited: HashSet<String> = queue.iter().cloned().collect();
    let mut count = queue.len();
    while !queue.is_empty() {
        let bag = queue.pop_front().unwrap();

        if !bags_in.contains_key(&bag) {
            continue;
        }

        for b in &bags_in[&bag] {
            if !visited.contains(b) {
                queue.push_back(b.clone());
                visited.insert(b.clone());
                count += 1;
            }
        }
    }

    return count;
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let pt1_result = pt1(&lines);
    println!("{}", pt1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_example() {
        let lines = vec![
            String::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            String::from("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
            String::from("bright white bags contain 1 shiny gold bag."),
            String::from("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
            String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
            String::from("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
            String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
            String::from("faded blue bags contain no other bags."),
            String::from("dotted black bags contain no other bags."),
        ];

        let count = pt1(&lines);
        assert_eq!(count, 4);
    }
}
