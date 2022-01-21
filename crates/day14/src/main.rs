use std::collections::HashMap;

use utils::InputType;

fn main() {
    let (template, rules) = read_input_old(InputType::Input);
    println!("Day14 part a = {}", part_a(&template, &rules, 10)); // 3213
    println!("Day14 part b = {}", part_b(&template, &rules, 40));
}

fn part_a(template: &Vec<char>, rules: &HashMap<Vec<char>, char>, n: usize) -> usize {
    let mut xs = template.clone();

    // apply the rules n times
    for _ in 1..=n {
        xs = apply_rules_old(&xs, &rules);
    }

    // count the individual chars
    let mut m: HashMap<char, usize> = HashMap::new();
    xs.iter().for_each(|x| {
        if m.contains_key(x) {
            m.insert(*x, m.get(x).unwrap() + 1);
        } else {
            m.insert(*x, 1);
        }
    });

    // find largest and smallest count
    let mut ls: Vec<usize> = m.iter().map(|(_k, v)| *v).collect();
    ls.sort();
    let largest = *ls.last().unwrap();
    let smallest = *ls.first().unwrap();

    largest - smallest
}

fn part_b(template: &Vec<char>, rules: &HashMap<Vec<char>, char>, n: usize) -> usize {
    part_a(template, rules, n)
}

fn apply_rules_old(template: &Vec<char>, rules: &HashMap<Vec<char>, char>) -> Vec<char> {
    // let mut res = vec![];
    // for i in 0..template.len() - 1 {
    //     let key = vec![template[i], template[i + 1]];
    //     res.push(template[i]);
    //     if rules.contains_key(&key) {
    //         res.push(*rules.get(&key).unwrap());
    //     }
    // }
    // res.push(*template.last().unwrap());

    // res

    vec![]
}

fn apply_rules(template: &mut HashMap<String, usize>, rules: &HashMap<String, (String, String)>) {
    // let mut res = vec![];
    // for i in 0..template.len() - 1 {
    //     let key = vec![template[i], template[i + 1]];
    //     res.push(template[i]);
    //     if rules.contains_key(&key) {
    //         res.push(*rules.get(&key).unwrap());
    //     }
    // }
    // res.push(*template.last().unwrap());

    let inc_template = |insert_key: &String, m: &mut HashMap<String, usize>| {
        if m.contains_key(insert_key) {
            m.insert(insert_key.clone(), m.get(insert_key).unwrap() + 1);
        } else {
            m.insert(insert_key.clone(), 1);
        }
    };

    let tm = template.clone();
    let mut additions: Vec<(String, usize)> = Vec::new();
    let mut subtractions: Vec<(String, usize)> = Vec::new();

    for (k, v) in tm {
        let (rule_key1, rule_key2) = rules.get(&k).unwrap();
        additions.push((rule_key1.clone(), v));
        additions.push((rule_key2.clone(), v));
        subtractions.push((k.clone(), v));

    }

    for (k, v) in additions {
        if template.contains_key(k) {
            template.insert(k, template.get(k).unwrap() + v)
        }
    }

    println!("aaa")
}

// inc_template(rule_key1, template);
// inc_template(rule_key2, template);
// if template.contains_key(rule_key1) {
//     let v = *template.get(rule_key1).unwrap();
//     let v = if v == 0 { 1 } else { 2 * v };
//     template.insert(rule_key1.clone(), v);

//     let reduction = if *template.get(&k).unwrap() > v {
//         template.get(&k).unwrap() - v
//     } else {
//         0
//     };
//     template.insert(k.clone(), reduction);
// } else {
//     template.insert(rule_key1.clone(), 1);
//     template.insert(k.clone(), 0);
// }

// if template.contains_key(rule_key2) {
//     let v = *template.get(rule_key2).unwrap();
//     let v = if v == 0 { 1 } else { 2 * v };
//     template.insert(rule_key2.clone(), v);

//     let reduction = if *template.get(&k).unwrap() > v {
//         template.get(&k).unwrap() - v
//     } else {
//         0
//     };
//     template.insert(k.clone(), reduction);
// } else {
//     template.insert(rule_key2.clone(), 1);
//     template.insert(k.clone(), 0);
// }

fn read_input_old(input_type: InputType) -> (Vec<char>, HashMap<Vec<char>, char>) {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };

    let mut template: Vec<char> = vec![];
    let mut rules: HashMap<Vec<char>, char> = HashMap::new();
    let ss: Vec<&str> = data.lines().collect();
    for s in ss {
        if s.is_empty() {
            continue;
        }

        if template.is_empty() {
            template = s.chars().collect();
        } else {
            let tup: Vec<&str> = s.split(" -> ").take(2).collect();
            let key = tup[0].chars().collect();
            let values: Vec<char> = tup[1].chars().collect();
            let value = values[0];
            rules.insert(key, value);
        }
    }

    (template, rules)
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
    use std::{collections::HashMap, hash::Hash};

    use utils::InputType;

    use crate::{apply_rules, apply_rules_old, part_a, part_b, read_input, read_input_old};

    #[test]
    fn test_apply_rules() {
        let (template, rules, last_char) = read_input(InputType::Sample);

        let m = &mut template.clone();

        for _ in 1..=1 {
            apply_rules(m, &rules);
        }

        let mut xs: Vec<(char, usize)> = vec![];
        m.iter().for_each(|(k, v)| {
            let cs: Vec<char> = k.chars().take(2).collect();
            xs.push((cs[0], *v));
            // xs.push((cs[1], *v));
        });
        xs.sort();

        // count the individual chars
        let mut m: HashMap<char, usize> = HashMap::new();
        m.insert(last_char, 1);
        xs.iter().for_each(|(c, v)| {
            if m.contains_key(c) {
                m.insert(*c, m.get(c).unwrap() + v);
            } else {
                m.insert(*c, *v);
            }
        });

        let ys: Vec<usize> = m.iter().map(|(_, v)| *v).collect();

        let cnt: usize = ys.iter().sum();

        println!("total chars = {}", cnt)
    }

    #[test]
    fn test_part_a() {
        let (template, rules) = read_input_old(InputType::Sample);
        assert_eq!(1588, part_a(&template, &rules, 10));
    }

    #[test]
    fn test_part_b() {
        let (template, rules) = read_input_old(InputType::Sample);
        assert_eq!(2188189693529, part_b(&template, &rules, 40));
    }
}
