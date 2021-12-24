use utils::InputType;
use std::collections::HashMap;

fn main() {
    let xs = read_input(InputType::Input);
    println!("Day06 part a = {}", part_a(&xs)); // 348664
    println!("Day06 part b = {}", part_b(&xs));
}

fn part_a(xs: &Vec<usize>) -> usize {
    let max = *xs.iter().max().unwrap() as usize;
    let min = *xs.iter().min().unwrap() as usize;
    let ys: Vec<usize> = (min..=max).map(|v| distance_a(&v, xs)).collect();
    let res = *ys.iter().min().unwrap();

    // let zipped: Vec<(&usize, usize)> = xs.iter().zip(ys).collect();
    // println!("{:?}", zipped);
    res
}

fn part_b(xs: &Vec<usize>) -> usize {
    let max = *xs.iter().max().unwrap() as usize;
    let min = *xs.iter().min().unwrap() as usize;
    let delta = max - min;
    let cache: HashMap<usize, usize> = (0..=delta).map(|v| (v, (0..=v).sum())).collect();
    let ys: Vec<usize> = (min..=max).map(|v| distance_b(&v, xs, &cache)).collect();
    let res = *ys.iter().min().unwrap();

    // let zipped: Vec<(&usize, usize)> = xs.iter().zip(ys).collect();
    // println!("{:?}", zipped);
    res
}

fn distance_a(destination: &usize, xs: &Vec<usize>) -> usize {
    xs.iter().fold(0, |acc, x| if x <= destination { acc + destination - x } else { acc + x - destination })
}

fn distance_b(destination: &usize, xs: &Vec<usize>, cache: &HashMap<usize, usize>) -> usize {
    xs.iter().fold(0, |acc, x|
        if x <= destination {
            let delta = destination - x;
            acc + cache.get(&delta).unwrap()
        } else {
            let delta = x - destination;
            acc + cache.get(&delta).unwrap()
        })
}

fn read_input(input_type: InputType) -> Vec<usize> {
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
    use crate::{read_input, part_a, part_b};
    use utils::InputType;

    #[test]
    fn test_part_a() {
        let xs = read_input(InputType::Sample);
        assert_eq!(37, part_a(&xs));
    }

    #[test]
    fn test_part_b() {
        let xs = read_input(InputType::Sample);
        assert_eq!(168, part_b(&xs));
    }
}