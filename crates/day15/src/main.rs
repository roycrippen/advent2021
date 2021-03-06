use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use utils::InputType;

fn main() {
    let graph = read_input(InputType::Input, false);
    println!("Day15 part a = {}", part_a(&graph, false)); // 589

    let graph = read_input(InputType::Input, true);
    println!("Day15 part b = {}", part_b(&graph)); // 2885
}

fn part_a(graph: &Graph, debug: bool) -> u32 {
    if debug {
        println!("graph = ");
        graph.show();
    }

    let destination = *graph.nodes.keys().max().unwrap();
    if let Some(cost) = graph.dijkstra((1, 1), destination) {
        cost
    } else {
        0
    }
}

fn part_b(graph: &Graph) -> u32 {
    let destination = *graph.nodes.keys().max().unwrap();
    if let Some(cost) = graph.dijkstra((1, 1), destination) {
        cost
    } else {
        0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Edge((usize, usize), u32);

impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        other.1.cmp(&self.1).then_with(|| self.0.cmp(&other.0))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<(usize, usize), HashMap<(usize, usize), Edge>>,
}

impl Graph {
    fn new(xss: &Vec<Vec<u32>>) -> Graph {
        let mut vss = xss.clone();

        // fill perimeter with zeros
        for vs in &mut vss {
            vs.insert(0, 0);
            vs.push(0);
        }
        let zeros = vec![0u32; vss[0].len()];
        vss.insert(0, zeros.clone());
        vss.push(zeros);

        let rows = vss.len();
        let cols = vss[0].len();
        let mut nodes: HashMap<(usize, usize), HashMap<(usize, usize), Edge>> = HashMap::new();

        // build graph
        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                let key = (i, j);

                let mut neighbors: HashMap<(usize, usize), Edge> = HashMap::new();
                neighbors.insert((i - 1, j), Edge((i - 1, j), vss[i - 1][j]));
                neighbors.insert((i + 1, j), Edge((i + 1, j), vss[i + 1][j]));
                neighbors.insert((i, j + 1), Edge((i, j + 1), vss[i][j + 1]));
                neighbors.insert((i, j - 1), Edge((i, j - 1), vss[i][j - 1]));

                neighbors = neighbors
                    .iter()
                    .filter(|(_, edge)| edge.1 > 0)
                    .map(|(node, edge)| (*node, *edge))
                    .collect();

                nodes.insert(key, neighbors);
            }
        }

        Graph { nodes }
    }

    fn dijkstra(&self, source: (usize, usize), destination: (usize, usize)) -> Option<u32> {
        // https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm

        // dist from source
        let mut dist: HashMap<(usize, usize), u32> = HashMap::new();

        // priority queue of edges
        let mut heap: BinaryHeap<Edge> = BinaryHeap::new();

        // initialize
        for v_node in self.nodes.iter() {
            if *v_node.0 != source {
                dist.insert(*v_node.0, std::u32::MAX);
                heap.push(Edge(*v_node.0, std::u32::MAX));
            } else {
                dist.insert(source, 0);
                heap.push(Edge(source, 0));
            }
        }

        while !heap.is_empty() {
            if let Some(Edge(u_node, _u_cost)) = heap.pop() {
                // are we at the destination?
                if u_node == destination {
                    return Some(*dist.get(&destination).unwrap());
                }

                // look at the neighbors
                for (v_node, Edge(_, cost)) in self.nodes.get(&u_node).unwrap().iter() {
                    let alt = *dist.get(&u_node).unwrap() + cost;
                    if alt < *dist.get(v_node).unwrap() {
                        dist.insert(*v_node, alt);
                        heap.push(Edge(*v_node, alt));
                    }
                }
            }
        }
        None
    }

    fn show(&self) {
        for edge in self.nodes.clone() {
            println!("{:>3?} -> {:?}", edge.0, edge.1)
        }
        println!("");
    }
}

fn read_input(input_type: InputType, is_expanded: bool) -> Graph {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };

    let mut xss: Vec<Vec<u32>> = data
        .lines()
        .map(|xs| xs.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    if is_expanded {
        let rows = xss.len();
        let cols = xss[0].len();

        // expand cols
        for i in 0..4 {
            for row in 0..rows {
                for col in 0..cols {
                    let x = xss[row][col + cols * i] + 1;
                    if x > 9 {
                        xss[row].push(1);
                    } else {
                        xss[row].push(x);
                    }
                }
            }
        }

        // expand rows
        let mut r_idx = rows - 1;
        let cols = xss[0].len();
        for i in 0..4 {
            for row in 0..rows {
                let xs = vec![0u32; cols];
                xss.push(xs);
                r_idx += 1;
                for col in 0..cols {
                    let x = xss[row + rows * i][col] + 1;
                    if x > 9 {
                        xss[r_idx][col] = 1;
                    } else {
                        xss[r_idx][col] = x;
                    }
                }
            }
        }
    }

    Graph::new(&xss)
}

#[cfg(test)]
mod tests {
    use utils::InputType;

    use crate::{part_a, read_input, Graph};

    #[test]
    fn test_part_a() {
        let graph = read_input(InputType::Sample, false);
        assert_eq!(10 * 10, graph.nodes.len());

        assert_eq!(40, part_a(&graph, false));
    }

    #[test]
    fn test_part_b() {
        let graph = read_input(InputType::Sample, true);
        assert_eq!(50 * 50, graph.nodes.len());

        assert_eq!(315, part_a(&graph, false));
    }

    #[test]
    fn test_solve() {
        let xss: Vec<Vec<u32>> = vec![
            vec![1, 9, 9, 9, 9],
            vec![1, 9, 1, 1, 1],
            vec![1, 1, 1, 9, 1],
        ];

        let graph = Graph::new(&xss);
        assert_eq!(15, graph.nodes.len());

        graph.show();
        if let Some(cost) = graph.dijkstra((1, 1), (3, 5)) {
            println!("cost = {}", cost);
            assert_eq!(8, cost);
        } else {
            assert!(false, "no solution")
        }
    }
}
