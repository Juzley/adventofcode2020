use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq)]
enum SeatStatus {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn to_vec(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }
}

#[derive(Clone)]
struct WaitingArea {
    seats: Vec<Vec<SeatStatus>>,
    height: usize,
    width: usize,
}

type Cache = HashMap<(usize, usize, Direction), SeatStatus>;

impl WaitingArea {
    pub fn from_lines(lines: &[String]) -> WaitingArea {
        let seats: Vec<Vec<SeatStatus>> = lines
            .into_iter()
            .map(|line| {
                let row: Vec<SeatStatus> = line
                    .chars()
                    .map(|c| match c {
                        'L' => SeatStatus::Empty,
                        '#' => SeatStatus::Occupied,
                        _ => SeatStatus::Floor,
                    })
                    .collect();

                return row;
            })
            .collect();

        let height = seats.len();
        let width = seats[0].len();

        return WaitingArea {
            seats: seats,
            height: height,
            width: width,
        };
    }

    pub fn from_file(filename: &str) -> WaitingArea {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        return WaitingArea::from_lines(&lines);
    }

    pub fn occupied_seats(&self) -> usize {
        let mut count = 0;
        for row in &self.seats {
            count += row
                .into_iter()
                .filter(|&&s| s == SeatStatus::Occupied)
                .count();
        }

        return count;
    }

    fn count_adjacent(&self, x: usize, y: usize, status: SeatStatus) -> usize {
        let mut count = 0;
        let mut update = |seat| -> () {
            if seat == status {
                count += 1;
            }
        };

        // Row above
        if y > 0 {
            if x > 0 {
                update(self.seats[y - 1][x - 1]);
            }

            update(self.seats[y - 1][x]);

            if x < self.seats[0].len() - 1 {
                update(self.seats[y - 1][x + 1]);
            }
        }

        // Same row
        if x > 0 {
            update(self.seats[y][x - 1]);
        }

        if x < self.seats[0].len() - 1 {
            update(self.seats[y][x + 1]);
        }

        // Row below
        if y < self.seats.len() - 1 {
            if x > 0 {
                update(self.seats[y + 1][x - 1]);
            }

            update(self.seats[y + 1][x]);

            if x < self.seats[0].len() - 1 {
                update(self.seats[y + 1][x + 1]);
            }
        }

        return count;
    }

    pub fn iterate(&mut self) -> usize {
        let mut new_seats = self.seats.clone();
        let mut updates = 0;

        for (y, row) in self.seats.iter().enumerate() {
            for (x, seat) in row.iter().enumerate() {
                if *seat == SeatStatus::Empty {
                    let count = self.count_adjacent(x, y, SeatStatus::Occupied);
                    if count == 0 {
                        new_seats[y][x] = SeatStatus::Occupied;
                        updates += 1;
                    }
                }

                if *seat == SeatStatus::Occupied {
                    let count = self.count_adjacent(x, y, SeatStatus::Occupied);
                    if count >= 4 {
                        new_seats[y][x] = SeatStatus::Empty;
                        updates += 1;
                    }
                }
            }
        }

        self.seats = new_seats;
        return updates;
    }

    fn get_adjacent_coords(&self, x: usize, y: usize, dir: Direction) -> Option<(usize, usize)> {
        let (x_inc, y_inc) = dir.to_vec();
        let new_x = x as isize + x_inc;
        let new_y = y as isize + y_inc;

        let w = self.width as isize;
        let h = self.height as isize;
        if new_x < 0 || new_x + 1 > w || new_y < 0 || new_y + 1 > h {
            return None;
        }
        return Some((new_x as usize, new_y as usize));
    }

    fn get_status_extended(
        &self,
        cache: &mut Cache,
        x: usize,
        y: usize,
        dir: Direction,
    ) -> SeatStatus {
        let cache_key = (x, y, dir);
        if !cache.contains_key(&cache_key) {
            let result;
            if self.seats[y][x] == SeatStatus::Empty {
                result = SeatStatus::Empty;
            } else if self.seats[y][x] == SeatStatus::Occupied {
                result = SeatStatus::Occupied;
            } else {
                if let Some((new_x, new_y)) = self.get_adjacent_coords(x, y, dir) {
                    result = self.get_status_extended(cache, new_x, new_y, dir);
                } else {
                    // Reached the edge of the area, return this tile.
                    result = SeatStatus::Floor;
                }
            }

            cache.insert(cache_key, result);
        }

        return cache[&cache_key];
    }

    fn count_adjacent_extended(
        &self,
        cache: &mut Cache,
        x: usize,
        y: usize,
        status: SeatStatus,
    ) -> usize {
        return vec![
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ]
        .into_iter()
        .filter(|&dir| {
            let adj = self.get_adjacent_coords(x, y, dir);
            return match adj {
                Some((adj_x, adj_y)) => {
                    self.get_status_extended(cache, adj_x, adj_y, dir) == status
                }
                None => false,
            };
        })
        .count();
    }

    pub fn iterate_extended(&mut self) -> usize {
        let mut cache: Cache = HashMap::new();
        let mut new_seats = self.seats.clone();
        let mut updates = 0;

        for (y, row) in self.seats.iter().enumerate() {
            for (x, seat) in row.iter().enumerate() {
                if *seat == SeatStatus::Empty {
                    let count =
                        self.count_adjacent_extended(&mut cache, x, y, SeatStatus::Occupied);
                    if count == 0 {
                        new_seats[y][x] = SeatStatus::Occupied;
                        updates += 1;
                    }
                }

                if *seat == SeatStatus::Occupied {
                    let count =
                        self.count_adjacent_extended(&mut cache, x, y, SeatStatus::Occupied);
                    if count >= 5 {
                        new_seats[y][x] = SeatStatus::Empty;
                        updates += 1;
                    }
                }
            }
        }

        self.seats = new_seats;
        return updates;
    }
}

fn pt1(mut waiting_area: WaitingArea) -> usize {
    let mut changes = waiting_area.iterate();
    while changes > 0 {
        changes = waiting_area.iterate();
    }

    return waiting_area.occupied_seats();
}

fn pt2(mut waiting_area: WaitingArea) -> usize {
    let mut changes = waiting_area.iterate_extended();
    while changes > 0 {
        changes = waiting_area.iterate_extended();
    }

    return waiting_area.occupied_seats();
}

fn main() {
    let waiting_area = WaitingArea::from_file("input");
    let pt1_result = pt1(waiting_area.clone());
    let pt2_result = pt2(waiting_area);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt2_example() {
        let lines = vec![
            String::from("L.LL.LL.LL"),
            String::from("LLLLLLL.LL"),
            String::from("L.L.L..L.."),
            String::from("LLLL.LL.LL"),
            String::from("L.LL.LL.LL"),
            String::from("L.LLLLL.LL"),
            String::from("..L.L....."),
            String::from("LLLLLLLLLL"),
            String::from("L.LLLLLL.L"),
            String::from("L.LLLLL.LL"),
        ];

        let waiting_area = WaitingArea::from_lines(&lines);
        let result = pt2(waiting_area);
        assert_eq!(result, 26);
    }
}
