use std::collections::HashMap;

static INPUT: &str = include_str!("../input.txt");
static TEST_INPUT: &str = include_str!("../test-input.txt");

fn part_one(input: &str) {
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    let graph: HashMap<_, _> = lines
        .skip(1)
        .map(|line| {
            let node = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            (node, (left, right))
        })
        .collect();

    let mut location = "AAA";
    let mut steps = 0;
    for dir in directions.chars().cycle() {
        if location == "ZZZ" {
            break;
        }

        steps += 1;

        let node = graph.get(location).unwrap();
        match dir {
            'L' => location = node.0,
            'R' => location = node.1,
            _ => unreachable!(),
        }
    }

    println!("{steps}");
}

#[derive(Debug, Clone, Copy)]
struct Cycle {
    start: usize,
    length: usize,
    steps_to_z: Option<usize>,
}

struct Path<'a> {
    location: &'a str,
    visited: HashMap<(&'a str, usize), usize>,
    cycle: Option<Cycle>,
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn part_two(input: &str) {
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    let graph: HashMap<_, _> = lines
        .skip(1)
        .map(|line| {
            let node = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            (node, (left, right))
        })
        .collect();

    let mut paths: Vec<_> = graph
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|location| Path {
            location,
            visited: HashMap::new(),
            cycle: None,
        })
        .collect();

    for (steps, (index, dir)) in directions.chars().enumerate().cycle().enumerate() {
        for path in &mut paths {
            if path.cycle.is_none() {
                if let Some(prev_step) = path.visited.insert((path.location, index), steps) {
                    path.cycle = Some(Cycle {
                        start: prev_step,
                        length: steps - prev_step,
                        steps_to_z: None,
                    });
                }
            }

            if let Some(cycle) = &mut path.cycle {
                if cycle.steps_to_z.is_none() && path.location.ends_with('Z') {
                    cycle.steps_to_z = Some(steps - cycle.start);
                }
            }
        }

        if paths.iter().all(|path| path.location.ends_with('Z')) {
            println!("{steps}");
            return;
        }

        if paths
            .iter()
            .all(|path| path.cycle.map(|c| c.steps_to_z.is_some()).unwrap_or(false))
        {
            break;
        }

        for path in &mut paths {
            let node = graph.get(path.location).unwrap();
            match dir {
                'L' => path.location = node.0,
                'R' => path.location = node.1,
                _ => unreachable!(),
            }
        }
    }

    let mut len = paths[0].cycle.unwrap().length;
    for path in paths.iter().skip(1) {
        len = lcm(len, path.cycle.unwrap().length);
    }

    println!("{len}");
}

fn main() {
    println!("--- PART ONE ---");
    part_one(TEST_INPUT);
    part_one(INPUT);

    println!("--- PART TWO ---");
    part_two(TEST_INPUT);
    part_two(INPUT);
}
