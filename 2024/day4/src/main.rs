use ::regex::Regex;
use std::error::Error;
use std::io::stdin;
use std::io::Read;

const WORD: &'static str = "XMAS";
const DELTAS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0,-1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn part_two(values: &Vec<Vec<char>>) -> i32 {
    let n = values.len();
    let m = values.get(0).map_or(0,  |v|v.len());
    let mut result = 0;
    for row in 1..(n-1) {
        for col  in 1..(m-1) {
            if values[row][col] == 'A' {
                let left = match (values[row-1][col-1], values[row+1][col+1])  {
                    ('M', 'S') | ('S', 'M') => true,
                    _ => false,
                };
                let right = match (values[row-1][col+1], values[row+1][col-1])  {
                    ('M', 'S') | ('S', 'M') => true,
                    _ => false,
                };
                if left && right {
                    result += 1;
                }
            }
        }
    }
    result
}

fn part_one(values: &Vec<Vec<char>>, needle: &str) -> i32 {
    fn find(
        values: &Vec<Vec<char>>,
        needle: &str,
        row: usize, 
        col: usize,
        dr: i32,
        dc: i32,
        index: usize
    ) -> i32 {
        let n_b = needle.as_bytes();
        if let Some(&c) = values.get(row).and_then(|v|v.get(col)) {
            if (c as u8) == n_b[index] {
                if index == n_b.len() - 1 {
                    1
                } else {
                    let nr = ((row as i32) + dr) as usize;
                    let nc = ((col as i32) + dc) as usize;
                    find(values, needle, nr, nc, dr, dc, index + 1)
                }
            } else {
                0
            }
        } else {
            0
        }
    }
    let n = values.len();
    let m = values.get(0).map_or(0, |v| v.len());
    let mut result = 0;
    for row in 0..n {
        for col in  0..m {
            for &(dr, dc) in DELTAS.iter() {
                result += find(values, needle, row, col,dr , dc, 0);
            }
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer: Vec<u8> = vec![];
    stdin().read_to_end(&mut buffer)?;
    let raw_input = String::from_utf8(buffer)?;
    let line_regex = Regex::new(r"\r?\n")?;
    let input = line_regex.split(&raw_input).map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    println!("Part 1: {}", part_one(&input, WORD));
    println!("Part 2: {}", part_two(&input));


    Ok(())
}
