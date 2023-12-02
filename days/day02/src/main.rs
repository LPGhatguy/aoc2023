use std::str::FromStr;

static INPUT: &str = include_str!("../input.txt");
static TEST_INPUT: &str = include_str!("../test-input.txt");

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => return Err(format!("{s} is not a creative color")),
        })
    }
}

fn part_one(input: &str) {
    let games = input.lines().map(|line| {
        let mut parts = line.split(": ");
        let _ = parts.next().unwrap();
        let round_text = parts.next().unwrap();

        round_text.split("; ").map(|round| {
            round.split(", ").map(|hand| {
                let mut parts = hand.split(' ');
                let quantity: u32 = parts.next().unwrap().parse().unwrap();
                let color: Color = parts.next().unwrap().parse().unwrap();
                (quantity, color)
            })
        })
    });

    let bag_contents = [12, 13, 14];
    let mut sum = 0;

    'each_game: for (index, game) in games.enumerate() {
        let id = index + 1;

        for hands in game {
            for (quantity, color) in hands {
                let max = bag_contents[color as usize];
                if quantity > max {
                    continue 'each_game;
                }
            }
        }

        sum += id;
    }

    println!("Answer: {sum}");
}

fn part_two(input: &str) {
    let games = input.lines().map(|line| {
        let mut parts = line.split(": ");
        let _ = parts.next().unwrap();
        let round_text = parts.next().unwrap();

        round_text.split("; ").map(|round| {
            round.split(", ").map(|hand| {
                let mut parts = hand.split(' ');
                let quantity: u32 = parts.next().unwrap().parse().unwrap();
                let color: Color = parts.next().unwrap().parse().unwrap();
                (quantity, color)
            })
        })
    });

    let answer = games
        .map(|game| {
            let mut max = [0, 0, 0];

            for hands in game {
                for (quantity, color) in hands {
                    max[color as usize] = max[color as usize].max(quantity);
                }
            }

            max[0] * max[1] * max[2]
        })
        .sum::<u32>();

    println!("Answer: {answer}");
}

fn main() {
    println!("--- PART ONE ---");
    part_one(TEST_INPUT);
    part_one(INPUT);

    println!("--- PART TWO ---");
    part_two(TEST_INPUT);
    part_two(INPUT);
}
