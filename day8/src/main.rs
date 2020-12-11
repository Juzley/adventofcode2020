use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop,
}

struct Program {
    prg: Vec<Operation>,
    pc: usize,
    acc: i32,
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
                    "nop" => return Operation::Nop,
                    _ => panic!("Unrecognized operation"),
                }
            })
            .collect();

        return Program {
            prg: program,
            pc: 0,
            acc: 0,
        };
    }

    pub fn from_file(filename: &str) -> Program {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        return Program::from_strings(&lines);
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
            Operation::Nop => {
                self.pc += 1;
            }
        }
    }

    pub fn run_til_loop(&mut self) -> i32 {
        let mut visited: HashSet<usize> = HashSet::new();
        loop {
            if visited.contains(&self.pc) {
                break;
            }

            visited.insert(self.pc);
            self.step();
        }
        return self.acc;
    }
}

fn main() {
    let mut prg = Program::from_file("input");
    let result = prg.run_til_loop();
    println!("{}", result);
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
        let result = prg.run_til_loop();
        assert_eq!(result, 5);
    }
}
