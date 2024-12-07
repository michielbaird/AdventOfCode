use ::regex::Regex;
use std::io::stdin;
use std::io::Read;
use std::error::Error;

fn part_one(input: &str) -> i64 {
    let to_read = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
    to_read.find_iter(input).map(|a| {
        let caps = to_read.captures(&input[a.start()..a.end()]).unwrap();
        let left = caps[1].parse::<i64>().unwrap();
        let right = caps[2].parse::<i64>().unwrap();
        left*right
    }).sum()
}

fn part_two(input: &str) -> i64 {
    let to_read = Regex::new(r"do(:?n't)?\(\)|mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
    to_read.find_iter(input).fold((true, 0), |(enabled, sum), a| {
        if &input[a.start()..a.end()] == "do()" {
            (true, sum)
        } else if &input[a.start()..a.end()] == "don't()" {
            (false, sum)
        } else {
            if enabled {
                let caps = to_read.captures(&input[a.start()..a.end()]).unwrap();
                let left = caps[2].parse::<i64>().unwrap();
                let right = caps[3].parse::<i64>().unwrap();
                (enabled, sum + left*right)
            } else {
                (enabled, sum)
            }
        }
    }).1
}

fn main() -> Result<(), Box<dyn Error>> {

    let mut buffer = vec![];
    stdin().read_to_end(&mut buffer)?;
    let raw_input = String::from_utf8(buffer)?;
    println!("Part 1: {}", part_one(&raw_input));
    println!("Part 2: {}", part_two(&raw_input));


    Ok(())
}
