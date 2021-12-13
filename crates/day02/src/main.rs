fn main() {
    let xs: Vec<(&str, i64)> = include_str!("input.txt")
        .lines()
        .map(|s| {
            let vs: Vec<&str> = s.split_whitespace().collect();
            let direction = *vs.get(0).unwrap();
            let v = vs.get(1).unwrap().parse::<i64>().unwrap();
            (direction, v)
        })
        .collect();

    println!("Day02 part a = {}", part_a(&xs)); // 1990000
    println!("Day02 part b = {}", part_b(&xs)); // 1975421260
}

fn process_input(xs: &[(&str, i64)]) -> Vec<(i64, i64)> {
    // Vec<(horizontal, vertical)>
    xs.iter()
        .map(|(s, v)| {
            match *s {
                "forward" => (*v, 0),
                "down" => (0, *v),
                "up" => (0, *v * -1),
                _ => (0, 0),
            }
        })
        .collect()
}

fn part_a(xs: &[(&str, i64)]) -> i64 {
    let (horizontal, vertical) = process_input(xs)
        .iter()
        .fold((0, 0), |(acc_x, acc_y), (x, y)| (acc_x + x, acc_y + y));

    horizontal * vertical
}

fn part_b(xs: &[(&str, i64)]) -> i64 {
    // Vec<(horizontal, aim)>, aim = cumulative vertical
    let vs: Vec<(i64, i64)> = process_input(xs)
        .iter()
        .scan((0i64, 0i64), |acc, (hor, aim)| {
            acc.0 = *hor;
            acc.1 += *aim;
            Some(*acc)
        })
        .collect();

    let (horizontal, depth) = vs
        .iter()
        .fold((0, 0), |(acc_x, acc_y), (x, y)| (acc_x + x, acc_y + (y * x)));

    horizontal * depth
}


#[cfg(test)]
mod tests {
    use crate::{part_a, part_b};

    static TS: [(&str, i64); 6] = [
        ("forward", 5),
        ("down", 5),
        ("forward", 8),
        ("up", 3),
        ("down", 8),
        ("forward", 2),
    ];


    #[test]
    fn test_part_a() {
        assert_eq!(150, part_a(&TS));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(900, part_b(&TS));
    }
}
