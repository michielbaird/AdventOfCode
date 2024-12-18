use std::collections::VecDeque;
use std::error::Error;
use std::io::stdin;
use std::io::Read;
use regex::Regex;



const ROWS: usize = 71;
const COLS: usize = 71;
const NANOS: usize = 1024;
const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn part_one(input: &Vec<(usize, usize)>) -> i64 {
    let mut map = vec![vec!['.'; COLS]; ROWS];
    for i in 0..NANOS {
        let (r,c) = input[i];
        map[r][c] = '#';
    }
    let mut que = VecDeque::new();
    que.push_back((0, 0, 0));
    map[0][0] = 'O';
    while let Some((w, row, col)) = que.pop_front() {
        for &(dr, dc) in DELTAS.iter() {
            let nr = (row as i32 + dr) as usize;
            let nc = (col as i32 + dc) as usize;
            if nr == ROWS -1 && nc == COLS -1 {
                return w + 1;
            }
            if map.get(nr).and_then(|v| v.get(nc)).map_or('#', |&e|e) == '.' {
                map[nr][nc] = 'O';
                que.push_back((w + 1, nr, nc));
            }
        }
    }
    0
}

fn find_parent(node: usize, parent: &mut Vec<usize>) -> usize {
    if parent[node] == node {
        return node;
    }
    let p = parent[node];
    let r = find_parent(p, parent);
    parent[node] = r;
    r
}
fn union_vals(node1: usize, node2: usize, parent: &mut Vec<usize>, rank: &mut Vec<usize>) {
    let mut p1 = find_parent(node1, parent);
    let mut p2 = find_parent(node2, parent);
    if p1 == p2 {
        return;
    }
    if rank[p1] == rank[p2] {
        rank[p1] += 1;
    } else if rank[p2] > rank[p1] {
        let tmp = p1;
        p1 = p2;
        p2 = tmp;
    }
    parent[p2] = p1;
}

fn part_two(input: &Vec<(usize, usize)>) -> String {
    let mut parent = (0..(ROWS*COLS)).collect::<Vec<_>>();
    let mut rank = vec![0; ROWS*COLS];
    let mut map = vec![vec!['.'; COLS]; ROWS];
    for &(r, c) in input {
        map[r][c] = '#';
    }
    for r in 0..ROWS {
        for c  in 0..COLS {
            if map[r][c] != '.' {
                continue;
            }
            for &(dr, dc) in DELTAS.iter() {
                let nr = (r as i32 + dr) as usize;
                let nc = (c as i32 + dc) as usize;
                if map.get(nr).and_then(|v| v.get(nc)).map_or('#', |&e|e) == '.' {
                    union_vals(r*COLS + c, nr*COLS + nc, &mut parent, &mut rank);
                }
            }
        }
    }
    let end_id = ROWS*COLS - 1;
    let start_id = 0;
    let mut t = input.len() - 1;
    while find_parent(start_id, &mut parent) != find_parent(end_id, &mut parent) {
        let (r, c) = input[t];
        map[r][c] = '.';
        for &(dr, dc) in DELTAS.iter() {
            let nr = (r as i32 + dr) as usize;
            let nc = (c as i32 + dc) as usize;
            if map.get(nr).and_then(|v| v.get(nc)).map_or('#', |&e|e) == '.' {
                union_vals(r*COLS + c, nr*COLS + nc, &mut parent, &mut rank);
            }
        }
        t -= 1;
    }
    format!("{},{}", input[t + 1].1, input[t+1].0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input = String::from_utf8(buf)?;
    let _line_re = Regex::new(r"\r?\n")?;
    let input = _line_re.split(&_raw_input)
            .map(|line|  {
                let mut coords = line.split(',');
                let c = coords.next().unwrap().parse::<usize>().unwrap();
                let r  = coords.next().unwrap().parse::<usize>().unwrap();
                (r, c)
            }).collect::<Vec<_>>();
                

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}
