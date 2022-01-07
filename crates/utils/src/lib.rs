pub fn flatten_zip3<A, B, C>(v: ((A, B), C)) -> (A, B, C) {
    let ((a, b), c) = v;
    (a, b, c)
}

pub fn ones_bit_count(xs: &Vec<u16>, k: usize) -> usize {
    xs.iter().fold(0 as usize, |acc, x| acc + ((*x as usize & (1 << k)) >> k))
}

pub fn from_digits(xs: &Vec<u16>) -> usize {
    let mut num: usize = 0;
    let mut fac = 10_usize.pow((xs.len() - 1) as u32);
    for i in 0..xs.len() {
        num += fac * *xs.get(i).unwrap() as usize;
        fac /= 10;
    }
    num
}

pub fn get_neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Option<Vec<(usize, usize)>> {
    if rows < 3 || cols < 3 || row > rows - 1 || col > cols - 1 {
        return None;
    }

    let r_lower = if row == 0 { row } else { row - 1 };
    let r_upper = if row == rows - 1 { row } else { row + 1 };

    let c_lower = if col == 0 { col } else { col - 1 };
    let c_upper = if col == cols - 1 { col } else { col + 1 };

    let mut xs = Vec::new();
    for r in r_lower..=r_upper {
        for c in c_lower..=c_upper {
            if r == row && c == col {
                continue;
            }
            xs.push((r, c));
        }
    }

    Some(xs)
}


#[derive(Debug)]
pub enum InputType { Sample, Input }

#[cfg(test)]
mod tests {
    use crate::{flatten_zip3, ones_bit_count, from_digits, get_neighbors};

    #[test]
    fn test_flatten_zip3() {
        let v = ((1, 'a'), true);
        assert_eq!((1, 'a', true), flatten_zip3(v))
    }

    #[test]
    fn test_bit_count() {
        let xs = vec![1, 2, 3, 4];
        assert_eq!(2, ones_bit_count(&xs, 0));
        assert_eq!(2, ones_bit_count(&xs, 1));
        assert_eq!(1, ones_bit_count(&xs, 2));
    }

    #[test]
    fn test_from_digits() {
        let aaa = from_digits(&vec![5, 3, 5, 3]);
        assert_eq!(5353, aaa);
    }

    #[test]
    fn test_get_neighbors() {
        for r in 0..3 {
            for c in 0..3 {
                if let Some(xs) = get_neighbors(r, c, 3, 3) {
                    match (r, c) {
                        (0, 0) | (0, 2) | (2, 0) | (2, 2) => assert_eq!(3, xs.len()),
                        (0, _) | (2, _) | (_, 0) | (_, 2) => assert_eq!(5, xs.len()),
                        _ => assert_eq!(8, xs.len()),
                    }
                    println!("{}, {} -> {:?}", r, c, &xs);
                }
            }
        }

        assert_eq!(None, get_neighbors(0, 0, 2, 2));
        assert_eq!(None, get_neighbors(0, 5, 5, 5));
    }
}
