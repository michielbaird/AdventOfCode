use std::error::Error;
use std::io::stdin;
use std::io::Read;
use regex::Regex;

fn part_one(towels: &Vec<&str>, designs: &Vec<&str>) -> i64 {
    let mut answer = 0;
    for &design in designs.iter() {
        let n = design.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            for &towel in towels.iter() {
                let m = towel.len();
                if m > i { continue;}
                if towel == &design[i-m..i] {
                    dp[i] |= dp[i-m];
                }
            }
        }

        answer += if dp[design.len()] {1} else {0};
    }
    answer

}

fn part_two(towels: &Vec<&str>, designs: &Vec<&str>) -> i64 {
    let mut answer = 0;
    for &design in designs.iter() {
        let n = design.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            for &towel in towels.iter() {
                let m = towel.len();
                if m > i { continue;}
                if towel == &design[i-m..i] {
                    dp[i] += dp[i-m];
                }
            }
        }

        answer += dp[design.len()];
    }
    answer

}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input: String = String::from_utf8(buf)?;
    let _line_re: Regex = Regex::new(r"\r?\n")?;
    let _part_re: Regex = Regex::new(r"\r?\n\r?\n")?;
    let _comma_split = Regex::new(r", ")?;


    let mut parts =_part_re.split(&_raw_input);
    let towels = _comma_split.split(parts.next().unwrap()).collect::<Vec<_>>();
    let designs = _line_re.split(parts.next().unwrap()).collect::<Vec<_>>();

    println!("Part 1: {}", part_one(&towels, &designs));
    println!("Part 2: {}", part_two(&towels, &designs));


    Ok(())
}
