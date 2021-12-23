use std::cmp::{max, min};
use std::collections::HashMap;

use utils::InputType;

fn main() {
    let xs = read_input(InputType::Input);
    println!("Day05 part a = {}", part_a(&xs, false)); // 8622
    println!("Day05 part b = {}", part_b(&xs, false)); // 22037
}

fn part_a(xs: &Vec<Vent>, show_dump: bool) -> usize {
    let vent_m: HashMap<(u32, u32), u32> = process_horizontal_and_vertical(xs);
    if show_dump {
        dump("dump part_a", &vent_m);
    }
    vent_m.values().fold(0, |acc, v| if *v > 1 { acc + 1 } else { acc })
}

fn part_b(xs: &Vec<Vent>, show_dump: bool) -> usize {
    let mut vent_m: HashMap<(u32, u32), u32> = process_horizontal_and_vertical(xs);
    process_diagonal(xs, &mut vent_m);
    if show_dump {
        dump("dump part_b", &vent_m);
    }
    vent_m.values().fold(0, |acc, v| if *v > 1 { acc + 1 } else { acc })
}

#[derive(Debug, PartialEq)]
pub enum Line { Horizontal, Vertical, Diagonal }


#[derive(Debug)]
struct Vent {
    from: (u32, u32),
    to: (u32, u32),
    line: Line,
    start: usize,
    end: usize,
}

impl Vent {
    fn new(xs: &[u32]) -> Vent {
        let mut line = Line::Diagonal;
        let mut start = 0;
        let mut end = 0;

        if xs.len() != 4 {
            return Vent { from: (0, 0), to: (0, 0), line, start, end };
        }

        if xs[0] == xs[2] {
            line = Line::Vertical;
            start = min(xs[1], xs[3]) as usize;
            end = max(xs[1], xs[3]) as usize
        } else if xs[1] == xs[3] {
            line = Line::Horizontal;
            start = min(xs[0], xs[2]) as usize;
            end = max(xs[0], xs[2]) as usize
        }
        Vent { from: (xs[0], xs[1]), to: (xs[2], xs[3]), line, start, end }
    }
}

fn process_horizontal_and_vertical(vents: &Vec<Vent>) -> HashMap<(u32, u32), u32> {
    // let mut vent_m: HashMap<String, u32> = HashMap::new();
    let mut vent_m: HashMap<(u32, u32), u32> = HashMap::new();

    vents.iter()
        .filter(|vent| vent.line != Line::Diagonal)
        .for_each(|vent| {
            for i in vent.start..vent.end + 1 {
                // let key = if vent.line == Line::Horizontal { format!("{}:{}", i, vent.from.1) } else { format!("{}:{}", vent.from.0, i) };
                let key = if vent.line == Line::Horizontal { (i as u32, vent.from.1) } else { (vent.from.0, i as u32) };
                update_map(key, &mut vent_m);
            }
        });
    vent_m
}

fn process_diagonal(vents: &Vec<Vent>, vent_m: &mut HashMap<(u32, u32), u32>) {
    vents.iter()
        .filter(|vent| vent.line == Line::Diagonal)
        .for_each(|vent| {
            let mut x = vent.from.0;
            let mut y = vent.from.1;
            loop {
                let key = (x, y);
                update_map(key, vent_m);

                if x == vent.to.0 || y == vent.to.1 {
                    break;
                }

                if vent.from.0 >= vent.to.0 { x -= 1 } else { x += 1 }
                if vent.from.1 >= vent.to.1 { y -= 1 } else { y += 1 }
            }
        })
}

fn update_map(key: (u32, u32), vent_m: &mut HashMap<(u32, u32), u32>) {
    if let Some(v) = vent_m.get(&key) {
        let v = *v;
        vent_m.insert(key, v + 1);
    } else {
        vent_m.insert(key, 1);
    }
}

fn read_input(input_type: InputType) -> Vec<Vent> {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };

    let vs: Vec<Vent> = data
        .lines()
        .map(|s| {
            let xs: Vec<u32> = s.replace(" -> ", ",")
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect();
            Vent::new(&xs[0..4])
        })
        .collect();

    vs
}

fn dump(label: &str, vent_m: &HashMap<(u32, u32), u32>) {
    println!("{}", label);
    for y in 0..10 {
        for x in 0..10 {
            let key = (x, y);
            if let Some(v) = vent_m.get(&key) {
                print!("{}", v)
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!()
}

#[cfg(test)]
mod tests {
    use utils::InputType;

    use crate::{part_a, part_b, read_input};

    #[test]
    fn test_part_a() {
        let xs = read_input(InputType::Sample);
        assert_eq!(5, part_a(&xs, true));
    }

    #[test]
    fn test_part_b() {
        let xs = read_input(InputType::Sample);
        assert_eq!(12, part_b(&xs, true));
    }
}