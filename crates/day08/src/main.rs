use utils::{InputType, from_digits};
use std::convert::TryInto;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

static SEVEN_ARRAY: &[(&str, u16)] = &[
    ("abcefg", 0),
    ("cf", 1),
    ("acdeg", 2),
    ("acdfg", 3),
    ("bcdf", 4),
    ("abdfg", 5),
    ("abdefg", 6),
    ("acf", 7),
    ("abcdefg", 8),
    ("abcdfg", 9),
];

fn main() {
    let xs = read_input(InputType::Input);
    println!("Day06 part a = {}", part_a(&xs)); // 514
    println!("Day06 part b = {}", part_b(&xs)); // 1012272
}

fn part_a(xs: &Vec<Segment>) -> usize {
    xs.iter().fold(0, |acc, x| acc + x.count_1478())
}

fn part_b(xs: &Vec<Segment>) -> usize {
    let m: HashMap<&str, u16> = SEVEN_ARRAY.iter().map(|v| *v).collect();
    xs.iter()
        .fold(0, |acc, seg| acc + from_digits(&get_digits(&seg.outputs, &m)))
}

#[derive(Debug, Clone)]
struct Segment {
    digits: [HashSet<char>; 10],
    outputs: [Vec<char>; 4],
    decoder: HashMap<char, char>,
}

impl Segment {
    fn new(digits: [HashSet<char>; 10], outputs: [Vec<char>; 4]) -> Segment {
        let mut segment = Segment { digits, outputs, decoder: HashMap::new() };
        segment.load_decoder();
        segment.decode();
        segment
    }

    fn count_1478(&self) -> usize {
        self.outputs
            .iter()
            .map(|cs| String::from_iter(cs))
            .fold(0, |acc, s| match s.len() {
                2 | 3 | 4 | 7 => acc + 1,
                _ => acc
            })
    }

    fn find_by_len(&self, len: usize) -> Vec<HashSet<char>> {
        let xs: Vec<HashSet<char>> = self.digits.iter()
            .filter(|cs| cs.len() == len)
            .map(|cs| cs.clone())
            .collect();
        xs
    }

    fn load_decoder(&mut self) {
        self.decoder.clear();

        // known numbers
        let one: HashSet<char> = self.find_by_len(2).first().unwrap().clone();
        let seven: HashSet<char> = self.find_by_len(3).first().unwrap().clone();
        let four: HashSet<char> = self.find_by_len(4).first().unwrap().clone();
        let zero_six_nine: Vec<HashSet<char>> = self.find_by_len(6);
        let eight: HashSet<char> = self.find_by_len(7).first().unwrap().clone();

        // find six
        let six: Vec<HashSet<char>> = zero_six_nine.iter()
            .filter(|cs| !one.is_subset(cs))
            .map(|v| v.clone())
            .collect();
        let six: HashSet<char> = six.first().unwrap().clone();

        // find zero and nine
        let zero_and_nine: Vec<HashSet<char>> = zero_six_nine.iter()
            .filter(|cs| six.symmetric_difference(cs).collect::<HashSet<&char>>().len() > 0)
            .map(|cs| cs.clone())
            .collect();

        let zero: Vec<HashSet<char>> = zero_and_nine.iter()
            .filter(|cs| cs.symmetric_difference(&four).collect::<HashSet<&char>>().len() == 4)
            .map(|cs| cs.clone())
            .collect();
        let zero: HashSet<char> = zero.first().unwrap().clone();

        let nine: Vec<HashSet<char>> = zero_and_nine.iter()
            .filter(|cs| **cs != zero)
            .map(|cs| cs.clone())
            .collect();
        let nine: HashSet<char> = nine.first().unwrap().clone();

        // find a
        let a_encoded: Vec<&char> = seven.symmetric_difference(&one).collect();
        let a_encoded: char = **a_encoded.first().unwrap();
        self.decoder.insert(a_encoded, 'a');

        // find f
        let f_encoded: Vec<&char> = six.intersection(&one).collect();
        let f_encoded = **f_encoded.first().unwrap();
        self.decoder.insert(f_encoded, 'f');

        // find c
        let c_encoded: Vec<&char> = one.iter().filter(|c| **c != f_encoded).collect();
        let c_encoded = **c_encoded.first().unwrap();
        self.decoder.insert(c_encoded, 'c');

        // find d
        let d_encoded: Vec<&char> = eight.symmetric_difference(&zero).collect();
        let d_encoded = **d_encoded.first().unwrap();
        self.decoder.insert(d_encoded, 'd');

        // find e
        let e_encoded: Vec<&char> = eight.symmetric_difference(&nine).collect();
        let e_encoded = **e_encoded.first().unwrap();
        self.decoder.insert(e_encoded, 'e');

        // find g
        let mut temp = four.clone();
        temp.insert(a_encoded);
        temp.insert(e_encoded);
        let g_encoded: Vec<&char> = eight.symmetric_difference(&temp).collect();
        let g_encoded = **g_encoded.first().unwrap();
        self.decoder.insert(g_encoded, 'g');

        // find b
        let mut temp = seven.clone();
        temp.insert(d_encoded);
        temp.insert(e_encoded);
        temp.insert(g_encoded);
        let b_encoded: Vec<&char> = eight.symmetric_difference(&temp).collect();
        let b_encoded = **b_encoded.first().unwrap();
        self.decoder.insert(b_encoded, 'b');
    }

