use utils::ones_bit_count;

fn main() {
    let xs: Vec<u16> = include_str!("input.txt")
        .lines()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    println!("Day03 part a = {}", part_a(&xs, 12)); // 1131506
    println!("Day03 part b = {}", part_b(&xs, 12)); // 7863147
}

fn part_a(xs: &Vec<u16>, str_len: usize) -> usize {
    let len = xs.len();
    let gamma_str = (0..str_len).rev()
        .fold("".to_string(), |acc, k| {
            let ones = ones_bit_count(xs, k);
            acc + dominate_bit(ones, len - ones)
        });
    let gamma = usize::from_str_radix(&*gamma_str, 2).unwrap();

    let epsilon_str: String = gamma_str.chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect();
    let epsilon = usize::from_str_radix(&*epsilon_str, 2).unwrap();

    // println!("gamma_str:   {}, gamma:   {}", gamma_str, gamma);
    // println!("epsilon_str: {}, epsilon: {}", epsilon_str, epsilon);

    gamma * epsilon
}

fn part_b(xs: &Vec<u16>, str_len: usize) -> usize {
    let oxygen = filter(xs, str_len, true).unwrap() as usize;
    let co2 = filter(xs, str_len, false).unwrap() as usize;

    // println!("oxygen generator rating = {:>6}", oxygen);
    // println!("CO2 scrubber rating     = {:>6}", co2);

    oxygen * co2
}

fn dominate_bit(ones: usize, zeros: usize) -> &'static str {
    if ones >= zeros { "1" } else { "0" }
}

fn filter(xs: &Vec<u16>, k: usize, use_dominate: bool) -> Option<u16> {
    let len = xs.len();
    if len == 0 {
        None
    } else if len == 1 {
        Some(*xs.get(0).unwrap())
    } else {
        let ys = filter_by_kth_bit(xs, k - 1, use_dominate);
        filter(&ys, k - 1, use_dominate)
    }
}

fn filter_by_kth_bit(xs: &Vec<u16>, k: usize, use_dominate: bool) -> Vec<u16> {
    let len = xs.len();
    xs.iter()
        .filter(|x| {
            let ones = ones_bit_count(xs, k);
            let dominate_bit = dominate_bit(ones, len - ones);
            let kth_bit = ((**x as usize & (1 << k)) >> k).to_string();
            if use_dominate {
                kth_bit == dominate_bit
            } else {
                kth_bit != dominate_bit
            }
        })
        .map(|x| *x)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{part_a, part_b};

    static TS: [&str; 12] = [
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];

    #[test]
    fn test_part_a() {
        let xs: Vec<u16> = TS.iter()
            .map(|s| u16::from_str_radix(s, 2).unwrap())
            .collect();
        assert_eq!(198, part_a(&xs, 5));
    }

    #[test]
    fn test_part_b() {
        let xs: Vec<u16> = TS.iter()
            .map(|s| u16::from_str_radix(s, 2).unwrap())
            .collect();
        assert_eq!(230, part_b(&xs, 5));
    }
}