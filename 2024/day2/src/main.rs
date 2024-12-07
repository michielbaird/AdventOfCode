use ::regex::Regex;
use std::io::stdin;
use std::io::Read;
use std::error::Error;

fn count_safe(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|line|{
        let first = line[0];
        let r = if line[0] > line[1] {
            line.iter().skip(1).try_fold(first, |prev, &cur| {
                let diff = prev - cur;
                if diff <= 3 && diff >= 1 {
                    Some(cur)
                } else {
                    None
                }
            }).is_some()
        } else {
            line.iter().skip(1).try_fold(first, |prev, &cur| {
                let diff = cur - prev;
                if diff <= 3 && diff >= 1 {
                    Some(cur)
                } else {
                    None
                }
            }).is_some()
        };
        if r { 1 } else { 0 }
    }).sum()
}


fn dp(row: &Vec<i32>) -> bool {
    let n = row.len();
    for &factor in [1, -1].iter() {
        let diff = (row[1]- row[0])*factor;
        let mut dp = vec![(true, false), (diff >= 1 && diff <= 3, true), (false, false)];
        
        for i in 2..n {
            let diff1 = (row[i]- row[i-1])*factor;
            let diff2 = (row[i] - row[i-2])*factor;
            dp[i.rem_euclid(3)].0 = dp[(i-1).rem_euclid(3)].0 && diff1 >= 1 && diff1 <= 3;
            dp[i.rem_euclid(3)].1 = (dp[(i-1).rem_euclid(3)].1 && diff1 >= 1 && diff1 <= 3) || 
                                    (dp[(i-2).rem_euclid(3)].0 && diff2 >= 1 && diff2 <= 3);
        }
        if dp[(n-1).rem_euclid(3)].0 || dp[(n-1).rem_euclid(3)].1 || dp[(n-2).rem_euclid(3)].0 {
            return true;
        }
    }
    false
}


fn count_safe2(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|line|{
        if dp(&line) {
            1
        } else {
            0
        }
    }).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let raw_input = String::from_utf8(buf)?;
    let input_s = raw_input.as_str();
    let line_split = Regex::new(r"\r?\n").unwrap();
    let input = line_split.split(input_s).map(|line| {
        line.split(' ').map(|num| {
            num.parse::<i32>().unwrap()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    println!("Safe 1: {}", count_safe(&input));
    println!("Safe 2: {}", count_safe2(&input));


    Ok(())
}
