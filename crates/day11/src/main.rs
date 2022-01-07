use std::collections::HashSet;

use utils::{get_neighbors, InputType};

fn main() {
    let m = read_input(InputType::Input);
    println!("Day11 part a = {}", part_a(&m)); // 1601
    println!("Day11 part b = {}", part_b(&m)); // 368
}

fn part_a(xss: &Vec<Vec<u32>>) -> usize {
    let mut m = xss.clone();
    let mut cnt = 0;
    for _i in 1..=100 {
        cnt += step(&mut m);
        // let s = format!("step {}", _i).to_string();
        // show(&s, &m);
    }
    cnt
}

fn part_b(xss: &Vec<Vec<u32>>) -> usize {
    let mut m = xss.clone();
    for i in 1.. {
        let cnt = step(&mut m);
        // let s = format!("step {}", i).to_string();
        // show(&s, &m);
        if cnt == 100 {
            return i;
        }
    }
    0
}

fn step(m: &mut Vec<Vec<u32>>) -> usize {
    // increment all
    for r in 0..m.len() {
        for c in 0..m[0].len() {
            m[r][c] += 1;
        }
    }

    // flash recursively any energy 10
    let mut flash_count = 0;
    let mut flashed_map: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..m.len() {
        for c in 0..m[0].len() {
            if m[r][c] == 10 {
                if !flashed_map.contains(&(r, c)) {
                    flashed_map.insert((r, c));
                    flash(r, c, m, &mut flash_count, &mut flashed_map);
                }
            }
        }
    }

    // set any energy 10 to 0
    for r in 0..m.len() {
        for c in 0..m[0].len() {
            if m[r][c] == 10 {
                m[r][c] = 0;
            }
        }
    }

    // return the flash count for this step
    flash_count
}

fn flash(r: usize, c: usize, m: &mut Vec<Vec<u32>>, cnt: &mut usize, flashed_map: &mut HashSet<(usize, usize)>) {
    if m[r][c] != 10 {
        println!("error, cell can not be flashed");
        return;
    }

    *cnt += 1;

    if let Some(neighbors) = get_neighbors(r, c, m.len(), m[0].len()) {
        for (nr, nc) in neighbors {
            // already flashed?
            if flashed_map.contains(&(nr, nc)) {
                continue;
            }

            // increment the neighbor
            if m[nr][nc] < 10 {
                m[nr][nc] += 1;
            }

            // need to flash the neighbor?
            if m[nr][nc] == 10 {
                flashed_map.insert((nr, nc));
                flash(nr, nc, m, cnt, flashed_map);
            }
        }
    }
}

#[allow(dead_code)]
fn show(s: &str, m: &Vec<Vec<u32>>) {
    println!("{}", s);
    for row in m.iter() {
        println!("  {:?}", row)
    }
}

fn read_input(input_type: InputType) -> Vec<Vec<u32>> {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };
    let xss: Vec<Vec<u32>> = data
        .lines()
        .map(|s| s.chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .collect())
        .collect();

    xss
}

#[cfg(test)]
mod tests {
    use utils::InputType;

    use crate::{part_a, part_b, read_input, show, step};

    #[test]
    fn test_part_a() {
        let xss = read_input(InputType::Sample);
        assert_eq!(1656, part_a(&xss));
    }

    #[test]
    fn test_part_b() {
        let xss = read_input(InputType::Sample);
        assert_eq!(195, part_b(&xss));
    }

    #[test]
    fn test_5x5_example() {
        let mut m: Vec<Vec<u32>> = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1],
        ];

        let s = format!("step {}", 0).to_string();
        show(&s, &m);
        for i in 1..=2 {
            step(&mut m);
            let s = format!("step {}", i).to_string();
            show(&s, &m);
        }

        let should_be: Vec<Vec<u32>> = vec![
            vec![4, 5, 6, 5, 4],
            vec![5, 1, 1, 1, 5],
            vec![6, 1, 1, 1, 6],
            vec![5, 1, 1, 1, 5],
            vec![4, 5, 6, 5, 4],
        ];

        assert_eq!(&should_be, &m);
    }
}