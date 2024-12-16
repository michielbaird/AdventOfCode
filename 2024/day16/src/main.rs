use std::collections::HashSet;
use std::error::Error;
use std::i32;
use std::io::stdin;
use std::io::Read;
use regex::Regex;

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;


const DELTAS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn get_start(maze: &Vec<Vec<char>>) -> (usize, usize) {
    let n = maze.len();
    let m = maze[0].len();
    for r in 0..n {
        for c in 0..m {
            if maze[r][c] == 'S' {
                return (r, c);
            }
        }
    }
    (0,0)
}

fn part_one(maze: &Vec<Vec<char>>) -> i64 {
    let n = maze.len();
    let m = maze[0].len();
    let mut scores = HashMap::new();
    let mut visited = HashSet::new();
    let start = get_start(&maze);
    let mut p_queue = BinaryHeap::new();
    p_queue.push(Reverse( (0, 0, start.0, start.1) ));
    scores.insert((0, start.0, start.1), 0);
    while let Some(Reverse((w, dir, r ,c))) = p_queue.pop() {
        if !visited.insert((dir, r, c)) { continue;}
        if maze[r][c] == 'E' {
            return w;
        }
        let nr = (r as i32 + DELTAS[dir].0) as usize;
        let nc = (c as i32 + DELTAS[dir].1) as usize;

        if (0..n).contains(&nr) &&
                (0..m).contains(&nc) && 
                maze[nr][nc] != '#' &&
                !visited.contains(&(dir, nr, nc)) &&
                scores.get(&(dir, nr, nc)).map_or(true, |&e| e > w + 1) {
            scores.insert((dir, nr, nc), w + 1);
            p_queue.push(Reverse((w + 1, dir, nr, nc)));
        }
        if !visited.contains(&((dir + 1) % 4, r, c)) &
                scores.get(&( (dir + 1) % 4, r, c)).map_or(true, |&e| e > w + 1000) {
            scores.insert(((dir + 1) % 4, r, c), w + 1000);
            p_queue.push(Reverse((w + 1000, (dir + 1) % 4, r, c)));
        }
        if !visited.contains(&(dir.checked_sub(1).unwrap_or(3), r, c)) &
                scores.get(&(dir.checked_sub(1).unwrap_or(3), r, c)).map_or(true, |&e| e > w + 1000) {
            scores.insert((dir.checked_sub(1).unwrap_or(3), r, c), w + 1000);
            p_queue.push(Reverse((w + 1000, dir.checked_sub(1).unwrap_or(3), r, c)));
        }
    }
    -1
}

fn part_two(maze: &Vec<Vec<char>>) -> i64 {
    let n = maze.len();
    let m = maze[0].len();
    let mut scores = HashMap::new();
    let mut visited = HashSet::new();
    let mut prev = HashMap::new();
    let start = get_start(&maze);
    let mut p_queue = BinaryHeap::new();
    p_queue.push(Reverse( (0, 0, start.0, start.1, None) ));
    scores.insert((0, start.0, start.1), 0);
    let mut best = i32::MAX;
    let mut end_positions = HashSet::new();
    while let Some(Reverse((w, dir, r ,c, from))) = p_queue.pop() {
        if *(scores.get(&(dir, r, c)).unwrap()) == w {
            if let Some((prev_dir, prev_row, prev_col)) = from {
                prev.entry((dir, r, c)).or_insert_with(|| vec![]).push((prev_dir, prev_row, prev_col));
            }
        }
        if !visited.insert((dir, r, c)) { continue;}
        if maze[r][c] == 'E'  {
            best = best.min(w);
            if w == best {
                end_positions.insert((dir,r, c));
            }
            continue;
        }
        let nr = (r as i32 + DELTAS[dir].0) as usize;
        let nc = (c as i32 + DELTAS[dir].1) as usize;

        if (0..n).contains(&nr) &&
                (0..m).contains(&nc) && 
                maze[nr][nc] != '#' &&
                !visited.contains(&(dir, nr, nc)) {
            scores.insert((dir, nr, nc), w + 1);
            p_queue.push(Reverse((w + 1, dir, nr, nc, Some((dir, r, c)))));
        }
        if !visited.contains(&((dir + 1) % 4, r, c)) {
            scores.insert(((dir + 1) % 4, r, c), w + 1000);
            p_queue.push(Reverse((w + 1000, (dir + 1) % 4, r, c, Some((dir, r, c)) )));
        }
        if !visited.contains(&(dir.checked_sub(1).unwrap_or(3), r, c)) {
            scores.insert((dir.checked_sub(1).unwrap_or(3), r, c), w + 1000);
            p_queue.push(Reverse((w + 1000, dir.checked_sub(1).unwrap_or(3), r, c, Some((dir, r, c)) )));
        }
    }
    let mut unique = HashSet::new();
    let mut current = end_positions.clone();
    while !current.is_empty() {
        let mut next = HashSet::new();
        for &(dir, r, c) in current.iter() {
            unique.insert((r, c));
            for &n in prev.get(&(dir, r, c)).unwrap_or(&vec![]) {
                next.insert(n);
            }
        }
        current = next;
    }

    unique.len() as i64
}

fn main() -> Result<(), Box<dyn Error>> {
    let line_re = Regex::new(r"\r?\n")?;
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let raw_input = String::from_utf8(buf)?;

    let maze = line_re.split(&raw_input).map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();



    println!("Part 1: {}", part_one(&maze));
    println!("Part 2: {}", part_two(&maze));


    Ok(())
}
