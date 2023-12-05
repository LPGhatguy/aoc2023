use std::mem;
use std::ops::Range;

static INPUT: &str = include_str!("../input.txt");
static TEST_INPUT: &str = include_str!("../test-input.txt");

#[derive(Debug)]
struct Rule {
    input: Range<u64>,
    output: u64,
}

fn parse_rules<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Vec<Rule>> {
    let mut rulesets = Vec::new();
    let mut current_ruleset = Vec::new();
    let mut primed = false;

    for line in lines {
        let is_numbers = line
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false);
        if !is_numbers {
            if primed {
                rulesets.push(mem::take(&mut current_ruleset));

                primed = false;
            }

            continue;
        }

        primed = true;

        let mut parts = line.split(' ');
        let dest: u64 = parts.next().unwrap().parse().unwrap();
        let src: u64 = parts.next().unwrap().parse().unwrap();
        let len: u64 = parts.next().unwrap().parse().unwrap();

        current_ruleset.push(Rule {
            input: src..(src + len),
            output: dest,
        });
    }

    if primed {
        rulesets.push(mem::take(&mut current_ruleset));
    }

    rulesets
}

fn part_one(input: &str) {
    let mut lines = input.lines();

    let seeds_line = lines.next().unwrap().strip_prefix("seeds: ").unwrap();
    let seeds: Vec<_> = seeds_line
        .split(' ')
        .filter_map(|v| v.parse::<u64>().ok())
        .collect();

    let rulesets = parse_rules(lines);

    let mut min = u64::MAX;

    for seed in seeds {
        let mut value = seed;

        for ruleset in &rulesets {
            for rule in ruleset {
                if rule.input.contains(&value) {
                    value -= rule.input.start;
                    value += rule.output;
                    break;
                }
            }
        }

        min = min.min(value);
    }

    println!("{min}");
}

fn part_two(input: &str) {
    let mut lines = input.lines();

    let seeds_line = lines.next().unwrap().strip_prefix("seeds: ").unwrap();
    let seeds: Vec<_> = seeds_line
        .split(' ')
        .filter_map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|slice| {
            let base = slice[0];
            let len = slice[1];
            base..(base + len)
        })
        .collect();

    let rulesets = parse_rules(lines);

    let mut min = u64::MAX;

    for range in seeds {
        let mut seed = range.start;
        while seed < range.end {
            let mut value = seed;
            let mut skip = u64::MAX;

            for ruleset in &rulesets {
                for rule in ruleset {
                    if rule.input.contains(&value) {
                        skip = skip.min(rule.input.end - value);
                        value -= rule.input.start;
                        value += rule.output;
                        break;
                    }
                }
            }

            seed += skip;
            min = min.min(value);
        }
    }

    println!("{min}");
}

fn main() {
    println!("--- PART ONE ---");
    part_one(TEST_INPUT);
    part_one(INPUT);

    println!("--- PART TWO ---");
    part_two(TEST_INPUT);
    part_two(INPUT);
}
