use std::char;
use std::error::Error;
use std::i32;
use std::io::stdin;
use std::io::Read;
use std::vec;
use regex::Regex;
use std::collections::VecDeque;


const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn get_pos(maze: &Vec<Vec<char>>, _c: char) -> (usize, usize) {
    let n = maze.len();
    let m = maze[0].len();
    for r in 0..n {
        for c in 0..m {
            if maze[r][c] == _c {
                return (r, c);
            }
        }
    }
    (0,0)
}

fn bfs(
    map: &Vec<Vec<char>>, 
    map2: &mut Vec<Vec<i32>>,
    start: (usize, usize), 
    end: (usize, usize),
) -> i32 {
    let rows = map.len();
    let cols = map.get(0).map_or(0,|v| v.len());
    let mut que = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    visited[start.0][start.1] = true;
    que.push_back((0, start.0, start.1));
    let mut fastest = i32::MAX;

    while let Some((w, row, col)) = que.pop_front() {
        map2[row][col] = w;
        if row == end.0 && col == end.1 {
            fastest = w;
        }
        for &(dr, dc) in DELTAS.iter() {
            let nr = (row as i32 + dr) as usize;
            let nc = (col as i32 + dc) as usize;
            if  map.get(nr).and_then(|v| v.get(nc)).map_or('#', |&e|e) != '#' {
                if !visited[nr][nc] {
                    visited[nr][nc] = true;
                    que.push_back((w + 1, nr, nc));
                }
            }
        }
    }
    fastest
} 

fn part_one_two(map: &Vec<Vec<char>>, deltas: &Vec<(i32, i32)>) -> i64 {
    let rows = map.len();
    let cols = map.get(0).map_or(0,|v| v.len());
    let start = get_pos(&map, 'S');
    let end = get_pos(&map, 'E');
    let mut map2 = vec![vec![i32::MAX/8; cols]; rows];
    let mut map3 = vec![vec![i32::MAX/8; cols]; rows];

    let fastest = bfs(map, &mut map2, end, start);
    bfs(map, &mut map3, start, end);

    let mut answer = 0;
    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] != '#' {
                for &(dr, dc) in deltas.iter() {
                    let nnr = (r as i32 + dr) as usize;
                    let nnc = (c as i32 + dc) as usize;
                    if map.get(nnr).and_then(|v| v.get(nnc)).map_or('#', |&e|e) != '#' {
                        let cost = map3[nnr][nnc] + map2[r][c] + r.abs_diff(nnr) as i32 + c.abs_diff(nnc) as i32;
                        let savings = fastest - cost;
                        if savings >= 100 {
                            answer += 1;
                        } 
                    }
                }
            }
        }
    }
    answer
}




fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input: String = String::from_utf8(buf)?;
    let _line_re: Regex = Regex::new(r"\r?\n")?;
    let _part_re: Regex = Regex::new(r"\r?\n\r?\n")?;

    let map: Vec<Vec<char>> = _line_re.split(&_raw_input).map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let part_one_deltas =  vec![(2, 0), (0, 2), (-2, 0), (0, -2)];
    println!("Part 1: {}", part_one_two(&map, &part_one_deltas));
    let part2_delta = (-20..=20)
        .map(|dr| (-20..=20).map(move |dc|{
            (dr, dc)
        }))
        .flatten()
        .filter(|(dr, dc): &(i32, i32)| dr.abs() + dc.abs() <= 20)
        .collect::<Vec<_>>();
    
    println!("Part 2: {}", part_one_two(&map, &part2_delta));


    Ok(())
}
