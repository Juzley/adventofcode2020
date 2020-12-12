use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Clone)]
struct Program {
    prg: Vec<Operation>,
    pc: usize,
    acc: i32,
    visited: HashSet<usize>,
}

impl Program {
    pub fn from_strings(lines: &[String]) -> Program {
        let program = lines
            .into_iter()
            .map(|line| {
                let strs: Vec<&str> = line.split(" ").collect();
                let val = strs[1].parse::<i32>().unwrap();
                match strs[0] {
                    "acc" => return Operation::Acc(val),
                    "jmp" => return Operation::Jmp(val),
                    "nop" => return Operation::Nop(val),
                    _ => panic!("Unrecognized operation"),
                }
            })
            .collect();

        return Program {
            prg: program,
            pc: 0,
            acc: 0,
            visited: HashSet::new(),
        };
    }

    pub fn from_file(filename: &str) -> Program {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        return Program::from_strings(&lines);
    }


    pub fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
        self.visited = HashSet::new();
    }

    pub fn step(&mut self) {
        let op = &self.prg[self.pc];
        println!("Op: {:?}, Pc: {}, Acc: {}", op, self.pc, self.acc);
        match op {
            Operation::Acc(val) => {
                self.acc += val;
                self.pc += 1;
            }
            Operation::Jmp(val) => {
                let pc = self.pc as i32 + val;
                self.pc = pc as usize;
            }
            Operation::Nop(_) => {
                self.pc += 1;
            }
        }
    }

    pub fn execute(&mut self) -> Result<i32, i32> {
        loop {
            if self.pc >= self.prg.len() {
                return Ok(self.acc);
            }

            if self.visited.contains(&self.pc) {
                return Err(self.acc);
            }

            self.visited.insert(self.pc);
            self.step();
        }
    }

    pub fn fix_loop(&mut self) -> i32 {
        let mut states: Vec<Program> = vec![];

        // First find the loop.
        loop {
            if self.visited.contains(&self.pc) {
                break;
            }

            states.push(self.clone());
            self.visited.insert(self.pc);
            self.step();
        }

        // Work back up the stack, trying substituing jmps for nops and
        // vice-versa, and see if that fixes the loop.
        loop {
            let mut candidate = states.pop().unwrap();

            match candidate.prg[candidate.pc] {
                Operation::Jmp(val) =>
                    candidate.prg[candidate.pc] = Operation::Nop(val),
                Operation::Nop(val) =>
                    candidate.prg[candidate.pc] = Operation::Jmp(val),
                _ => continue,
            }

            let result = candidate.execute();
            match result {
                Ok(val) => return val,
                Err(_) => continue,
            }
        }
    }
}

fn main() {
    let mut prg = Program::from_file("input");
    if let Err(result) = prg.execute() {
        println!("Part 1: {}", result);
    }

    prg.reset();
    let result = prg.fix_loop();
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_example() {
        let lines = vec![
            String::from("nop +0"),
            String::from("acc +1"),
            String::from("jmp +4"),
            String::from("acc +3"),
            String::from("jmp -3"),
            String::from("acc -99"),
            String::from("acc +1"),
            String::from("jmp -4"),
            String::from("acc +6"),
        ];
        let mut prg = Program::from_strings(&lines);
        let result = prg.execute();
        assert_eq!(result, Err(5));
    }

    #[test]
    fn pt2_example() {
        let lines = vec![
            String::from("nop +0"),
            String::from("acc +1"),
            String::from("jmp +4"),
            String::from("acc +3"),
            String::from("jmp -3"),
            String::from("acc -99"),
            String::from("acc +1"),
            String::from("jmp -4"),
            String::from("acc +6"),
        ];
        let mut prg = Program::from_strings(&lines);
        let result = prg.fix_loop();
        assert_eq!(result, 8);
    }
}
