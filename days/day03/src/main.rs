use std::collections::{HashMap, HashSet, VecDeque};

static INPUT: &str = include_str!("../input.txt");
static TEST_INPUT: &str = include_str!("../test-input.txt");

struct Grid {
    storage: Vec<u8>,
    width: isize,
    height: isize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let storage: Vec<_> = input
            .lines()
            .flat_map(|line| {
                width = line.len() as isize;
                height += 1;
                line.bytes()
            })
            .collect();

        Self {
            storage,
            width,
            height,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || y < 0 || x > (self.width - 1) || y > (self.height - 1) {
            return None;
        }

        let index = (y * self.width + x) as usize;
        Some(self.storage[index])
    }

    fn iter(&self) -> impl Iterator<Item = (isize, isize, u8)> + '_ {
        self.storage
            .iter()
            .copied()
            .enumerate()
            .map(|(index, value)| {
                let x = (index as isize) % self.width;
                let y = (index as isize) / self.width;
                (x, y, value)
            })
    }
}

fn adjacent(x: isize, y: isize) -> [(isize, isize); 8] {
    [
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
        (x - 1, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y + 1),
    ]
}

fn part_one(input: &str) {
    let grid = Grid::parse(input);

    let mut visited = HashSet::new();
    let mut digit_buffer = Vec::new();
    let mut symbol_queue = VecDeque::new();

    let mut sum = 0;

    for (x, y, square) in grid.iter() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        if let Some(d) = (square as char).to_digit(10) {
            digit_buffer.clear();
            symbol_queue.clear();
            symbol_queue.extend(adjacent(x, y));

            digit_buffer.push(d);

            for x in (x + 1)..grid.width {
                let square = grid.get(x, y).unwrap();
                if let Some(d) = (square as char).to_digit(10) {
                    digit_buffer.push(d);
                    visited.insert((x, y));
                    symbol_queue.extend(adjacent(x, y));
                } else {
                    break;
                }
            }

            let mut number = 0;
            for (power, digit) in digit_buffer.iter().rev().enumerate() {
                number += digit * 10u32.pow(power as u32);
            }

            let mut found_symbol = false;
            for &(x, y) in &symbol_queue {
                let Some(square) = grid.get(x, y) else {
                    continue;
                };

                if !square.is_ascii_digit() && square != b'.' {
                    found_symbol = true;
                    break;
                }
            }

            if found_symbol {
                sum += number;
            }
        }
    }

    println!("{sum}");
}

fn part_two(input: &str) {
    let grid = Grid::parse(input);

    let mut numbers = Vec::new();
    let mut number_map = HashMap::new();

    let mut visited = HashSet::new();
    let mut digit_buffer = Vec::new();

    for (x, y, square) in grid.iter() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        if let Some(d) = (square as char).to_digit(10) {
            digit_buffer.clear();
            digit_buffer.push(d);

            let number_id = numbers.len();
            number_map.insert((x, y), number_id);

            for x in (x + 1)..grid.width {
                let square = grid.get(x, y).unwrap();
                if let Some(d) = (square as char).to_digit(10) {
                    digit_buffer.push(d);
                    visited.insert((x, y));
                    number_map.insert((x, y), number_id);
                } else {
                    break;
                }
            }

            let mut number = 0;
            for (power, digit) in digit_buffer.iter().rev().enumerate() {
                number += digit * 10u32.pow(power as u32);
            }

            numbers.push(number);
        }
    }

    let mut numbers_found = HashSet::new();
    let mut sum = 0;

    for (x, y, square) in grid.iter() {
        if square != b'*' {
            continue;
        }

        numbers_found.clear();

        for (x, y) in adjacent(x, y) {
            if let Some(number_id) = number_map.get(&(x, y)) {
                numbers_found.insert(number_id);
            }
        }

        if numbers_found.len() == 2 {
            let mut iter = numbers_found.iter();
            let a_id = iter.next().unwrap();
            let b_id = iter.next().unwrap();
            let a = numbers[**a_id];
            let b = numbers[**b_id];

            sum += a * b;
        }
    }

    println!("{sum}");
}

fn main() {
    println!("--- PART ONE ---");
    part_one(TEST_INPUT);
    part_one(INPUT);

    println!("--- PART TWO ---");
    part_two(TEST_INPUT);
    part_two(INPUT);
}
