use std::mem::swap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardTy {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    A = 0,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
    Max,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Card::A,
            "K" => Card::K,
            "Q" => Card::Q,
            "J" => Card::J,
            "T" => Card::T,
            "9" => Card::N9,
            "8" => Card::N8,
            "7" => Card::N7,
            "6" => Card::N6,
            "5" => Card::N5,
            "4" => Card::N4,
            "3" => Card::N3,
            "2" => Card::N2,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand: [Card; 5],
    ty: CardTy,
    wager: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ty
            .cmp(&other.ty)
            .then_with(|| self.hand.cmp(&other.hand))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let hand: [Card; 5] = [0, 1, 2, 3, 4].map(|i| line[i..i + 1].parse().unwrap());
            let wager = line[6..].parse::<u32>().unwrap();

            let mut values = [0; Card::Max as usize];
            for card in hand {
                values[card as usize] += 1;
            }

            let mut best = (0, 0);
            let mut pen = (0, 0);

            for mut entry in values.iter().copied().enumerate() {
                if entry.1 > best.1 {
                    swap(&mut entry, &mut best);
                }

                if entry.1 > pen.1 {
                    swap(&mut entry, &mut pen);
                }
            }

            let ty = match (best.1, pen.1) {
                (5, _) => CardTy::FiveKind,
                (4, _) => CardTy::FourKind,
                (3, 2) => CardTy::FullHouse,
                (3, _) => CardTy::ThreeKind,
                (2, 2) => CardTy::TwoPair,
                (2, _) => CardTy::OnePair,
                _ => CardTy::HighCard,
            };

            Hand { hand, wager, ty }
        })
        .collect();

    hands.sort();

    let mut sum = 0;

    for (i, hand) in hands.iter().enumerate() {
        let rank = hands.len() - i;
        sum += hand.wager * rank as u32;
    }

    println!("{sum}");
}