    fn decode(&mut self) {
        for i in 0..=3 {
            let mut cs: Vec<char> = self.outputs[i]
                .iter()
                .map(|c| *self.decoder.get(c).unwrap())
                .collect();
            cs.sort();
            self.outputs[i] = cs;
        }
    }
}

fn get_digits(css: &[Vec<char>; 4], m: &HashMap<&str, u16>) -> Vec<u16> {
    css.iter()
        .map(|cs| {
            let s: String = cs.iter().collect();
            *m.get(&*s).unwrap()
        })
        .collect()
}


fn read_input(input_type: InputType) -> Vec<Segment> {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };

    load_input(data)
}

fn load_input(data: &str) -> Vec<Segment> {
    let ys: Vec<&str> = data
        .lines()
        .flat_map(|s| s.split(" | "))
        .collect();

    let zs: Vec<(&str, &str)> = ys
        .chunks(2)
        .map(|ls| (ls[0], ls[1]))
        .collect();

    let zs: Vec<Segment> = zs.iter()
        .map(|(s1, s2)| {
            let ds: Vec<&str> = s1.split(" ").collect();
            let ds: Vec<HashSet<char>> = ds.iter().map(|s| s.chars().collect()).collect();
            let digits: [HashSet<char>; 10] = ds.try_into().expect("slice with incorrect length");

            let os: Vec<&str> = s2.split(" ").collect();
            let os: Vec<Vec<char>> = os.iter().map(|s| s.chars().collect()).collect();
            let outputs: [Vec<char>; 4] = os.try_into().expect("slice with incorrect length");

            Segment::new(digits, outputs)
        })
        .collect();

    zs
}

#[cfg(test)]
mod tests {
    use utils::{InputType, from_digits};
    use crate::{load_input, read_input, part_a, SEVEN_ARRAY, get_digits, part_b};
    use std::collections::HashMap;

    #[test]
    fn test_part_a() {
        let xs = read_input(InputType::Sample);
        assert_eq!(26, part_a(&xs));
    }

    #[test]
    fn test_part_b() {
        let xs = read_input(InputType::Sample);
        assert_eq!(61229, part_b(&xs));
    }

    #[test]
    fn test_decoder() {
        let data = include_str!("single-sample.txt");
        let segs = load_input(data);
        for seg in segs {
            println!("decoded outputs: {:?}", seg.outputs);

            let m: HashMap<&str, u16> = SEVEN_ARRAY.iter().map(|v| *v).collect();
            let digits = get_digits(&seg.outputs, &m);
            println!("digits: {:?}", digits);
            let result = from_digits(&digits);
            println!("result: {}", result);
            assert_eq!(5353, result)
        }
    }
}