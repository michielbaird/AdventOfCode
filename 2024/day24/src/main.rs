use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::io::stdin;
use std::io::Read;
use std::vec;


fn run_gates<'a>(input_vals: &Vec<(&'a str, bool)>, gate_map: &HashMap<&'a str, (&'a str, &'a str, &'a str)>) -> HashMap<&'a str, bool> {
    let mut ingress = HashMap::new();
    let mut reverse_map = HashMap::new();
    let mut results = HashMap::new();
    for (&g, &(_, a1, a2)) in gate_map.iter() {
        ingress.insert(g, 2);
        reverse_map.entry(a1).or_insert_with(|| vec![]).push(g);
        reverse_map.entry(a2).or_insert_with(|| vec![]).push(g);
    }
    let mut queue = VecDeque::new();
    for &(a1, val) in input_vals.iter() {
        results.insert(a1, val);
        for &g in reverse_map.get(&a1).unwrap_or(&Vec::default()).iter() {
            let ing = ingress.get_mut(&g).unwrap();
            *ing -= 1;
            if *ing == 0 {
                queue.push_back(g);
            } 
        }
    }
    while let Some(gate) =  queue.pop_front() {
        let &(gtype, a1_name, a2_name) = gate_map.get(&gate).unwrap();
        let &a1 = results.get(&a1_name).unwrap();
        let &a2 = results.get(&a2_name).unwrap();
        let val = match gtype {
            "OR" => a1 || a2,
            "XOR" => a1 ^ a2,
            _ => a1 && a2
        };
        results.insert(gate, val);
        for &g in reverse_map.get(&gate).unwrap_or(&Vec::default()).iter() {
            let ing = ingress.get_mut(&g).unwrap();
            *ing -= 1;
            if *ing == 0 {
                queue.push_back(g);
            } 
        }
    }
    results
}

fn extract_z(results: &HashMap<&str, bool>) -> i64 {
    let mut to_sort = results.iter().filter_map(|(&k, &v)| 
        if k.starts_with("z") {Some((k, if v {1} else {0}))} else {None} 
    ).collect::<Vec<_>>();
    to_sort.sort();
    let mut answer = 0;
    for &(_, v) in to_sort.iter().rev() {
        answer <<= 1;
        answer |= v;
    }
    answer    
}


fn part_one(input_vals: &Vec<(&str, bool)>, gates: &Vec<(String, String, String, String)>) -> i64 {
    let mut gate_map = HashMap::new();
    for (g, a1, a2, out) in gates.iter() {
        gate_map.insert(out.as_str(), (g.as_str(), a1.as_str(), a2.as_str()));
    }
    let results = run_gates(input_vals, &gate_map);
    extract_z(&results)    
}

struct Summer<'a> {
    inputs: HashMap<String, bool>,
    gates: HashMap<&'a str, (&'a str, &'a str, &'a str)>,
}

impl<'a> Summer<'a> {
    fn new(input_vals: &'a Vec<(&str, bool)>, gates: &'a Vec<(String, String, String, String)>) -> Self {
        let mut gate_map = HashMap::new();
        for (g, a1, a2, out) in gates.iter() {
            gate_map.insert(out.as_str(), (g.as_str(), a1.as_str(), a2.as_str()));
        }
        let to_start = input_vals.iter().map(|&(k, v)| (k.to_string(), v)).collect::<HashMap<_, bool>>();
        Self {
            inputs: to_start,
            gates: gate_map
        }
    }
    fn swap_gates(&mut self, key1: &'a str, key2: &'a str) {
        let value1 = self.gates.remove(key1).unwrap();
        let value2 = self.gates.remove(key2).unwrap();
        self.gates.insert(key1, value2);
        self.gates.insert(key2, value1);
    }

