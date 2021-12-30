use utils::InputType;
use std::collections::HashSet;

fn main() {
    let xss = read_input(InputType::Input);
    println!("Day09 part a = {}", part_a(&xss)); // 591
    println!("Day09 part b = {}", part_b(&xss)); // 1113424
}

fn part_a(xss: &Vec<Vec<u32>>) -> usize {
    let res = get_low_spots(&xss).iter().fold(0, |acc, (r, c)| acc + 1 + xss[*r][*c] as usize);
    res
}

fn part_b(xss: &Vec<Vec<u32>>) -> usize {
    let low_spots = get_low_spots(&xss);

    let mut in_basin = HashSet::new();
    let mut basins: Vec<usize> = low_spots.iter()
        .map(|(r, c)| count_basin(&mut in_basin, xss, *r, *c))
        .collect();

    basins.sort_by(|a, b| b.cmp(a));
    basins.iter().take(3).fold(1, |acc, v| acc * v)
}

fn get_low_spots(m: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut ys = Vec::new();
    let rows = m.len() - 2;
    let cols = m[0].len() - 2;
    for r in 1..=rows {
        for c in 1..=cols {
            if is_low_spot(m, r, c) {
                ys.push((r, c));
            }
        }
    }
    ys
}

fn is_low_spot(m: &Vec<Vec<u32>>, r: usize, c: usize) -> bool {
    if r < 1 || r > m.len() - 2 || c < 1 || c > m[0].len() - 2 {
        return false;
    }

    let v = m[r][c];

    v < m[r - 1][c]  // up
        && v < m[r + 1][c]  // down
        && v < m[r][c - 1]  // left
        && v < m[r][c + 1]  // right
}

fn count_basin(in_basin: &mut HashSet<(usize, usize)>, m: &Vec<Vec<u32>>, r: usize, c: usize) -> usize {
    let mut count = 1;
    in_basin.insert((r, c));

    if m[r - 1][c] != 9 && !in_basin.contains(&(r - 1, c)) {
        in_basin.insert((r - 1, c));
        count += count_basin(in_basin, m, r - 1, c);
    }

    if m[r + 1][c] != 9 && !in_basin.contains(&(r + 1, c)) {
        in_basin.insert((r + 1, c));
        count += count_basin(in_basin, m, r + 1, c);
    }

    if m[r][c - 1] != 9 && !in_basin.contains(&(r, c - 1)) {
        in_basin.insert((r, c - 1));
        count += count_basin(in_basin, m, r, c - 1);
    }

    if m[r][c + 1] != 9 && !in_basin.contains(&(r, c + 1)) {
        in_basin.insert((r, c + 1));
        count += count_basin(in_basin, m, r, c + 1);
    }

    count
}

fn read_input(input_type: InputType) -> Vec<Vec<u32>> {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };

    let mut xss: Vec<Vec<u32>> = data
        .lines()
        .map(|s| s.chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .collect())
        .collect();

    // pad matrix with zeros
    for xs in &mut xss {
        xs.insert(0, 9);
        xs.push(9)
    }

    let cols = xss.first().unwrap().len();
    let zeros = vec![9; cols];
    xss.insert(0, zeros.clone());
    xss.push(zeros.clone());

    xss
}

#[cfg(test)]
mod tests {
    use crate::{read_input, part_a, part_b};
    use utils::InputType;

    #[test]
    fn test_part_a() {
        let xss = read_input(InputType::Sample);
        assert_eq!(15, part_a(&xss));
    }

    #[test]
    fn test_part_b() {
        let xss = read_input(InputType::Sample);
        for xs in &xss {
            println!("{:?}", xs);
        }

        assert_eq!(1134, part_b(&xss));
    }
}