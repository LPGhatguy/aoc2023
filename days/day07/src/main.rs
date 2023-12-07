mod part_one;
mod part_two;

static INPUT: &str = include_str!("../input.txt");
static TEST_INPUT: &str = include_str!("../test-input.txt");

fn main() {
    println!("--- PART ONE ---");
    part_one::part_one(TEST_INPUT);
    part_one::part_one(INPUT);

    println!("--- PART TWO ---");
    part_two::part_two(TEST_INPUT);
    part_two::part_two(INPUT);
}
