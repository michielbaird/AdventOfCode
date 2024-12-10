use std::io::stdin;
use std::io::Read;
use std::error::Error;

fn part_two(input: &str) -> i64{
    let input_s = input.as_bytes();
    let n = input.len();
    let mut index = 0;
    let mut current: i64 = 0;
    let mut values = vec![];
    let mut empties = vec![];
    for i in (0..n).step_by(2) {
        let l = (input_s[i] - b'0') as i64;
        values.push((index, current, l));
        index += l;
        current += 1;
        if (i + 1) < input_s.len() {
            let l2 = (input_s[i + 1] - b'0') as i64;
            if l2 > 0 {
                empties.push((index, l2));
            }
            index += l2
        }
    }


    let mut moved = vec![];
    let mut final_positions = vec![];

    for back_index in (0..(values.len())).rev() {
        let mut empty_index = 0;
        while empty_index < empties.len() && values[back_index].0 > empties[empty_index].0 && empties[empty_index].1 < values[back_index].2 {
            empty_index += 1;
        }
        if empty_index < empties.len() && values[back_index].0 > empties[empty_index].0 && empties[empty_index].1 >= values[back_index].2 {
            moved.push((empties[empty_index].0, values[back_index].1, values[back_index].2));
            empties[empty_index].0 += values[back_index].2;
            empties[empty_index].1 -= values[back_index].2;
            values[back_index].2 = 0;
        }
    }
    moved.sort_by_key(|e| e.0);
    let mut v_iter = values.iter().copied().filter(|e| e.2 > 0).peekable();
    let mut m_iter = moved.iter().copied().peekable();
    while v_iter.peek().is_some() || m_iter.peek().is_some() {
        match (v_iter.peek(), m_iter.peek()) {
            (Some(&v), Some(&m)) if v.0 < m.0 => {
                v_iter.next();
                final_positions.push(v);
            },
            (Some(_),Some(&m)) => {
                m_iter.next();
                final_positions.push(m);
            },
            (None,Some(&m)) => {
                m_iter.next();
                final_positions.push(m);
            }
            (Some(&v), None) => {
                v_iter.next();
                final_positions.push(v);
            }
            _ => {}
        }
    }    

    
    let mut check_sum: i64 = 0;
    for &(s_idx, val, size) in final_positions.iter() {
        for idx in s_idx..(s_idx + size) {
            check_sum += val*idx;
        }
    }

    check_sum
}


fn part_one(input: &str) -> i64 {
    let input_s = input.as_bytes();
    let n = input.len();
    let mut index = 0;
    let mut current: i64 = 0;
    let mut values = vec![];
    let mut empties = vec![];
    for i in (0..n).step_by(2) {
        let l = (input_s[i] - b'0') as i64;
        values.push((index, current, l));
        index += l;
        current += 1;
        if (i + 1) < input_s.len() {
            let l2 = (input_s[i + 1] - b'0') as i64;
            if l2 > 0 {
                empties.push((index, l2));
            }
            index += l2
        }
    }
    let mut front_index = 0;
    let mut back_index = values.len() - 1;
    let mut empty_index = 0;
    let mut final_positions = vec![];
    loop {
        let (empty_pos, empty_size) = empties[empty_index]; //this gonna crash :)
        while front_index <= back_index && values[front_index].0 < empty_pos {
            final_positions.push(values[front_index]);
            front_index += 1;
        }
        if back_index < front_index { break; }
        let (_back_pos, file_idx, back_size ) = values[back_index];
        if back_size < empty_size {
            final_positions.push((empty_pos, file_idx, back_size));
            empties[empty_index].0 += back_size;
            empties[empty_index].1 -= back_size;
            back_index -= 1;
        } else if back_size > empty_size {
            final_positions.push((empty_pos, file_idx, empty_size));
            values[back_index].2 -= empty_size;
            empty_index += 1;
        } else {
            final_positions.push((empty_pos, file_idx, empty_size));
            empty_index += 1;
            back_index -= 1;
        }
    }
    let mut check_sum: i64 = 0;
    for &(s_idx, val, size) in final_positions.iter() {
        for idx in s_idx..(s_idx + size) {
            check_sum += val*idx;
        }
    }

    check_sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let input = String::from_utf8(buf)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));


    
    Ok(())
}
