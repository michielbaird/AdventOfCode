use ::regex::Regex;
use std::collections::HashSet;
use std::io::stdin;
use std::io::Read;
use std::error::Error;
use std::collections::HashMap;

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    let n = map.len();
    let m = map[0].len();
    for r in 0..n {
        for c in 0..m {
            if map[r][c] =='@' {
                return (r, c);
            }
        }
    }
    (0, 0)
}

fn part_one(map: &Vec<Vec<char>>, movements: &Vec<Vec<char>>) -> i64 {
    let mut map = map.clone();
    let mut position = find_start(&map);
    //println!("{:?}", position);
    for &mv in movements.iter().map(|v| v.iter()).flatten() {
        let (dr, dc) = match mv {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            _ => (0, -1)
        };
        let nr = (position.0 as i32 + dr) as usize;
        let nc = (position.1 as i32 + dc) as usize;
        let mut cr = nr;
        let mut cc = nc;
        while map[cr][cc] == 'O' {
            cr = (cr as i32 + dr) as usize;
            cc = (cc as i32 + dc) as usize;
        }
        if map[cr][cc] == '#' { continue; }
        map[cr][cc] = 'O';
        map[nr][nc] = '@';
        map[position.0][position.1] = '.';
        position.0 = nr;
        position.1 = nc;
    }

    let mut result = 0;
    for (r, row) in map.iter().enumerate() {
        for (c, &e) in row.iter().enumerate() {
            if e == 'O' {
                result += (r*100 + c) as i64;
            }
        }
    }
    for line in map.iter() {
        println!("{}", line.iter().copied().collect::<String>());
    }
    result
}

fn print_map(
    width: usize,
    height: usize,
    boxes: &Vec<(usize, usize)>,
    walls: &HashSet<(usize, usize)>,
    robot: (usize, usize),
) {
    let mut grid = vec![vec!['.'; width]; height];
    for &(r, c) in walls.iter() {
        grid[r][c] = '#';
    }
    for &(r, c) in boxes.iter() {
        grid[r][c] = '[';
        grid[r][c + 1] = ']';
    }
    grid[robot.0][robot.1] = '@';
    println!();

    for line in grid {
        println!("{}", line.iter().copied().collect::<String>())
    }

}

fn part_two(
    start: (usize, usize), 
    walls: &HashSet<(usize, usize)>,
    positions: &mut HashMap<(usize, usize), usize>,
    boxes: &mut Vec<(usize, usize)>,
    movements: &Vec<Vec<char>>,
    width: usize,
    height: usize,
) -> i64 {
    let mut row = start.0;
    let mut col: usize = start.1;

    'outer: for &mv in movements.iter().map(|v| v.iter()).flatten() {
        let (dr, dc) = match mv {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            _ => (0, -1)
        };
        let nr = (row as i32 + dr) as usize;
        let nc = (col as i32 + dc) as usize;
        if walls.contains(&(nr, nc)) {
            continue;
        } else if !positions.contains_key(&(nr, nc)) {
            row = nr;
            col = nc;
            continue 'outer;
        }
        let &box_id = positions.get(&(nr, nc)).unwrap();
        let mut moved_boxes = HashSet::new();
        moved_boxes.insert(box_id);
        let mut que = vec![box_id];
        while !que.is_empty() {
            let mut next = vec![];
            while let Some(box_id) = que.pop() {
                let (box_row, box_col) = boxes[box_id];
                if dc != 0 {
                    let col_check = if dc == 1 { box_col + 2} else { box_col - 1};
                    if walls.contains(&(box_row, col_check)) {
                        continue 'outer;
                    };
                    if let Some(&next_box_id) = positions.get(&(box_row, col_check)) {
                        moved_boxes.insert(next_box_id);
                        next.push(next_box_id);
                    }
                    continue;
                }
                let row_check = if dr == 1 { box_row + 1} else { box_row - 1};
                for (check_row, check_col) in [(row_check, box_col), (row_check, box_col + 1)] {
                    if walls.contains(&(check_row, check_col)) {
                        continue 'outer; //No movement
                    }
                    if let Some(&next_box_id) = positions.get(&(check_row, check_col)) {
                        if moved_boxes.insert(next_box_id) {
                            next.push(next_box_id);
                        }
                    }
                }
            
            }
            que = next;
        }
        moved_boxes.iter().for_each(|&box_id| {
            let (box_row, box_col) = boxes[box_id];
            positions.remove(&(box_row, box_col));
            positions.remove(&(box_row, box_col + 1));
            boxes[box_id] = ((box_row as i32 + dr) as usize, (box_col as i32 + dc) as usize);
        });
        moved_boxes.iter().for_each(|&box_id| {
            let (box_row, box_col) = boxes[box_id];
            positions.insert((box_row, box_col), box_id);
            positions.insert((box_row, box_col + 1), box_id);
        });
        row = nr;
        col = nc;
        //println!("{} {} {}", row, col, mv);
    }
    let mut result = 0;
    print_map(width, height, &boxes, walls, (row, col));

    for &(box_row, box_col) in boxes.iter() {
        result += (box_row*100 + box_col) as i64;
    }

    result
}


fn main() -> Result<(), Box<dyn Error>> {
    let part_re = Regex::new(r"\r?\n\r?\n")?;
    let line_re = Regex::new(r"\r?\n")?;
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let raw_input = String::from_utf8(buf)?;
    let mut parts = part_re.split(&raw_input);
    let map = line_re.split(parts.next().unwrap()).map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let movements: Vec<Vec<char>> = line_re.split(parts.next().unwrap())
        .map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut boxes = vec![];
    let mut walls = HashSet::new();
    let mut positions = HashMap::new();
    let mut start = (0,0);
    let width = map[0].len() * 2;
    let height = map.len();
    for r in 0..height {
        for c in 0..(width/2) {
            if map[r][c] == 'O' {
                let id = boxes.len();
                boxes.push((r, c*2));
                positions.insert((r, c*2), id);
                positions.insert((r, c*2 + 1), id);
            } else if map[r][c] == '#' {
                walls.insert((r, c*2));
                walls.insert((r, c*2 + 1));
            } else if map[r][c] == '@' {
                start = (r, c*2);
            }
        }
    }

    println!("Part 1: {}", part_one(&map, &movements));
    println!("Part 2: {}", part_two(start, &walls, &mut positions, &mut boxes, &movements, width, height));


    Ok(())
}
