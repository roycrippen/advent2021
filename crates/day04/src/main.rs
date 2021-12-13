fn main() {
    let xs: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Day04 part a = {}", part_a(&xs));
    println!("Day04 part b = {}", part_b(&xs));
}

fn part_a(xs: &[usize]) -> usize {
    todo!()
}

fn part_b(xs: &[usize]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(true, true);
    }
}