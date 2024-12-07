use std::error::Error;
use std::io::stdin;
use std::io::Read;
use ::regex::Regex;

fn part_one(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    fn solve(idx: usize, nums: &Vec<i64>, cur: i64, target: i64) -> bool {
        if idx == nums.len() {
            return cur == target;
        }
        let plus = if let Some(r) = cur.checked_add(nums[idx]) {
            solve(idx + 1, nums, r, target)
        } else {false};
        let mul = if let Some(r) = cur.checked_mul(nums[idx]) {
            solve(idx + 1, nums, r, target)
        } else {false};
        plus || mul
    }
    input.iter().filter_map(|(v, nums)| {
        let v = *v;
        if solve(1, &nums, nums[0], v) {
            Some(v)
        } else {
            None
        }
    }).sum()
}

fn part_two(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    fn solve(idx: usize, nums: &Vec<i64>, cur: i64, target: i64) -> bool {
        if idx == nums.len() {
            return cur == target;
        }
        let plus = if let Some(r) = cur.checked_add(nums[idx]) {
            solve(idx + 1, nums, r, target)
        } else {false};
        let mul = if let Some(r) = cur.checked_mul(nums[idx]) {
            solve(idx + 1, nums, r, target)
        } else {false};
        let mut next_s = cur.to_string();
        next_s.push_str(&(nums[idx].to_string()));
        let con: bool =  if let Ok(next) = next_s.parse::<i64>() {
            solve(idx + 1, nums, next, target)
        } else {false};
        plus || mul || con
    }
    input.iter().filter_map(|(v, nums)| {
        let v = *v;
        if solve(1, &nums, nums[0], v) {
            Some(v)
        } else {
            None
        }
    }).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input = String::from_utf8(buf)?;
    let _line_split = Regex::new(r"\r?\n")?;
    let _part_split = Regex::new(r"(\d+):\s+(.*)$")?;
    let input = _line_split.split(&_raw_input).map(|line|{
        let  parts = _part_split.captures(line).unwrap();
        let target = parts[1].parse::<i64>().unwrap();
        let rest = &parts[2];
        let nums: Vec<i64> = rest.split(' ').map(|v| v.parse::<i64>().unwrap()).collect::<Vec<_>>();
        (target, nums)
    }).collect::<Vec<_>>();




    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    

    Ok(())
}