    fn sum(&mut self, x: i64, y: i64) -> i64 {
        for i in 0..45 {
            let bit_x = (x >> i) & 1 == 1;
            let bit_y = (y >> i) & 1 == 1;
            if let Some(x_ref) = self.inputs.get_mut(&format!("x{:02}", i)) {
                *x_ref = bit_x;
            }
            if let Some(y_ref) = self.inputs.get_mut(&format!("y{:02}", i)) {
                *y_ref = bit_y;
            }
        
        }
        let to_send: Vec<(&str, bool)> = self.inputs.iter().map(|(k, &v)| (k.as_str(), v)).collect();
        let results = run_gates(&to_send, &self.gates);
        extract_z(&results)
    }
}
fn run_tests(summer: &mut Summer, tests: &Vec<(i64, i64)>, bits: i64) -> bool {
    let mut all_pass = true;
    for &(a, b) in tests.iter() {
        let a = a & ((1 << bits)) - 1;
        let b = b & ((1 << bits)) - 1;
        let expected = a + b;
        let result: i64 = summer.sum(a, b);
        //println!("{:0b} + {:0b} ")
        if expected != result {
            all_pass = false;
            break;
        }
    }
    all_pass
}

fn part_two(input_vals: &Vec<(&str, bool)>, gates: &Vec<(String, String, String, String)>) -> String {
    let tests = vec![
        (0i64, (1i64 << 45) - 1),
        ((1i64 << 45) - 1, 0),
        (1i64, (1i64 << 44) - 1),
        ((1i64 << 44) - 1, 1),
        (5864062014805, 2932031007402),
        (2932031007402, 5864062014805)

    ];
    let mut summer = Summer::new(input_vals, gates);
    let mut swapped = vec![];
    let mut original = summer.gates.clone();
    let gates = summer.gates.keys().map(|&e|e).collect::<Vec<_>>();
    for bits in 0..=45 {
        println!("{} {:?}", bits, swapped);
        if !run_tests(&mut summer, &tests, bits) {
            'next_test: for i in 0..(gates.len()-1) {
                for j in (i+1)..(gates.len()) {
                    summer.swap_gates(gates[i], gates[j]);
                    if run_tests(&mut summer, &tests, bits + 1) {
                        swapped.push(gates[i]);
                        swapped.push(gates[j]);
                        //summer.swap_gates(gates[i], gates[j]);
                        break 'next_test;
                    } else {
                        summer.swap_gates(gates[i], gates[j]);
                    }
                }
            }
            if !run_tests(&mut summer, &tests, bits) {
                swapped.pop();
                swapped.pop();
            }
        }
    }
    let mut answer = original.iter().fold(vec![], |mut final_swap, (&k, v)| {
        if summer.gates.get(&k) != Some(v) {
            final_swap.push(k);
        }
        final_swap
    });
    println!("{:?} {:?}", swapped, answer);
    answer.sort();
    answer.join(",")
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input: String = String::from_utf8(buf)?;
    let _line_re: Regex = Regex::new(r"\r?\n")?;
    let _parts_re: Regex = Regex::new(r"\r?\n\r?\n")?;
    
    let mut parts = _parts_re.split(&_raw_input);
    let part1 = parts.next().unwrap();
    let part2 = parts.next().unwrap();
    let inputs_vals = _line_re.split(&part1).map(|line| {
        let mut p = line.split(": ");
        let name = p.next().unwrap();
        let val = p.next().unwrap() == "1";
        (name, val)
    }).collect::<Vec<_>>();
    let gate_re = Regex::new(r"([\d\w]+) (AND|OR|XOR) ([\d\w]+) -> ([\d\w]+)")?;
    let gates = _line_re.split(&part2).map(|gate | {
        let cap = gate_re.captures(gate).unwrap();
        let a = cap[1].to_string();
        let b = cap[3].to_string();
        let gate       = cap[2].to_string();
        let out  = cap[4].to_string();
        (gate, a, b, out)
    }).collect::<Vec<_>>();



    println!("Part 1: {}", part_one(&inputs_vals, &gates));
    println!("Part 2: {}", part_two(&inputs_vals, &gates));

    Ok(())
}
