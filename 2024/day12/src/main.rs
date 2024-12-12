use ::regex::Regex;
use std::io::stdin;
use std::io::Read;
use std::error::Error;

const  DELTAS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0 ), (0, -1)];

fn build_map2(map: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let n = map.len();
    let m = map.get(0).map_or(0, |v| v.len());
    let mut map2 = vec![vec![0; m]; n];
    for r in 0..n {
        for c in 0..m {
            let v = map[r][c];
            for (i,&(dr, dc)) in DELTAS.iter().enumerate() {
                let nr: usize = (r as i32 + dr) as usize;
                let nc: usize = (c as i32 + dc) as usize;
                if map.get(nr)
                    .and_then(|row| row.get(nc))
                    .map_or(true, |&e| e != v) {
                    map2[r][c] |= 1<<i;
                }
            } 
        }
    }
    map2
}

fn dfs(
    r: usize,
    c: usize,
    map: &Vec<Vec<char>>,
    map2: &Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    count: &mut i32,
    perimiter: &mut i32,
    sides: &mut i32, 

) {
    //println!("{} {}", r, c);
    visited[r][c] = true;
    let v = map[r][c];
    *count += 1;
    for (i,&(dr, dc)) in DELTAS.iter().enumerate() {
        let nr: usize = (r as i32 + dr) as usize;
        let nc: usize = (c as i32 + dc) as usize;
        let nnr = (r as i32 + DELTAS[(i + 1) % 4].0) as usize;
        let nnc: usize = (c as i32 + DELTAS[(i + 1) % 4].1) as usize;
        if map2[r][c] & (1 << i) != 0 && 
            (map.get(nnr)
                .and_then(|row| row.get(nnc))
                .map_or(true, |&e| e != v) || 
               map2.get(nnr)
                .and_then(|row| row.get(nnc)) 
                .map_or(true, |&e| e & (1 << i) == 0)) {
            *sides += 1;
        }
        
        if map.get(nr)
                .and_then(|row| row.get(nc))
                .map_or(true, |&e| e != v) {
            *perimiter += 1;
        } else if !visited.get(nr).and_then(|row| row.get(nc)).map_or(true, |&e| e) {
            dfs(nr, nc, map, map2, visited, count, perimiter, sides);
        }
    }

}

fn part_one(input: &Vec<Vec<char>>) -> i64 {
    let n = input.len();
    let m = input.get(0).map_or(0, |v| v.len());
    let mut visited = vec![vec![false; m]; n];
    let map2: Vec<Vec<i32>> = build_map2(&input);
    let mut result = 0;
    for r in 0..n {
        for c in 0..m {
            if !visited[r][c] {
                let mut count = 0;
                let mut per = 0;
                let mut sides = 0;
                dfs(r, c, input, &map2, &mut visited, &mut count, &mut per, &mut sides);
                result += (count as i64)*(per as i64);
            }
        }
    }
    result
}

fn part_two(input: &Vec<Vec<char>>) -> i64 {
    let n = input.len();
    let m = input.get(0).map_or(0, |v| v.len());
    let map2: Vec<Vec<i32>> = build_map2(&input);

    let mut visited = vec![vec![false; m]; n];
    let mut result = 0;
    for r in 0..n {
        for c in 0..m {
            if !visited[r][c] {
                let mut count = 0;
                let mut per = 0;
                let mut side = 0;
                dfs(r, c, input, &map2, &mut visited, &mut count, &mut per, &mut side);
                result += (count as i64)*(side as i64);
            }
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let _line_re = Regex::new(r"\r?\n")?;
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input = String::from_utf8(buf)?;

    let input = _line_re
        .split(&_raw_input)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}
