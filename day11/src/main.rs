use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq)]
enum SeatStatus {
    Floor,
    Empty,
    Occupied,
}

struct WaitingArea {
    seats: Vec<Vec<SeatStatus>>,
}

impl WaitingArea {
    pub fn from_lines(lines: &[String]) -> WaitingArea {
        let seats = lines
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

        return WaitingArea { seats: seats };
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
}

fn main() {
    let mut waiting_area = WaitingArea::from_file("input");
    let mut changes = waiting_area.iterate();
    while changes > 0 {
        changes = waiting_area.iterate();
    }

    println!("{}", waiting_area.occupied_seats());
}
