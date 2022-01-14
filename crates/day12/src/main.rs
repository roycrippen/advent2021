use utils::InputType;
use std::collections::{HashSet, HashMap};
use std::option::Option::Some;

fn main() {
    let graph = read_input(InputType::Input);
    println!("Day12 part a = {}", part_a(&graph)); // 4691
    println!("Day12 part b = {}", part_b(&graph)); // 140718
}

fn part_a(graph: &Graph) -> usize {
    let paths = graph.get_paths(&Part::A);
    paths.len()
}

fn part_b(graph: &Graph) -> usize {
    let paths = graph.get_paths(&Part::B);
    paths.len()
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Case { Lower, Upper }

enum Part { A, B }

struct Graph {
    m: HashMap<String, HashSet<(String, Case)>>,
}

impl Graph {
    fn new(lines: &Vec<&str>) -> Graph {
        let mut m: HashMap<String, HashSet<(String, Case)>> = HashMap::new();

        let mut add_node = |s1: &str, s2: &str| {
            if s1.eq("end") || s2.eq("start") {
                return;
            }

            let lower: Case = if s2.chars().next().unwrap().is_lowercase() { Case::Lower } else { Case::Upper };

            if let Some(nodes) = m.get(s1) {
                let mut nodes = nodes.clone();
                nodes.insert((s2.to_string(), lower.clone()));
                m.insert(s1.to_string(), nodes);
            } else {
                let mut nodes: HashSet<(String, Case)> = HashSet::new();
                nodes.insert((s2.to_string(), lower));
                m.insert(s1.to_string(), nodes);
            }
        };

        for line in lines {
            let xs: Vec<&str> = line.split("-").collect();
            assert_eq!(2, xs.len());
            add_node(xs[0], xs[1]);
            add_node(xs[1], xs[0]);
        }

        Graph { m }
    }

    fn get_paths(&self, part: &Part) -> Vec<Vec<String>> {
        let mut completed_paths: Vec<Vec<String>> = Vec::new();
        let path: Vec<String> = vec!["start".to_string()];
        let mut cnt = 0;
        self.traverse(&path, &mut completed_paths, part, &mut cnt);

        // println!("recursion count: {}", cnt);
        completed_paths
    }

    fn traverse(&self, path: &Vec<String>, completed_paths: &mut Vec<Vec<String>>, part: &Part, cnt: &mut usize) {
        if *cnt > 1_000_000 {
            return;
        }
        *cnt += 1;

        // expand the path
        let expanded_paths = self.expand_paths(path, part);

        // split paths into two groups
        let (completes, incomplete_paths): (Vec<&Vec<String>>, Vec<&Vec<String>>) = expanded_paths
            .iter()
            .partition(|&p| p.last().unwrap().eq("end"));

        // save completed paths
        for p in completes {
            completed_paths.push(p.clone());
        }

        // recurse remaining paths
        for p in incomplete_paths {
            self.traverse(p, completed_paths, part, cnt);
        }
    }

    fn expand_paths(&self, path: &Vec<String>, part: &Part) -> Vec<Vec<String>> {
        let is_allowed = |n: &(String, Case)| {
            return match part {
                Part::A => {
                    // allowed if uppercase node OR node not already in path
                    n.1 == Case::Upper || !path.contains(&n.0)
                }
                Part::B => {
                    // allowed if uppercase node
                    if n.1 == Case::Upper { return true; }

                    // count the lowercase nodes
                    let mut lowers_map: HashMap<String, u16> = HashMap::new();
                    for s in path.iter().skip(1) {
                        if s.chars().next().unwrap().is_lowercase() {
                            if lowers_map.contains_key(s) {
                                let v = *lowers_map.get(s).unwrap();
                                lowers_map.insert(s.clone(), v + 1);
                            } else {
                                lowers_map.insert(s.clone(), 1);
                            }
                        }
                    }

                    // allowed if no lowercase nodes in the path
                    if lowers_map.len() == 0 { return true; }

                    if let Some(dup) = lowers_map.iter().find(|(_k, &v)| v > 1) {
                        // not allowed if node is the duplicate
                        if n.0.eq(dup.0) { return false; }

                        // not allowed if node is already in path
                        if path.contains(&n.0) { return false; }

                        true
                    } else {
                        true
                    }
                }
            };
        };

        let mut paths: Vec<Vec<String>> = Vec::new();
        if let Some(last_key) = path.last() {
            if let Some(nodes) = self.m.get(last_key) {
                for node in nodes {
                    if is_allowed(node) {
                        let mut expanded_path = path.clone();
                        expanded_path.push(node.clone().0);
                        paths.push(expanded_path)
                    }
                }
            }
        }
        paths
    }
}

fn read_input(input_type: InputType) -> Graph {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };
    let xs: Vec<&str> = data.lines().collect();
    Graph::new(&xs)
}


#[cfg(test)]
mod tests {
    use crate::{Graph, read_input, part_a, Part, part_b};
    use utils::InputType;

    #[test]
    fn test_part_a_example_1() {
        let lines = vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ];

        let graph = Graph::new(&lines);
        let paths = graph.get_paths(&Part::A);
        // for p in &paths {
        //     println!("{:?}", p)
        // }
        println!("part a example 1 paths length = {}", paths.len());
        assert_eq!(10, paths.len())
    }

    #[test]
    fn test_part_a_example_2() {
        let lines = vec![
            "dc-end",
            "HN-start",
            "start-kj",
            "dc-start",
            "dc-HN",
            "LN-dc",
            "HN-end",
            "kj-sa",
            "kj-HN",
            "kj-dc",
        ];

        let graph = Graph::new(&lines);
        let paths = graph.get_paths(&Part::A);
        // for p in &paths {
        //     println!("{:?}", p)
        // }
        println!("part a example 2 paths length = {}", paths.len());
        assert_eq!(19, paths.len())
    }

    #[test]
    fn test_part_a() {
        let graph = read_input(InputType::Sample);
        assert_eq!(226, part_a(&graph));
    }

    #[test]
    fn test_part_b_example_1() {
        let lines = vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ];

        let graph = Graph::new(&lines);
        let paths = graph.get_paths(&Part::B);
        // for p in &paths {
        //     println!("{:?}", p)
        // }
        println!("part b example 1 paths length = {}", paths.len());
        assert_eq!(36, paths.len())
    }

    #[test]
    fn test_part_b_example_2() {
        let lines = vec![
            "dc-end",
            "HN-start",
            "start-kj",
            "dc-start",
            "dc-HN",
            "LN-dc",
            "HN-end",
            "kj-sa",
            "kj-HN",
            "kj-dc",
        ];

        let graph = Graph::new(&lines);
        let paths = graph.get_paths(&Part::B);
        // for p in &paths {
        //     println!("{:?}", p)
        // }
        println!("part b example 2 paths length = {}", paths.len());
        assert_eq!(103, paths.len())
    }

    #[test]
    fn test_part_b() {
        let graph = read_input(InputType::Sample);
        assert_eq!(3509, part_b(&graph));
    }
}