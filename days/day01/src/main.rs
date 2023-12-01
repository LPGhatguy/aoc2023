static INPUT: &str = include_str!("../input.txt");
static TEST_INPUT: &str = include_str!("../test-input.txt");
static TEST_INPUT_2: &str = include_str!("../test-input-2.txt");

fn solve<'a, F, I>(input: &'a str, f: F) -> usize
where
    F: Fn(&'a str) -> I,
    I: Iterator<Item = usize> + 'a,
{
    input
        .lines()
        .filter_map(|line| {
            let mut iter = f(line);
            let first = iter.next()?;
            let mut last = first;

            for c in iter {
                last = c;
            }

            Some(first * 10 + last)
        })
        .sum::<usize>()
}

fn part_one(input: &str) {
    let answer = solve(input, |line| {
        line.chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as usize))
    });

    println!("{answer}");
}

fn part_two(input: &str) {
    let numbers = [
        ["1", "one"],
        ["2", "two"],
        ["3", "three"],
        ["4", "four"],
        ["5", "five"],
        ["6", "six"],
        ["7", "seven"],
        ["8", "eight"],
        ["9", "nine"],
    ];

    let answer = solve(input, |line| {
        (0..line.len()).filter_map(|i| {
            let remaining = &line[i..];

            for (digit, names) in numbers.iter().enumerate() {
                for name in names {
                    if remaining.starts_with(name) {
                        return Some(digit + 1);
                    }
                }
            }

            None
        })
    });

    println!("{answer}");
}

fn main() {
    println!("--- Part One ---");
    part_one(TEST_INPUT);
    part_one(INPUT);

    println!("--- Part Two ---");
    part_two(TEST_INPUT_2);
    part_two(INPUT);
}
