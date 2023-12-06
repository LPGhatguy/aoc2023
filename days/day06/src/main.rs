static INPUT: &str = include_str!("../input.txt");
static TEST_INPUT: &str = include_str!("../test-input.txt");

fn part_one(input: &str) {
    let mut lines = input.lines();
    let times = lines.next().unwrap()[6..]
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok());
    let distances = lines.next().unwrap()[10..]
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok());

    let mut total = 1;

    for (time, distance) in times.zip(distances) {
        let mut ways_to_win = 0;

        for hold_duration in 1..time {
            let remaining = time - hold_duration;
            let distance_traveled = remaining * hold_duration;

            if distance_traveled > distance {
                ways_to_win += 1;
            }
        }

        total *= ways_to_win;
    }

    println!("{total}");
}

fn part_two(input: &str) {
    let mut lines = input.lines();
    let time = lines.next().unwrap()[6..]
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let distance = lines.next().unwrap()[10..]
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    let mut ways_to_win = 0;

    for hold_duration in 1..time {
        let remaining = time - hold_duration;
        let distance_traveled = remaining * hold_duration;

        if distance_traveled > distance {
            ways_to_win += 1;
        }
    }

    println!("{ways_to_win}");
}

fn main() {
    println!("--- PART ONE ---");
    part_one(TEST_INPUT);
    part_one(INPUT);

    println!("--- PART TWO ---");
    part_two(TEST_INPUT);
    part_two(INPUT);
}
