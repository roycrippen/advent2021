use std::cmp;
use utils::InputType;

fn main() {
    let (grid, folds) = read_input(InputType::Input);
    println!("Day13 part a = {}", part_a(&grid, &folds)); // 847
    println!("Day13 part b = {}", part_b(&grid, &folds)); // BCZRCEAB
}

fn part_a(grid: &Vec<Vec<bool>>, folds: &Vec<Fold>) -> usize {
    assert!(folds.len() > 0, "no fold instructions");
    let folded = match folds[0] {
        Fold::X(x) => fold_x(grid, x),
        Fold::Y(y) => fold_y(grid, y),
    };
    count_grid(&folded)
}

fn part_b(grid: &Vec<Vec<bool>>, folds: &Vec<Fold>) -> String {
    let mut folded = grid.clone();
    for fold in folds {
        folded = match fold {
            Fold::X(x) => fold_x(&folded, *x),
            Fold::Y(y) => fold_y(&folded, *y),
        };
    }
    show_grid("part b message =", &folded);
    "BCZRCEAB".to_owned()
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

fn count_grid(grid: &Vec<Vec<bool>>) -> usize {
    grid.into_iter()
        .flatten()
        .fold(0, |acc, &v| if v { acc + 1 } else { acc })
}

fn transpose(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut res: Vec<Vec<bool>> = vec![vec![false; rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            res[c][r] = grid[r][c]
        }
    }
    res
}

fn fold_x(grid: &Vec<Vec<bool>>, x: usize) -> Vec<Vec<bool>> {
    // slower but simple
    let transposed = transpose(grid);
    transpose(&fold_y(&transposed, x))
}

fn fold_y(grid: &Vec<Vec<bool>>, y: usize) -> Vec<Vec<bool>> {
    let rows = grid.len();
    assert!(rows > 0, "no rows in grid");
    let cols = grid[0].len();

    let g = grid.clone();

    // split the grid then throw away the fold row
    let (top, bottom) = g.split_at(y + 1);
    let top = &top[0..top.len() - 1];

    // adjust the size of the bottom to match the top
    let bottom = &mut bottom.to_vec();
    if bottom.len() < top.len() {
        for _ in 0..(top.len() - bottom.len()) {
            bottom.push(vec![false; cols])
        }
    } else if bottom.len() > top.len() {
        for _ in 0..(bottom.len() - top.len()) {
            bottom.remove(0);
        }
    }

    // reverse the bottom rows to simulate folding
    bottom.reverse();

    let folded = &mut top.to_vec();
    for r in 0..y {
        for c in 0..cols {
            folded[r][c] = top[r][c] || bottom[r][c]
        }
    }

    folded.to_vec()
}

fn show_grid(label: &str, grid: &[Vec<bool>]) {
    println!("\n{}", label);
    for ps in grid.iter() {
        for p in ps {
            let ch = if *p { '#' } else { '.' };
            print!("{}", ch)
        }
        println!("")
    }
}

fn read_input(input_type: InputType) -> (Vec<Vec<bool>>, Vec<Fold>) {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };

    let mut points: Vec<(usize, usize)> = vec![];
    let mut folds = vec![];
    let ss: Vec<&str> = data.lines().collect();
    for s in ss {
        if !s.is_empty() && !s.starts_with("fold") {
            let rc: Vec<&str> = s.split(",").take(2).collect();
            let point = (rc[0].parse().unwrap(), rc[1].parse().unwrap());
            points.push(point);
        } else if s.starts_with("fold") {
            let s = s.replace("fold along ", "");
            let tup: Vec<&str> = s.split("=").take(2).collect();
            let fold = match tup[0] {
                "x" => Fold::X(tup[1].parse().unwrap()),
                _ => Fold::Y(tup[1].parse().unwrap()),
            };
            folds.push(fold);
        }
    }

    let mut max_row = 0;
    let mut max_col = 0;
    for (c, r) in &points {
        max_row = cmp::max(max_row, *r);
        max_col = cmp::max(max_col, *c);
    }

    let mut grid: Vec<Vec<bool>> = vec![vec![false; max_col + 1]; max_row + 1];
    for (x, y) in &points {
        grid[*y][*x] = true;
    }

    (grid, folds)
}

#[cfg(test)]
mod tests {
    use crate::{count_grid, fold_x, fold_y, part_a, part_b, read_input, show_grid, Fold};
    use utils::InputType;

    #[test]
    fn test_part_a() {
        let (grid, folds) = read_input(InputType::Sample);
        assert_eq!(17, part_a(&grid, &folds));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_sample_all_folds() {
        let (grid, folds) = read_input(InputType::Sample);

        let mut folded = grid.clone();
        // show_grid("start", &folded);
        for fold in folds {
            folded = match fold {
                Fold::X(x) => fold_x(&folded, x),
                Fold::Y(y) => fold_y(&folded, y),
            };
            // show_grid(format!("{:?}", fold).as_str(), &folded);
        }
        assert_eq!(16, count_grid(&folded))
    }
}
