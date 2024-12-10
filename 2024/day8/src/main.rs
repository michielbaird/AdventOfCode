use std::collections::HashSet;
use std::error::Error;
use std::io::stdin;
use std::io::Read;
use ::regex::Regex;
use std::collections::HashMap;
use gcd::Gcd;

fn add_antinodes(
    n: i32, m: i32,
    positions: &mut Vec<(i32, i32)>, 
    new: (i32, i32), 
    antinodes: &mut HashSet<(i32, i32)>
) {
    for &(p_row, p_col) in positions.iter( ) {
        let dr = new.0 - p_row;
        let dc: i32 = new.1 - p_col;
        let candidates = vec![(new.0 + dr, new.1 + dc), (p_row - dr, p_col - dc)];
        //println!("{:?},  ({}, {}) {:?}", new, p_row, p_col, candidates);
        for &c in candidates.iter() {
            if (0..n).contains(&c.0) && (0..m).contains(&c.1) {
                antinodes.insert(c);
            }
        }
    }
    positions.push(new);

}

fn add_antinodes2(
    n: i32, m: i32,
    positions: &mut Vec<(i32, i32)>, 
    new: (i32, i32), 
    antinodes: &mut HashSet<(i32, i32)>
) {
    for &(p_row, p_col) in positions.iter( ) {
        let dr = new.0 - p_row;
        let dc: i32 = new.1 - p_col;
        let d = (dr.abs() as u32).gcd(dc.abs() as u32) as i32;
        let dr = dr / d;
        let dc = dc / d;

        let mut nr = new.0;
        let mut nc = new.1;
        while (0..n).contains(&nr) && (0..m).contains(&nc) { 
            antinodes.insert((nr, nc));
            nr += dr;
            nc += dc;
        }
        nr = p_row;
        nc = p_col;
        while (0..n).contains(&nr) && (0..m).contains(&nc) { 
            antinodes.insert((nr, nc));
            nr -= dr;
            nc -= dc;
        }
    }

    positions.push(new);

}

fn part_one_or_2(input: &Vec<Vec<char>>, part: i32) -> i32 {
    let mut positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes = HashSet::new();
    let n = input.len();
    let m = input.get(0).map_or(0, |v| v.len());
    for i in 0..n {
        for j in 0..m {
            let value = input[i][j]; 
            if value != '.' {
                //println!("{}", value);
                let entry = positions.entry(value).or_insert_with(|| vec![]);
                if part == 1 {
                    add_antinodes(n as i32, m as i32, entry, (i as i32, j as i32 ), &mut antinodes); 
                } else {
                    add_antinodes2(n as i32, m as i32, entry, (i as i32, j as i32 ), &mut antinodes);
                }
            }

        }
    } 
    // let mut grid  = input.clone();
    // for p in antinodes.iter() {
    //     grid[p.0 as usize][p.1 as usize] = '#'
    // }
    // println!("");

    // for line in grid.iter() {
    //     let s = line.iter().copied().collect::<String>();
    //     println!("{}", s);
    // }
    antinodes.len() as i32
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input = String::from_utf8(buf)?;
    let _line_re = Regex::new(r"\r?\n")?;
    let input: Vec<Vec<char>> = _line_re.split(&_raw_input)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();




    println!("Part One: {}", part_one_or_2(&input, 1));
    println!("Part Two: {}", part_one_or_2(&input, 2));

    Ok(())
}
