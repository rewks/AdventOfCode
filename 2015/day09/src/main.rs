use std::collections::HashMap;
use itertools::Itertools;

struct Edge {
    node_a: String,
    node_b: String,
    distance: u32,
}

// https://en.wikipedia.org/wiki/Held%E2%80%93Karp_algorithm
fn held_karp(matrix: Vec<Vec<u32>>, n: usize) -> u32 {
    let mut C = HashMap::new();

    for k in 1..n {
        C.insert((1 << k, k), (matrix[0][k], 0 as usize));
    }

    for subset_size in 2..n {
        for subset in (1..n).combinations(subset_size) {
            let mut bits = 0;
            for bit in subset.clone() {
                bits |= 1 << bit;
            }

            for k in subset.clone() {
                let prev = bits & !(1 << k);

                let mut res: Vec<(u32, usize)> = Vec::new();
                for m in subset.clone() {
                    if m == 0 || m == k {
                        continue;
                    }
                    let (t, z) = *C.get(&(prev, m)).unwrap();
                    res.push((t + matrix[m][k], m));
                }
                C.insert((bits, k), *res.iter().min().unwrap()); // change to max for part 2
            }
        }
    }

    let bits = (2u32.pow(n.try_into().unwrap()) - 1) - 1;

    let mut res: Vec<(u32, usize)> = Vec::new();
    for k in 1..n {
        let (t, z) = *C.get(&(bits, k)).unwrap();
        res.push((t + matrix[k][0], k));
    }

    let (r, p) = *res.iter().min().unwrap(); // change to max for part 2

    r
}

fn main() {
    let data = include_str!("../input.txt");
    let mut edges: Vec<Edge> = Vec::new();
    let mut nodes: HashMap<&str, usize> = HashMap::new();

    nodes.insert("Dummy", 0);  // node with a 0 weight edge to all other nodes. TSP algorithms expect salesman to return to original node but this challenge does not.
    for l in data.lines() {
        let components: Vec<&str> = l.split(" ").collect();
        let edge = Edge {
            node_a: String::from(components[0]),
            node_b: String::from(components[2]),
            distance: components[4].parse().unwrap()
        };
        edges.push(edge);
        let n = nodes.len();
        nodes.entry(components[0]).or_insert(n);
        let n = nodes.len();
        nodes.entry(components[2]).or_insert(n);
    }

    let number_of_nodes = nodes.len();
    let mut dist_matrix: Vec<Vec<u32>> = vec![vec![0u32; number_of_nodes]; number_of_nodes];
    
    for edge in edges {
        let x = *nodes.get(edge.node_a.as_str()).unwrap();
        let y = *nodes.get(edge.node_b.as_str()).unwrap();
        dist_matrix[x][y] = edge.distance;
        dist_matrix[y][x] = edge.distance;
    }


    let path_length = held_karp(dist_matrix, number_of_nodes);

    println!("Part 1: {}", path_length);

    println!("Part 2: {}", "");
}