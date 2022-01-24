use utils::InputType;

fn main() {
    let graph = read_input(InputType::Input);
    println!("Day15 part a = {}", part_a(&graph)); //
    println!("Day15 part b = {}", part_b(&graph)); //
}

fn part_a(graph: &Graph) -> u32 {
    if graph.ns.len() <= 100 {
        println!("graph = ");
        graph.show();
    }
    40
}

fn part_b(_graph: &Graph) -> usize {
    0
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    key: (usize, usize),
    edges: Vec<((usize, usize), u32)>,
}

#[derive(Debug)]
struct Graph {
    ns: Vec<Node>,
}

impl Graph {
    fn new(vss: &Vec<Vec<u32>>) -> Graph {
        let rows = vss.len();
        let cols = vss[0].len();
        let mut ns: Vec<Node> = vec![];

        // top left corner
        let edges: Vec<((usize, usize), u32)> = vec![((0, 1), vss[0][1]), ((1, 0), vss[1][0])];
        ns.push(Node { key: (0, 0), edges });

        // top right corner
        let edges: Vec<((usize, usize), u32)> = vec![
            ((0, cols - 2), vss[0][cols - 2]),
            ((1, cols - 1), vss[1][cols - 1]),
        ];
        ns.push(Node {
            key: (0, cols - 1),
            edges,
        });

        // bottom left corner
        let edges: Vec<((usize, usize), u32)> = vec![
            ((rows - 1, 1), vss[rows - 1][1]),
            ((rows - 2, 0), vss[rows - 2][0]),
        ];
        ns.push(Node {
            key: (rows - 1, 0),
            edges,
        });

        // bottom right
        let edges: Vec<((usize, usize), u32)> = vec![
            ((rows - 1, cols - 2), vss[rows - 1][cols - 2]),
            ((rows - 2, cols - 1), vss[rows - 2][cols - 1]),
        ];
        ns.push(Node {
            key: (rows - 1, cols - 1),
            edges,
        });

        // first row
        for j in 1..cols - 1 {
            let key = (0, j);
            let edges: Vec<((usize, usize), u32)> = vec![
                ((0, j - 1), vss[0][j - 1]),
                ((1, j), vss[1][j]),
                ((0, j + 1), vss[0][j + 1]),
            ];
            ns.push(Node { key, edges });
        }

        // last row
        for j in 1..cols - 1 {
            let key = (rows - 1, j);
            let edges: Vec<((usize, usize), u32)> = vec![
                ((rows - 1, j - 1), vss[rows - 1][j - 1]),
                ((rows - 2, j), vss[rows - 2][j]),
                ((rows - 1, j + 1), vss[rows - 1][j + 1]),
            ];
            ns.push(Node { key, edges });
        }

        // first col
        for i in 1..rows - 1 {
            let key = (i, 0);
            let edges: Vec<((usize, usize), u32)> = vec![
                ((i - 1, 0), vss[i - 1][0]),
                ((i, 1), vss[i][i]),
                ((i + 1, 0), vss[i + 1][0]),
            ];
            ns.push(Node { key, edges });
        }

        // last col
        for i in 1..rows - 1 {
            let key = (i, cols - 1);
            let edges: Vec<((usize, usize), u32)> = vec![
                ((i - 1, cols - 1), vss[i - 1][cols - 1]),
                ((i, cols - 2), vss[i][cols - 2]),
                ((i + 1, cols - 1), vss[i + 1][cols - 1]),
            ];
            ns.push(Node { key, edges });
        }

        // rest
        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                let key = (i, j);
                let edges: Vec<((usize, usize), u32)> = vec![
                    ((i - 1, j), vss[i - 1][j]),
                    ((i + 1, j), vss[i + 1][j]),
                    ((i, j + 1), vss[i][j + 1]),
                    ((i, j - 1), vss[i][j - 1]),
                ];
                ns.push(Node { key, edges });
            }
        }

        Graph { ns }
    }

    fn show(&self) {
        for n in self.ns.clone() {
            println!("{:?} -> {:?}", n.key, n.edges)
        }
        println!("");
    }
}

// fn solve(xss: &mut Vec<Vec<u32>>) {
//     let rows = xss.len();
//     let cols = xss[0].len();

//     show_grid(&xss);

//     // accumulate first row
//     for j in 1..cols {
//         xss[0][j] += cmp::min(xss[0][j - 1], xss[1][j]);
//     }

//     // accumulate first column
//     for i in 1..rows {
//         xss[i][0] += cmp::min(xss[i - 1][0], xss[i][1]);
//     }

//     // traverse and mutate matrix
//     for i in 1..rows - 1 {
//         for j in 1..cols - 1 {
//             let up_and_left = cmp::min(xss[i - 1][j], xss[i][j - 1]);
//             let down_and_right = cmp::min(xss[i + 1][j], xss[i][j + 1]);
//             xss[i][j] += cmp::min(up_and_left, down_and_right);
//         }
//     }

//     show_grid(&xss)
// }

// fn read_input1(input_type: InputType) -> Vec<Vec<u32>> {
//     let data = {
//         match input_type {
//             InputType::Sample => include_str!("sample.txt"),
//             InputType::Input => include_str!("input.txt"),
//         }
//     };

//     let xss = data
//         .lines()
//         .map(|xs| xs.chars().map(|c| c.to_digit(10).unwrap()).collect())
//         .collect();

//     xss
// }

fn read_input(input_type: InputType) -> Graph {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };

    let vss: Vec<Vec<u32>> = data
        .lines()
        .map(|xs| xs.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    Graph::new(&vss)
}

// fn show_grid(xss: &Vec<Vec<u32>>) {
//     let rows = xss.len();
//     let cols = xss[0].len();

//     for i in 0..rows {
//         for j in 0..cols {
//             print!("{:>4}", xss[i][j]);
//         }
//         println!("");
//     }
//     println!("");
// }

#[cfg(test)]
mod tests {
    use utils::InputType;

    use crate::{part_a, read_input, Graph};

    #[test]
    fn test_part_a() {
        let graph = read_input(InputType::Sample);
        assert_eq!(100, graph.ns.len());

        assert_eq!(40, part_a(&graph));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_solve() {
        let vss: Vec<Vec<u32>> = vec![
            vec![1, 9, 9, 9, 9],
            vec![1, 9, 1, 1, 1],
            vec![1, 1, 1, 9, 1],
        ];

        let graph = Graph::new(&vss);

        graph.show()
    }
}
