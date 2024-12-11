use ::regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::io::stdin;
use std::io::Read;
use std::collections::HashMap;


fn expand(map: &Vec<Vec<i32>>) -> HashMap<i32, Vec<(usize, usize)>> {
    let n = map.len();
    let m = map.get(0).map_or(0, |v| v.len());
    (0..n).map(|row| {
        (0..m).map(move |column| {
            (row, column)
        })
    }).flatten().fold(HashMap::new(), |mut result,  (r, c)| {
        result.entry(map[r][c]).or_insert_with(|| vec![]).push((r, c));
        result
    })
}

fn bfs(
    map: &Vec<Vec<i32>>, 
    values:&mut Vec<Vec<i32>>,
    r: usize, 
    c: usize
) {
    let mut que = HashSet::new();
    que.insert((r, c));
    for v in (1..10).rev() {
        let mut next = HashSet::new();
        for (r, c) in que.into_iter() {
            for &(dr, dc) in DELTAS.iter() {
                let nr = (r as i32 + dr) as usize;
                let nc = (c as i32 + dc) as usize;
                if map.get(nr).and_then(|v|v.get(nc)).map_or(false, |&e| e == v - 1) {
                    next.insert((nr, nc));
                }
            }
        }
        //println!("{:?}", next);
        que = next;
    }
    for (r, c) in que.into_iter() {
        values[r][c] += 1;
    }
}

const DELTAS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
fn part_one(map: &Vec<Vec<i32>>) -> i64 {
    let n = map.len();
    let m = map.get(0).map_or(0, |v| v.len());
    let reverse = expand(map);
    let mut values = vec![vec![0; m]; n];
    reverse.get(&9).unwrap().iter().for_each(|&(r,c)| {
        bfs(map, &mut values, r, c);
    });

    
    //println!("{:?} {:?}", values, reverse);
    reverse.get(&0).unwrap().iter().map(|&(r,c)| {
        values[r][c] as i64
    }).sum::<i64>()
}

fn part_two(map: &Vec<Vec<i32>>) -> i64 {
    let n = map.len();
    let m = map.get(0).map_or(0, |v| v.len());
    let reverse = expand(map);
    let mut values = vec![vec![0; m]; n];
    reverse.get(&9).unwrap().iter().for_each(|&(r,c)| {
        values[r][c] = 1;
    });
    for v in (0..9).rev() {
        reverse.get(&v).unwrap().iter().for_each(|&(r,c)| {
            for &(dr, dc) in DELTAS.iter() {
                let nr = (r as i32 + dr) as usize;
                let nc = (c as i32 + dc) as usize;
                if map.get(nr).and_then(|v|v.get(nc)).map_or(false, |&e| e == v + 1) {
                    values[r][c] += values[nr][nc];
                }
            }
        });
    }


    //println!("{:?} {:?}", values, reverse);
    reverse.get(&0).unwrap().iter().map(|&(r,c)| {
        values[r][c] as i64
    }).sum::<i64>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _input = String::from_utf8(buf)?;
    let _line_re = Regex::new(r"\r?\n")?;
    let input: Vec<Vec<i32>> = _line_re.split(&_input).map(|line| {
        line.as_bytes().iter().map(|&c| (c - b'0') as i32).collect()
    }).collect();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));


    Ok(())
}
