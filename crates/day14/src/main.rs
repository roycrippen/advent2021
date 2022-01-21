use std::collections::HashMap;

use utils::InputType;

fn main() {
    let (template, rules, last_char) = read_input(InputType::Input);
    println!(
        "Day14 part a = {}",
        part_a(&template, &rules, last_char, 10)
    ); // 3213
    println!(
        "Day14 part b = {}",
        part_a(&template, &rules, last_char, 40)
    ); // 3711743744429
}

fn part_a(
    template: &HashMap<String, usize>,
    rules: &HashMap<String, (String, String)>,
    last_char: char,
    n: usize,
) -> usize {
    let solution_m = solve(template, rules, last_char, n);
    let mut ls: Vec<usize> = solution_m.iter().map(|(_, v)| *v).collect();
    ls.sort();
    let largest = *ls.last().unwrap();
    let smallest = *ls.first().unwrap();

    largest - smallest
}

fn solve(
    template: &HashMap<String, usize>,
    rules: &HashMap<String, (String, String)>,
    last_char: char,
    n: usize,
) -> HashMap<char, usize> {
    let m = &mut template.clone();

    // apply the riles n times mutating m
    for _ in 1..=n {
        apply_rules(m, &rules);
    }

    // transform from char pairs to char and sort
    let mut xs: Vec<(char, usize)> = vec![];
    m.iter().for_each(|(k, v)| {
        let cs: Vec<char> = k.chars().take(1).collect();
        xs.push((cs[0], *v));
    });
    xs.sort();

    // group by char with count foreach
    let mut m: HashMap<char, usize> = HashMap::new();
    m.insert(last_char, 1);
    xs.iter().for_each(|(c, v)| {
        if m.contains_key(c) {
            m.insert(*c, m.get(c).unwrap() + v);
        } else {
            m.insert(*c, *v);
        }
    });

    m
}

fn apply_rules(template: &mut HashMap<String, usize>, rules: &HashMap<String, (String, String)>) {
    let tm = template.clone();
    let mut additions: Vec<(String, usize)> = Vec::new();
    let mut subtractions: Vec<(String, usize)> = Vec::new();

    // find additions and subtractions to apply to the template
    for (k, v) in tm {
        if v != 0 {
            let (rule_key1, rule_key2) = rules.get(&k).unwrap();
            additions.push((rule_key1.clone(), v));
            additions.push((rule_key2.clone(), v));
            subtractions.push((k.clone(), v));
        }
    }

    // mutate by applying subtractions
    for (k, v) in &subtractions {
        if let Some(template_v) = template.get(k).cloned() {
            if template_v >= *v {
                template.insert(k.clone(), template_v - v);
            }
        }
    }

    // mutate by applying additions
    for (k, v) in &additions {
        if let Some(template_v) = template.get(k).cloned() {
            template.insert(k.clone(), template_v + v);
        } else {
            template.insert(k.clone(), *v);
        }
    }
}

fn read_input(
    input_type: InputType,
) -> (
    HashMap<String, usize>,
    HashMap<String, (String, String)>,
    char,
) {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };

    let mut template: HashMap<String, usize> = HashMap::new();
    let mut rules: HashMap<String, (String, String)> = HashMap::new();
    let ss: Vec<&str> = data.lines().collect();
    let mut last_char = ' ';
    for s in ss {
        if s.is_empty() {
            continue;
        }

        if template.is_empty() {
            let cs: Vec<char> = s.chars().collect();
            last_char = *cs.last().unwrap();
            for i in 0..cs.len() - 1 {
                let k = format!("{}{}", cs[i], cs[i + 1]);
                if template.contains_key(&k) {
                    template.insert(k.clone(), template.get(&k).unwrap() + 1);
                } else {
                    template.insert(k, 1);
                }
            }
        } else {
            let tup: Vec<&str> = s.split(" -> ").take(2).collect();
            let ks: Vec<char> = tup[0].chars().collect();
            let value: (String, String) = (
                format!("{}{}", ks[0], tup[1]),
                format!("{}{}", tup[1], ks[1]),
            );
            let k = ks.into_iter().collect();
            rules.insert(k, value);
        }
    }

    (template, rules, last_char)
}

#[cfg(test)]
mod tests {
    use utils::InputType;

    use crate::{part_a, read_input, solve};

    #[test]
    fn test_apply_rules() {
        let (template, rules, last_char) = read_input(InputType::Sample);
        let m = solve(&template, &rules, last_char, 40);
        assert_eq!(2192039569602, *m.get(&'B').unwrap());
        assert_eq!(3849876073, *m.get(&'H').unwrap());
    }

    #[test]
    fn test_part_a() {
        let (template, rules, last_char) = read_input(InputType::Sample);
        assert_eq!(1588, part_a(&template, &rules, last_char, 10));
    }

    #[test]
    fn test_part_b() {
        let (template, rules, last_char) = read_input(InputType::Sample);
        assert_eq!(2188189693529, part_a(&template, &rules, last_char, 40));
    }
}
