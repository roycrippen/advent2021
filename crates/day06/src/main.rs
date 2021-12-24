use utils::InputType;

fn main() {
    let xs = read_input(InputType::Input);
    println!("Day06 part a = {}", part_a(&xs, 80)); // 352872
    println!("Day06 part b = {}", part_b(&xs, 256)); // 1604361182149
}

#[derive(Debug, Clone, Copy)]
pub struct Accumulator {
    bin: [usize; 9],
}

impl Accumulator {
    fn new(xs: &Vec<u32>) -> Accumulator {
        let mut bin = [0; 9];
        for x in xs {
            let i = *x as usize;
            match i {
                0 => bin[i] += 1,
                1 => bin[i] += 1,
                2 => bin[i] += 1,
                3 => bin[i] += 1,
                4 => bin[i] += 1,
                5 => bin[i] += 1,
                6 => bin[i] += 1,
                7 => bin[i] += 1,
                8 => bin[i] += 1,
                _ => panic!("unreachable, x = {}", x)
            }
        }
        Accumulator { bin }
    }

    fn total(&self) -> usize {
        self.bin.iter().sum()
    }
}

impl Iterator for Accumulator {
    type Item = Accumulator;

    fn next(&mut self) -> Option<Self::Item> {
        let zeros = self.bin[0];
        self.bin.rotate_left(1);
        self.bin[6] += zeros;
        Some(*self)
    }
}

fn part_a(xs: &Vec<u32>, days: u32) -> usize {
    let mut count = 1;
    let mut ys = xs.clone();
    // println!("Initial state: {:?}", ys);
    while count <= days {
        let zeros: usize = ys.iter().fold(0, |acc, y| if *y == 0 { acc + 1 } else { acc });
        ys = ys.iter().map(|y| {
            match y {
                1..=8 => *y - 1u32,
                0 => 6,
                _ => panic!("should not be here, y = {}", y)
            }
        }).collect::<Vec<u32>>();
        let mut zs: Vec<u32> = (0..zeros).map(|_i| 8u32).collect();
        ys.append(&mut zs);
        // println!("after {:>2} days: {:?}", count, ys);
        count += 1;
    }
    ys.len()
}

fn part_b(xs: &Vec<u32>, days: usize) -> usize {
    let acc = Accumulator::new(xs);
    let ys = acc.into_iter().nth(days - 1).unwrap();
    let total = ys.total();
    // println!("total: {}, {:?}", total, ys);
    total
}

fn read_input(input_type: InputType) -> Vec<u32> {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };

    data.split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use utils::InputType;
    use crate::{read_input, part_a, part_b};

    #[test]
    fn test_part_a() {
        let xs = read_input(InputType::Sample);
        assert_eq!(26, part_a(&xs, 18));
        assert_eq!(5934, part_a(&xs, 80));
    }

    #[test]
    fn test_part_b() {
        let xs = read_input(InputType::Sample);
        assert_eq!(26, part_b(&xs, 18));
        assert_eq!(5934, part_b(&xs, 80));
        assert_eq!(26984457539, part_b(&xs, 256));
    }
}