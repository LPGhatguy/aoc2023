use std::collections::VecDeque;

static INPUT: &str = include_str!("../input.txt");
static TEST_INPUT: &str = include_str!("../test-input.txt");

fn card_matches(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines().map(|line| {
        let mut parts = line.split(": ");
        let _ = parts.next().unwrap();
        let data = parts.next().unwrap();

        let mut parts = data.split('|');
        let winning = parts.next().unwrap();
        let mine = parts.next().unwrap();

        let winning: Vec<_> = winning
            .split(' ')
            .filter_map(|num| num.trim().parse::<u32>().ok())
            .collect();
        let mine: Vec<_> = mine
            .split(' ')
            .filter_map(|num| num.trim().parse::<u32>().ok())
            .collect();

        mine.iter().filter(|n| winning.contains(n)).count() as u32
    })
}

fn part_one(input: &str) {
    let score: u32 = card_matches(input)
        .filter_map(|matching| Some(2u32.pow(matching.checked_sub(1)?)))
        .sum();

    println!("{score}");
}

fn part_two(input: &str) {
    let card_matches: Vec<_> = card_matches(input).map(|n| n as usize).collect();

    let mut queue = VecDeque::new();
    queue.extend(0..card_matches.len());

    let mut visited = 0;

    while let Some(index) = queue.pop_front() {
        visited += 1;

        let Some(&count) = card_matches.get(index) else {
            continue;
        };

        let range = (index + 1)..(index + 1 + count);
        queue.extend(range);
    }

    println!("{visited}");
}

fn main() {
    println!("--- PART ONE ---");
    part_one(TEST_INPUT);
    part_one(INPUT);

    println!("--- PART TWO ---");
    part_two(TEST_INPUT);
    part_two(INPUT);
}
