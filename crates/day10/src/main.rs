use utils::InputType;

fn main() {
    let xss = read_input(InputType::Input);
    println!("Day10 part a = {}", part_a(&xss)); // 358737
    println!("Day10 part b = {}", part_b(&xss)); // 4329504793
}

fn part_a(xss: &Vec<Vec<char>>) -> usize {
    let corrupt_chars: Vec<char> = xss.iter()
        .map(|xs| find_corrupt_char(xs).0)
        .filter(|o: &Option<char>| o.is_some())
        .map(|o: Option<char>| o.unwrap())
        .collect();

    corrupt_chars.iter().fold(0, |acc, ch| acc + score_error_char(ch))
}

fn part_b(xss: &Vec<Vec<char>>) -> usize {
    let mut scores: Vec<usize> = Vec::new();
    for xs in xss {
        if let (None, missing_chars) = find_corrupt_char(&xs) {
            let score = missing_chars.iter().fold(0, |acc, ch| 5 * acc + score_char(ch));
            scores.push(score);
        }
    }

    let len = scores.len();
    assert!(len > 0, "no scores");
    assert_ne!(len % 2, 0, "even number of scores");
    scores.sort();
    *scores.get(len / 2).unwrap()
}

fn find_corrupt_char(chs: &Vec<char>) -> (Option<char>, Vec<char>) {
    let mut stack = Vec::new();
    for ch in chs {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(flip(ch)),
            _ => {
                if let Some(closing_char) = stack.pop() {
                    if closing_char != *ch {
                        stack.reverse();
                        return (Some(*ch), stack);
                    }
                }
            }
        }
    }
    stack.reverse();
    (None, stack)
}

fn flip(ch: &char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("function flip: input data is corrupt"),
    }
}

fn score_error_char(ch: &char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("function score_error_char: input data is corrupt"),
    }
}

fn score_char(ch: &char) -> usize {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("function score_char: input data is corrupt"),
    }
}

fn read_input(input_type: InputType) -> Vec<Vec<char>> {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };

    let xss: Vec<Vec<char>> = data
        .lines()
        .map(|s| s.chars()
            .collect())
        .collect();
    xss
}

#[cfg(test)]
mod tests {
    use utils::InputType;

    use crate::{find_corrupt_char, part_a, part_b, read_input, score_error_char};

    #[test]
    fn test_part_a() {
        let xss = read_input(InputType::Sample);
        assert_eq!(26397, part_a(&xss));
    }

    #[test]
    fn test_part_b() {
        let xss = read_input(InputType::Sample);
        assert_eq!(288957, part_b(&xss));
    }

    #[test]
    fn test_find_corrupt_char() {
        let xs = "{([(<{}[<>[]}>{[]{[(<()>".chars().collect();

        if let (Some(ch), _stack) = find_corrupt_char(&xs) {
            assert_eq!('}', ch);
            println!("char = {}, score = {}", ch, score_error_char(&ch))
        } else {
            assert!(false, "no corrupt char found")
        }
    }

    #[test]
    fn test_missing_chars() {
        let xs: Vec<char> = "[({(<(())[]>[[{[]{<()<>>".chars().collect();
        let (ch, missing_chars) = find_corrupt_char(&xs);
        assert_eq!(None, ch);

        println!("missing_chars: {:?}", &missing_chars);

        let should_be: Vec<char> = "}}]])})]".chars().collect();
        assert_eq!(should_be, missing_chars)
    }
}
