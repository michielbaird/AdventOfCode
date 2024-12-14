
use ::regex::Regex;
use std::io::stdin;
use std::io::Read;
use std::error::Error;

fn part_one(input: &Vec<(i64, i64, i64, i64)>, t: i64, print_canvas: bool) -> i64 {
    let w = 101;
    let h = 103;
    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;
    let mut grid = vec![vec!['.'; w as usize]; h as usize];

    input.iter().for_each(|&(px, py, dx,dy)| {
        let x = (px + dx*t).rem_euclid(w);
        let y = (py + dy*t).rem_euclid(h);
        grid[y as usize][x as usize] = '#';
        if (0..(w/2)).contains(&x) && (0..(h/2)).contains(&y) {
            tl += 1;
        } else if (((w+1)/2)..w).contains(&x) && (0..(h/2)).contains(&y) {
            tr += 1;
        } else if (0..(w/2)).contains(&x) && (((h+1)/2)..h).contains(&y) {
            bl += 1;
        } else if (((w+1)/2)..w).contains(&x) && (((h+1)/2)..h).contains(&y) {
            br += 1;
        }
    });
    if print_canvas {
        for i in 0..(h as usize) {
            println!("{}", grid[i].iter().copied().collect::<String>())
        }
    }
    tl*tr*bl*br
}


const DELTAS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part_two(input: &Vec<(i64, i64, i64, i64)>) -> i64 {
    let w = 101;
    let h = 103;
    let mut best = 0;
    let mut best_t = 0;
    for t in 0..10000 {
        let mut grid = vec![vec!['.'; w as usize]; h as usize];
        let mut score = 0;
        input.iter().for_each(|&(px, py, dx,dy)| {
            let x = (px + dx*t).rem_euclid(w);
            let y = (py + dy*t).rem_euclid(h);
            for &(dx, dy) in DELTAS.iter() {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if grid.get(ny).and_then(|v| v.get(nx)).map_or(false, |&e| e  == '#') {
                    score += 1;
                }
            }
            grid[y as usize][x as usize] = '#';
        });
        if score > best {
            best = score;
            best_t = t;
        }
    }
    best_t as i64
}
fn main() -> Result<(), Box<dyn Error>> {
    let _line_re = Regex::new(r"\r?\n")?;

    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    // p=9,5 v=-3,-3
    let _input_re = Regex::new(r"p\=(\-?\d+),(\-?\d+) v\=(\-?\d+),(\-?\d+)")?;
    let _raw_input = String::from_utf8(buf)?;
    let input = _line_re.split(&_raw_input).map(|line| {
        let caps = _input_re.captures(line).unwrap();
        (
            caps[1].parse::<i64>().unwrap(),
            caps[2].parse::<i64>().unwrap(),
            caps[3].parse::<i64>().unwrap(),
            caps[4].parse::<i64>().unwrap(),
        )
    }).collect::<Vec<_>>();
    println!("[");
    for line in input.iter() {
        println!("[{},{},{},{}],", line.0, line.1, line.2, line.3);
    }
    println!("]");


    let part_1 = part_one(&input, 100, false);
    let part_2 = part_two(&input);
    //part_one(&input, part_2, true);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);


    Ok(())
}
