use ::regex::Regex;
use std::io::stdin;
use std::io::Read;
use std::error::Error;
use std::collections::HashSet;

fn find_start(input: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let n  = input.len();
    let m = input.get(0).map_or(0, |v|v.len());
    for r in 0..n {
        for c in 0..m {
            if input[r][c] == '^' {
                return Some((r,c));
            }
        }
    }
    None
}

const DELTAS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];


fn part_one(input: &Vec<Vec<char>>) -> i64 {
    let mut input = input.clone();
    let Some(start) = find_start(&input) else {
        panic!("bad input");
    };
    let (mut r, mut c) = start;
    let mut dir = 0;
    let mut result = 0;
    loop {
        let nr: usize = (r as i32 + DELTAS[dir].0) as usize;
        let nc: usize = (c as i32 + DELTAS[dir].1) as usize;
        match input.get_mut(nr).and_then(|v| v.get_mut(nc)) {
            Some(c) if *c == '#' => {
                dir = (dir + 1) % 4;
                continue;
            },
            Some(c) if *c == '.' => {
                *c = 'X';
                result += 1;
            },
            None => {
                break;
            }
            _ => {}
        }
        r = nr;
        c = nc;
    }
    result + 1
}

fn part_two(input: &Vec<Vec<char>>) -> i64 {
    let n = input.len();
    let m = input.get(0).map_or(0, |v|v.len());
    let Some(start) = find_start(&input) else {
        panic!("bad input");
    };
    let original_input = input.into_iter().map(|line| {
        line.into_iter().map(|c| {
            match c  {
                '^' => 1,
                '#' => 16,
                _ => 0,
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let obstacals = (0..n).map(|r| {
        (0..m).filter_map(|c| if original_input[r][c] == 0 {
             Some((r, c))
        } else {
            None
        }).collect::<Vec<_>>()
    }).flatten().collect::<Vec<_>>();
    let mut positions = HashSet::new();

    for &(or, oc) in obstacals.iter() {
        let mut input = original_input.clone();
        let (mut r, mut c) = start;
        let mut dir = 0;

        input[or][oc] = 16;
        loop {
            let nr: usize = (r as i32 + DELTAS[dir].0) as usize;
            let nc: usize = (c as i32 + DELTAS[dir].1) as usize;
            match input.get_mut(nr).and_then(|v| v.get_mut(nc)) {
                Some(c) if *c == 16 => {
                    dir = (dir + 1) % 4;
                    continue;
                },
                Some(c) if *c == 0 => {
                    *c = 1 << dir;
                },
                Some(c) => {
                    if (*c) & (1 << dir) != 0 {
                        positions.insert((or, oc));
                        break;
                    }
                    *c |= 1 << dir;
                } 
                None => {
                    break;
                }
            }
            r = nr;
            c = nc;
        }
    }
    positions.len() as i64
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer: Vec<u8> = vec![];
    stdin().read_to_end(&mut buffer)?;
    let _raw_input = String::from_utf8(buffer)?;

    let line_re = Regex::new(r"\r?\n")?;
    let input = line_re.split(&_raw_input).map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));




    Ok(())
}
