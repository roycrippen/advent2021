use std::cmp;

use utils::InputType;

fn main() {
    let xss = read_input(InputType::Input);
    println!("Day15 part a = {}", part_a(&xss)); // 595
    println!("Day15 part b = {}", part_b(&xss));
}

fn part_a(yss: &Vec<Vec<u32>>) -> u32 {
    let mut xss = yss.clone();
    solve(&mut xss);
    let last_idx = xss.len() - 1;

    xss[last_idx][last_idx] - xss[0][0]
}

fn part_b(_yss: &Vec<Vec<u32>>) -> usize {
    0
}

fn solve(xss: &mut Vec<Vec<u32>>) {
    let len = xss.len();

    let mut zs: Vec<(usize, usize)> = vec![];

    // accumulate first row and column
    for i in 1..len {
        xss[0][i] += xss[0][i - 1];
        xss[i][0] += xss[i - 1][0];
    }

    // traverse and mutate matrix
    for i in 1..len {
        for j in 1..len {
            xss[i][j] += cmp::min(xss[i - 1][j], xss[i][j - 1]);
        }
    }

    show_grid(0, len, &xss)
}

fn read_input(input_type: InputType) -> Vec<Vec<u32>> {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };

    let xss = data
        .lines()
        .map(|xs| xs.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    xss
}

fn show_grid(start: usize, end: usize, xss: &Vec<Vec<u32>>) {
    for i in start..end {
        for j in start..end {
            print!("{:>4}", xss[i][j]);
        }
        println!("");
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use utils::InputType;

    use crate::{part_a, read_input, solve};

    #[test]
    fn test_part_a() {
        let xss = read_input(InputType::Sample);
        assert_eq!(40, part_a(&xss));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_solve() {
        let mut xss: Vec<Vec<u32>> = vec![
            vec![1, 1, 6, 3],
            vec![1, 3, 8, 1],
            vec![2, 1, 3, 6],
            vec![3, 6, 9, 4],
        ];

        let show_grid = |yss| {
            for ys in yss {
                println!("{:?}", ys)
            }
            println!("");
        };

        show_grid(xss.clone());
        solve(&mut xss);
        show_grid(xss.clone());
    }
}
