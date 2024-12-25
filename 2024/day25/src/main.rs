use regex::Regex;
use std::error::Error;
use std::io::stdin;
use std::io::Read;


fn build_key(key: &Vec<&str>) -> Vec<i32> {
    let mut result = vec![];
    for i in 0..5 {
        let mut h = 0;
        for j in 1..6 {
            if key[j].as_bytes()[i] == b'#' {
                h += 1;
            } else {
                break;
            }
        }
        result.push(h);
    }
    result
}

fn build_lock(key: &Vec<&str>) -> Vec<i32> {
    let mut result = vec![];
    for i in 0..5 {
        let mut h = 0;
        for j in (1..6).rev() {
            if key[j].as_bytes()[i] == b'#' {
                h += 1;
            } else {
                break;
            }
        }
        result.push(h);
    }
    result
}

fn part_one(input: &Vec<Vec<&str>>) -> i64 {
    let mut keys = vec![];
    let mut locks = vec![];

    for kv in input.iter() {
        if kv[0] == "#####" {
            keys.push(build_key(kv));
        } else {
            locks.push(build_lock(kv));
        }
    }
    let mut result = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if lock.iter().zip(key.iter()).all(|(k,v)| k + v < 6) {
                result += 1;
            }
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input: String = String::from_utf8(buf)?;
    let _line_re: Regex = Regex::new(r"\r?\n")?;
    let _part_re: Regex = Regex::new(r"\r?\n\r?\n")?;

    let input =_part_re.split(&_raw_input).map(|key_lock| {
        _line_re.split(key_lock).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    println!("Part 1: {}", part_one(&input));

    Ok(())
}
