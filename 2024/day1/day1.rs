
use std::io::{stdin};
use std::io::Read;
use std::error::Error;
use std::collections::HashMap;
use ::regex::Regex;

fn find_similarity(values: &Vec<(i32, i32)>) -> i64 {
    let (left, right) = values.iter().fold((vec![], vec![]), |(mut left, mut right), &(l, r)| {
        left.push(l);
        right.push(r);
        (left, right)
    });
    let right_count = right.into_iter().fold(HashMap::new(), |mut counts, v| {
        *(counts.entry(v).or_insert(0)) += 1;
        counts
    });
    left.iter().fold(0i64, |result, &val| {
        result + (right_count.get(&val).map_or(0, |&v| v) as i64)*(val as i64)
    })
}

fn find_differences(values: &Vec<(i32, i32)>) -> i32 {
    let (mut left, mut right) = values.iter().fold((vec![], vec![]), |(mut left, mut right), &(l, r)| {
        left.push(l);
        right.push(r);
        (left, right)
    });

    left.sort();
    right.sort();
    left.into_iter().zip(right.into_iter()).fold(0, |result, (left, right)| {
        result + right.abs_diff(left) as i32
    })
}


pub fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let values = String::from_utf8(buf)?;
    let input_s = values.as_str();
    let space_split = Regex::new(r"[\s]+").unwrap();
    let new_line_split = Regex::new(r"\n").unwrap();
    let formated  = new_line_split.split(input_s)
        .filter(|s|!s.is_empty())
        .filter_map(|line| {
        let mut l_iter = space_split.split(line);
        match (l_iter.next(), l_iter.next()) {
            (Some(v1), Some(v2)) => {
                Some((v1.parse::<i32>().unwrap(), v2.parse::<i32>().unwrap()))
            },
            _ => {
                None
            }
        }
    }).collect::<Vec<_>>();
    println!();
    println!("Part 1: {}", find_differences(&formated));
    println!("Part 2: {}", find_similarity(&formated));

    Ok(())

}