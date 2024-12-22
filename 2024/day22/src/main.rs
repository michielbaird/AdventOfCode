use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::io::stdin;
use std::io::Read;

fn part_one(input: &Vec<i64>) -> i64 {
    let mut sum = 0;
    let modulo = 16777216;
    for &v in input {
        let mut cur = v;
        for _ in 0..2000 {
            cur = ((cur*64)^cur) % modulo;
            cur = ((cur/32)^cur) % modulo;
            cur = ((cur*2048)^cur) % modulo;
            //println!("{}", cur);
        }
        sum += cur;
    }
    
    sum
}

fn part_two(input: &Vec<i64>) -> i64 {
    fn advance(cur: &mut i64) -> i8 {
        let prev = *cur;
        let mut c = *cur;
        let modulo = 16777216;
        c = ((c*64)^c) % modulo;
        c = ((c/32)^c) % modulo;
        c = ((c*2048)^c) % modulo;
        *cur = c;
        ((c % 10) - (prev % 10)) as i8
    }
    let mut hash_maps = vec![];
    for &v in input.iter() {
        let mut best = HashMap::new();
        let mut cur = v;
    
        let mut window = VecDeque::new();
        window.push_back(advance(&mut cur));
        window.push_back(advance(&mut cur));
        window.push_back(advance(&mut cur));
        window.push_back(advance(&mut cur));
        let low: i8 = (cur % 10) as i8;
        best.insert((window[0], window[1], window[2], window[3]), low);
        
        for _ in 0..2000 {
            window.pop_front();
            window.push_back(advance(&mut cur));
            let low: i8 = (cur % 10) as i8;
            if !best.contains_key(&(window[0], window[1], window[2], window[3])) {
                best.insert((window[0], window[1], window[2], window[3]), low);
            }
        }
        hash_maps.push(best);
    }
    let mut fullmap = HashMap::new();
    for map in hash_maps {
        for (k, v) in map {
            *(fullmap.entry(k).or_insert(0)) += v as i64;
        }
    }
    fullmap.values().copied().max().unwrap()
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input: String = String::from_utf8(buf)?;
    let _line_re: Regex = Regex::new(r"\r?\n")?;
    let input= _line_re.split(&_raw_input).map(|v| {
        v.parse::<i64>().unwrap()
    }).collect::<Vec<_>>();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));


    Ok(())
}
