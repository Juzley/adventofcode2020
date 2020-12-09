use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
enum MapSquare {
    Empty,
    Tree,
}

struct Map {
    grid: Vec<Vec<MapSquare>>,
}

impl Map {
    fn from_strings(input: &[String]) -> Map {
        let grid = input.into_iter().map(|line| {
            let squares: Vec<MapSquare> = line
                .chars()
                .map(|c| match c {
                    '#' => MapSquare::Tree,
                    _ => MapSquare::Empty,
                }).collect();
            return squares;
        }).collect();

        return Map {
            grid: grid,
        };
    }

    fn from_file(filename: &str) -> Map {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let result: Result<Vec<String>, _> = reader.lines().collect();
        let input = result.unwrap();
        return Map::from_strings(&input);
    }
}

fn main() {
    let map = Map::from_file("input");
    
    let height = map.grid.len();
    let width = map.grid[0].len();

    let slopes = vec![(1,1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut result: u64 = 1;
    for slope in slopes {
        let x_inc = slope.0;
        let y_inc = slope.1;

        let mut x = 0;
        let mut count = 0;
        for y in (y_inc..height).step_by(y_inc) {
            x += x_inc;
            x = x % width;

            if map.grid[y][x] == MapSquare::Tree {
                count += 1;
            }
        }

        println!("Slope ({}, {}), trees {}", x_inc, y_inc, count);
        result *= count;
    }

    println!("Result: {}", result);
}
