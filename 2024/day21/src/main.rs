use std::collections::HashMap;
use std::error::Error;
use std::i32;
use std::i64;
use std::iter;
use std::usize;
use regex::Regex;
use std::io::stdin;
use std::io::Read;

struct PTrack {
    track: Vec<(i32, i32)>,
    cache: HashMap<(i32, i32, i32, i32), Vec<String>>
}

impl PTrack {
    fn new(to_add: &Vec<&str>) -> Self {
        let mut track = vec![(-1,-1); 256];
        for (i, &s) in to_add.iter().enumerate() {
            for (j, c) in s.chars().enumerate() {
                track[c as usize] = (i as i32, j as i32);
            }
        }
        Self {
            track: track,
            cache: HashMap::new(),
        }
    }
    fn get_pos(&self, val: char) -> (i32, i32) {
        self.track[val as usize]
    }
    fn get_pos2(&mut self, from: char, to: char) -> Vec<String> {
        let from = self.get_pos(from);
        let to = self.get_pos(to);
        self.get_possibilities(from, to).clone()
    }
    fn get_possibilities_expensive(&self, from: (i32, i32), to: (i32, i32)) -> Vec<String> {
        let (dr, dc) = (from.0 - to.0, from.1 - to.1);
        let h = if dc < 0 {'>'} else { '<' };
        let v = if dr < 0 {'v'} else { '^' };
        let horizontal = iter::repeat(h).take(dc.abs() as usize).collect::<String>();
        let vertical = iter::repeat(v).take(dr.abs() as usize).collect::<String>();
        if dr == 0 || dc == 0 {
            vec![
                format!("{}{}A", horizontal, vertical)
            ]
        } else {
            vec![
                format!("{}{}A", horizontal, vertical),
                format!("{}{}A", vertical, horizontal)
            ]
        }.into_iter().filter(|s| {
            let mut pos = from;
            let mut candidate = true;
            for v in s.chars() {
                pos = match v {
                    '<' => (pos.0, pos.1 - 1),
                    '^' => (pos.0 - 1, pos.1),
                    '>' => (pos.0, pos.1 + 1),
                    'v' => (pos.0 + 1, pos.1),
                    _ => (pos.0, pos.1),
                };
                if self.track['*' as usize] == pos {
                    candidate = false;
                    break;
                }
            }
            candidate
        }).collect()
    }

    fn get_possibilities(&mut self, from: (i32, i32), to: (i32, i32)) -> Vec<String> {
        if let Some(r) = self.cache.get(&(from.0, from.1, to.0, to.1)) {
            return r.clone();
        }
        let r = self.get_possibilities_expensive(from, to);
        self.cache.insert((from.0, from.1, to.0, to.1), r.clone());
        r
    }
}

//789
//456
//123
///0A
 
// ^A
//<v>


fn print_get_options(
        pos: (i32, i32),
        value: &str, 
        track: &mut PTrack
    ) -> Vec<String> {

    fn backtrack(
        index: usize,
        pos: (i32, i32),
        track: &mut PTrack,
        value: &Vec<char>,
        cur: String,
        result: &mut Vec<String>
    ) {
        if index == value.len() {
            result.push(cur);
            return;
        }
        let to = track.get_pos(value[index]);
        for m in track.get_possibilities(pos, to) {
            let mut next =  cur.clone();
            next.push_str(&m);
            backtrack(index + 1, to, track, value, next, result);
        }
    }
    
    let value = value.chars().collect::<Vec<_>>();
    let mut result = vec![];
    backtrack(0, pos, track, &value, String::from(""), &mut result);
    result

}

fn part_one(input: &Vec<&str>) -> i64 {
    let keypad =  PTrack::new(&vec![
        "789",
        "456",
        "123",
        "*0A",
    ]);
    let robot = PTrack::new(&vec![
        "*^A",
        "<v>",
    ]);
    let mut tracks = vec![
        keypad,
        robot,
    ];
    let mut answer = 0;
    for &line in input.iter() {
        let next = print_get_options((3, 2), line, &mut tracks[0]);
        let mut robots1: Vec<(usize, String)> = vec![];
        for (i,r2) in next.iter().enumerate() {
            let p2 = print_get_options((0, 2), r2, &mut tracks[1]);
            robots1.extend(p2.into_iter().map(|v| (i, v)));
        }
        let mut robots2: Vec<(usize, String)> = vec![];
        for (i, (_, r3)) in robots1.iter().enumerate() {
            let p3 = print_get_options((0, 2), r3, &mut tracks[1]);
            robots2.extend(p3.into_iter().map(|v| (i, v)));
        }
        let mut best = usize::MAX;
        for p in robots2.iter() {
            if p.1.len() < best {
                best = p.1.len();
            }
        }
        let line_value: i64 = line[..(line.len() - 1)].parse::<i64>().unwrap();
        answer += (best as i64)*line_value;
    }
    answer
}


fn solve_line(
    line: String,
    track: &mut PTrack,
    depth: i32,
    cache: &mut HashMap<(i32, String), i64> 
) -> i64 {
    if depth == 0 {
        return line.len() as i64;
    }
    if let Some(&value) = cache.get(&(depth, line.clone())) {
        return value;
    }
    let mut prev = 'A';
    let mut sum = 0;
    for c in line.chars() {
        let mut best = i64::MAX;
        for seq in track.get_pos2(prev, c) {
            let v = solve_line(seq, track, depth - 1, cache);
            best = best.min(v);
        }
        sum += best;
        prev = c;
    }
    cache.insert((depth, line), sum);
    sum
}
fn part_two(lines: &Vec<&str>, robots: i32) -> i64 {
    let mut keypad =  PTrack::new(&vec![
        "789",
        "456",
        "123",
        "*0A",
    ]);
    let mut robot = PTrack::new(&vec![
        "*^A",
        "<v>",
    ]);
    
    let mut result = 0;
    for line in lines.iter() {
        let mut best = i64::MAX;
        for path in print_get_options((3, 2), line, &mut keypad) {
            let mut cache: HashMap<(i32, String), i64> = HashMap::new();
            best = best.min(solve_line(path.clone(), &mut robot, robots, &mut cache));
        }
        let line_value: i64 = line[..(line.len() - 1)].parse::<i64>().unwrap();
        result += line_value*best;

    }
    // println!("{} {}", best, line_value);
    result

}


fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input: String = String::from_utf8(buf)?;
    let _line_re: Regex = Regex::new(r"\r?\n")?;
    let input = _line_re.split(&_raw_input).collect::<Vec<_>>();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input, 25));
    

    Ok(())
}
