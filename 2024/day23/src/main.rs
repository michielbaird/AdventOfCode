use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io::stdin;
use std::io::Read;

fn part_one(edges: &Vec<(String, String)>) -> i64 {
    let mut graph= HashMap::new();
    let mut answers = HashSet::new();
    for (f, t) in edges.iter() {
        graph.entry(f.as_str()).or_insert_with(|| HashSet::new()).insert(t.as_str());
        graph.entry(t.as_str()).or_insert_with(|| HashSet::new()).insert(f.as_str());

        
        let check1 = f.starts_with('t');
        let check2: bool = t.starts_with('t');
        let e1 =graph.get(&t.as_str()).unwrap();
        let e2 = graph.get(&f.as_str()).unwrap();
        
        for &n in e1.intersection(&e2) {
            if n.starts_with('t') || check1 || check2 {
                answers.insert((f.as_str(), t.as_str(), n));
            }
        }
    }
    answers.len() as i64 
}

fn part_two(edges: &Vec<(String, String)>) -> String {
    let mut graph= HashMap::new();
    let mut start: HashSet<Vec<&str>> = HashSet::new();
    
    for (f, t) in edges.iter() {
        graph.entry(f.as_str()).or_insert_with(|| HashSet::new()).insert(t.as_str());
        graph.entry(t.as_str()).or_insert_with(|| HashSet::new()).insert(f.as_str());

        let e1 =graph.get(&t.as_str()).unwrap();
        let e2 = graph.get(&f.as_str()).unwrap();
            
        for &n in e1.intersection(&e2) {
            let mut to_sort: Vec<&str> = vec![f.as_str(), t.as_str(), n];
            to_sort.sort();
            start.insert(to_sort);
        }
    }
    let mut prev = HashSet::new();
    while !start.is_empty() {
        let mut next = HashSet::new();
        for group in start.iter() {
            let mut cur: HashSet<&str> = graph.get(group[0]).unwrap().clone();
            for i in 1..(group.len()) {
                let mut val = cur.intersection(&graph.get(group[i]).unwrap()).into_iter().copied().collect::<HashSet<_>>();
                cur = val;
            }
            for v in cur {
                let mut to_add = group.clone();
                to_add.push(v);
                to_add.sort();
                next.insert(to_add);
            }
            
        }
        prev = start;
        start = next;
    }

    let mut nodes = prev.into_iter().next().unwrap();
    nodes.sort();
    nodes.join(",")
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input: String = String::from_utf8(buf)?;
    let _line_re: Regex = Regex::new(r"\r?\n")?;
    let edges = _line_re.split(&_raw_input).map(|line| {
        let mut nodes = line.split('-').collect::<Vec<_>>();
        (nodes[0].to_string(), nodes[1].to_string())
    }).collect::<Vec<_>>();
    println!("Part 1: {}", part_one(&edges));
    println!("Part 2: {}", part_two(&edges));

    Ok(())
}
