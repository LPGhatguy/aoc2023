static INPUT: &str = include_str!("../input.txt");
static TEST_INPUT: &str = include_str!("../test-input.txt");

struct History {
    rows: Vec<Vec<i64>>,
}

fn run(input: &str) {
    let mut histories: Vec<History> = input
        .lines()
        .map(|line| {
            let top = line.split(' ').map(|num| num.parse().unwrap()).collect();

            History { rows: vec![top] }
        })
        .collect();

    let mut part_one = 0;
    let mut part_two = 0;

    for history in &mut histories {
        // triangulate...
        loop {
            let last = history.rows.last().unwrap();

            let done = last.iter().all(|&x| x == 0);
            if done {
                break;
            }

            let next_row: Vec<_> = last
                .windows(2)
                .map(|window| {
                    let (a, b) = (window[0], window[1]);
                    b - a
                })
                .collect();
            history.rows.push(next_row);
        }

        let last = history.rows.last_mut().unwrap();
        last.push(0);

        // ...and extend!
        for i in (1..history.rows.len()).rev() {
            let a = &history.rows[i];
            let a_last = *a.last().unwrap();
            let a_first = a[0];
            let b = &mut history.rows[i - 1];
            let b_last = *b.last().unwrap();
            let b_first = b[0];

            b.push(a_last + b_last);
            b.insert(0, b_first - a_first);
        }

        part_one += history.rows[0].last().unwrap();
        part_two += history.rows[0][0];
    }

    println!("Part one: {part_one}");
    println!("Part two: {part_two}");
}

fn main() {
    run(TEST_INPUT);
    run(INPUT);
}
