use std::collections::HashSet;
use std::error::Error;
use std::io::stdin;
use std::io::Read;
use std::collections::HashMap;
use ::regex::Regex;

fn dfs(
    node: i32,
    graph: &HashMap<i32, Vec<i32>>,
    visited: &mut HashSet<i32>,
    order: &mut Vec<i32>,
) {
    let Some(neighbours) = graph.get(&node) else {
        order.push(node);
        return;
    };
    for &neigh in neighbours.iter() {
        if visited.insert(neigh) {
            dfs(neigh, graph, visited, order);
        }
    }
    order.push(node)
}

fn part_one(edges: &Vec<Vec<i32>>, orders: &Vec<Vec<i32>>) -> i32 {
    let graph: HashMap<i32, HashSet<i32>> = edges.iter().fold(HashMap::new(), |mut graph, e| {
        let f = e[1];
        let t = e[0];
        graph.entry(f).or_insert_with(||HashSet::new()).insert(t);
        graph
    });
    let mut result = 0;
    'outer: for order in orders.iter() {
        let mut disq = HashSet::new();
        for &val in order.iter() {
            if disq.contains(&val) {
                continue 'outer;
            }
            if let Some(neigh) = graph.get(&val) {
                disq.extend(neigh.iter().copied());
            }
        }
        result += order[order.len()/2];
    }
    result
}

fn part_two(edges: &Vec<Vec<i32>>, orders: &Vec<Vec<i32>>) -> i32 {
    let mut ingress: HashMap<i32, i32> = HashMap::new();
    let graph: HashMap<i32, HashSet<i32>> = edges.iter().fold(HashMap::new(), |mut graph, e| {
        let f = e[1];
        let t = e[0];
        graph.entry(f).or_insert_with(||HashSet::new()).insert(t);
        *(ingress.entry(t).or_insert(0)) += 1;
        graph
    });
    let mut to_fix = vec![];
    'outer: for order in orders.iter() {
        let mut disq = HashSet::new();
        for &val in order.iter() {
            if disq.contains(&val) {
                to_fix.push(order.clone());
                continue 'outer;
            }
            if let Some(neigh) = graph.get(&val) {
                disq.extend(neigh.iter().copied());
            }
        }
    }
    let mut result = 0;
    for order in to_fix.iter() {
        let order_set = order.iter().copied().collect::<HashSet<_>>();
        let mut ingress: HashMap<i32, i32> = HashMap::new();
        let mut subgraph = HashMap::new();
        for &v in order.iter() {
            subgraph.insert(
                v, 
                graph
                    .get(&v)
                    .map_or(vec![], |n| 
                        order_set.intersection(&n)
                            .copied()
                            .inspect(|&n| {
                                *(ingress.entry(n).or_insert(0)) += 1;
                            })
                            .collect())
                    );
        }
        let mut new_order = vec![];
        let mut visited = HashSet::new();
        for &v in order.iter() {
            if ingress.get(&v).map_or(true, |&v| v == 0) {
                dfs(v, &subgraph, &mut visited, &mut new_order);
            }
        }
        result += new_order[new_order.len()/2];
    }
    result
    
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer: Vec<u8> = vec![];
    stdin().read_to_end(&mut buffer)?;
    let _raw_input = String::from_utf8(buffer)?;
    let part_split = Regex::new(r"\r?\n\r?\n")?;
    let line_split = Regex::new(r"\r?\n")?;
    let mut iter = part_split.split(&_raw_input);
    let raw_edges = iter.next().unwrap();
    let edges = line_split.split(raw_edges).map(|e|{
        e.split("|").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let raw_orders=  iter.next().unwrap();
    let orders = line_split.split(raw_orders).map(|order|{
        order.split(r",").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    println!("Part 1: {}", part_one(&edges, &orders));
    println!("Part 2: {}", part_two(&edges, &orders));
    
    Ok(())
}
 