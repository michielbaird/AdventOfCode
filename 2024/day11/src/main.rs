use ::regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::stdin;
use std::io::Read;

fn convert_stone(v: i64) -> Vec<i64> {
    let mut next = vec![];
    if v == 0 {
        next.push(1);
    } else {
        let s = v.to_string();
        if s.as_bytes().len() % 2 == 0 {
            let mid =  s.as_bytes().len()/2;
            next.push((&s[..mid]).parse::<i64>().unwrap());
            next.push((&s[mid..]).parse::<i64>().unwrap());
        } else {
            next.push(v * 2024);
        }
    }
    next
}

fn part_one_two(steps: i32, input: &Vec<i64>) -> i64 {
    let mut lookup = HashMap::new();
    let mut counts  = input.iter().fold(HashMap::new(), |mut counts, &v| {
        *(counts.entry(v).or_insert(0)) += 1;
        counts
    });

    for _ in 0..steps {
        let mut next = HashMap::new();
        for (&k,count) in counts.iter() {
            for &v  in lookup.entry(k).or_insert_with(||convert_stone(k)).iter() {
                *(next.entry(v).or_insert(0)) += count;
            }
        }
        counts = next;
    } 
    counts.values().copied().sum::<i64>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let _line_re = Regex::new(r"\r?\n")?;
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input = String::from_utf8(buf)?;
    let input = _raw_input.split(' ').map(|e| e.parse::<i64>().unwrap()).collect::<Vec<_>>();


    println!("Part 1: {}", part_one_two(25, &input));
    println!("Part 2: {}", part_one_two(75, &input));


    Ok(())
}
