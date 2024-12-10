
use ::regex::Regex;
use std::cmp::Reverse;
use std::error::Error;
use std::io::stdin;
use std::io::Read;


#[derive(Clone, PartialEq, Eq, Ord, Debug)]
struct Hand {
    bet: i64,
    rank: i32,
    cards: Vec<i32>
}


impl Hand {
    fn calc_rank(hand: &Vec<i32>) -> i32 {
        let mut counts = hand.iter().fold(vec![0; 13], |mut counts, &card| {
            counts[card as usize] += 1;
            counts
        });
        counts.sort_by_key(|&v| Reverse(v));
        let to_check = &counts[0..5];
        match to_check {
            v if v[0] == 5 => 0,
            v if v[0] == 4 => 1,
            v if v[0] == 3 && v[1] == 2 => 2,
            v if v[0] == 3 => 3,
            v if v[0] == 2 && v[1] == 2 => 4,
            v if v[0] == 2 => 5,
            _ => 6
        }
    }

    fn calc_rank_part2(hand: &Vec<i32>) -> i32 {
        let (jokers, mut counts) = hand.iter().fold((0, vec![0; 13]), |(jokers, mut counts), &card| {
            if card != 3 {
                counts[card as usize] += 1;
            }
            (jokers + if card == 3 { 1 } else { 0 }, counts)
        });
        counts.sort_by_key(|&v| Reverse(v));
        counts[0] += jokers;
        let to_check = &counts[0..5];
        match to_check {
            v if v[0] == 5 => 0,
            v if v[0] == 4 => 1,
            v if v[0] == 3 && v[1] == 2 => 2,
            v if v[0] == 3 => 3,
            v if v[0] == 2 && v[1] == 2 => 4,
            v if v[0] == 2 => 5,
            _ => 6
        }
    }

    fn from_str(input: &str, part2: bool) -> Self {
        let line = input.split(' ').collect::<Vec<_>>();
        let mut cards = line[0]
            .chars()
            .map(|c| {
                match c {
                   'A' => 0,
                   'K' => 1,
                   'Q' => 2,
                   'J' => 3,
                   'T' => 4,
                   _ => (14 - (c as u8 - b'0')) as i32
                }
            }).collect::<Vec<_>>();
        let rank = if part2 {
            let rank = Self::calc_rank_part2(&cards);
            cards.iter_mut().for_each(|e| if *e == 3 {*e = 20});
            rank
        } else {
            Self::calc_rank(&cards)
        };
        Self {
            bet: line[1].parse::<i64>().unwrap(),
            rank: rank,
            cards: cards
        }
        
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.rank.partial_cmp(&other.rank) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.cards.partial_cmp(&other.cards)
    }
}

fn part_one(hands: &Vec<Hand>) -> i64 {
    let mut hands = hands.clone();
    hands.sort();
    hands.reverse();

    hands.iter().enumerate().fold(0, |total, (i, hand)| {
        total.checked_add(((i + 1) as i64)*hand.bet).unwrap()
    })

}


fn main() -> Result<(), Box<dyn Error>> {
    let _line_re = Regex::new(r"\r?\n")?;
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let mut _raw_input = String::from_utf8(buf)?;
    let hands_part1 =_line_re.split(&_raw_input).map(|line| {
        Hand::from_str(line, false)
    }).collect::<Vec<_>>();

    let hands_part2 =_line_re.split(&_raw_input).map(|line| {
        Hand::from_str(line, true)
    }).collect::<Vec<_>>();


    println!("Part 1: {}", part_one(&hands_part1));
    println!("Part 2: {}", part_one(&hands_part2));


    Ok(())
}
