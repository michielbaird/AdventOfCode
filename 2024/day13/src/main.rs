use ndarray::{array, Array1, Array2};
use ndarray_linalg::Solve;

use ::regex::Regex;
use std::io::stdin;
use std::io::Read;
use std::error::Error;



#[derive(Debug)]
struct TestCase {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64), 
}


fn part_one(input: &Vec<TestCase>) -> i64 {
    let mut sum = 0;
    for t in input.iter() {
        let mut min = 9999;
        for i in 0..=100 {
            for j in 0..=100 {
                if t.button_a.0*i + t.button_b.0*j == t.prize.0 && 
                    t.button_a.1*i + t.button_b.1*j == t.prize.1 {
                        min = min.min(3*i + j);
                    }
            }
        } 
        if min != 9999 {
            sum += min
        }
    }
    sum
}

fn part_two(input: &Vec<TestCase>) -> i64 {
    let mut sum = 0;
    for tc in input.iter() {
        let a: Array2<f64> = array![
            [tc.button_a.0 as f64, tc.button_b.0 as f64], 
            [tc.button_a.1 as f64, tc.button_b.1 as f64]
        ];
        let b: Array1<f64> = array![tc.prize.0 as f64, tc.prize.1 as f64];
        let x = a.solve_into(b).unwrap();
        let b_a = x[0].round() as i64;
        let b_b = x[1].round() as i64;
        //println!("a: {}, b: {}", b_a, b_b);
        if tc.button_a.0*b_a + tc.button_b.0*b_b == tc.prize.0 &&
            tc.button_a.1*b_a + tc.button_b.1*b_b == tc.prize.1 {
            sum += 3*b_a + b_b;
        }
    }
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let _line_re = Regex::new(r"\r?\n")?;
    let _part_split = Regex::new(r"\r?\n\r?\n")?;

    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input = String::from_utf8(buf)?;
    let _b_regex = Regex::new(r"Button [AB]\: X\+(\d+), Y\+(\d+)")?;
    let _prize=  Regex::new(r"Prize\: X=(\d+), Y=(\d+)")?;
    let input = _part_split.split(&_raw_input).map(|part| {
        let mut iter = _line_re.split(&part);
        let b_a = _b_regex.captures(iter.next().unwrap()).unwrap();
        let b_b = _b_regex.captures(iter.next().unwrap()).unwrap();
        let prize = _prize.captures(iter.next().unwrap()).unwrap();
        TestCase {
            button_a: (b_a[1].parse::<i64>().unwrap(), b_a[2].parse::<i64>().unwrap()),
            button_b: (b_b[1].parse::<i64>().unwrap(), b_b[2].parse::<i64>().unwrap()),
            prize: (prize[1].parse::<i64>().unwrap(), prize[2].parse::<i64>().unwrap()),
        }
    }).collect::<Vec<_>>();
    //println!("{:?}", input);



    println!("Part 1: {}", part_one(&input));

    let input2 = input.iter().map(|tc| {
        TestCase {
            button_a: tc.button_a,
            button_b: tc.button_b,
            //prize: tc.prize,
            prize: (tc.prize.0 + 10000000000000, tc.prize.1 + 10000000000000),
        }
    }).collect::<Vec<_>>();
    println!("Part 2: {}", part_two(&input2));



    Ok(())
}
