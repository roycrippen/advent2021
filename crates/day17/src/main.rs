use utils::InputType;

fn main() {
    let aaa = read_input(InputType::Input);
    println!("Day17 part a = {}", part_a(aaa));
    println!("Day17 part b = {}", part_b(aaa));
}

fn part_a(_xs: ()) -> usize {
    todo!()
}

fn part_b(_xs: ()) -> usize {
    todo!()
}

fn read_input(input_type: InputType) -> () {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };
    println!("Source: {:?}, length: {}", input_type, data.len());

    ()
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
