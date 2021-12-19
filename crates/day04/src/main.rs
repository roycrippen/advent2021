use utils::InputType;
use std::convert::TryInto;

fn main() {
    let (moves, mut board) = read_input(InputType::Input);
    println!("Day04 part a = {}", part_a(&moves, &mut board)); // 21607
    println!("Day04 part b = {}", part_b(&moves, &mut board)); // 19012
}

const BOARD_SIZE: usize = 5;

fn part_a(draws: &Vec<u16>, boards: &mut Vec<Board>) -> usize {
    for draw in draws {
        apply_draw(boards, *draw);
        if let Some(_winner_count) = set_winners(boards) {
            return  boards.iter().find(|b| b.winner).unwrap().score(*draw);
        }
    }
    0
}

fn part_b(draws: &Vec<u16>, boards: &mut Vec<Board>) -> usize {
    for draw in draws {
        apply_draw(boards, *draw);
        if let Some(winner_count) = set_winners(boards) {
            if boards.len() > 0 && boards.len() == winner_count {
                return boards.get(boards.len() - 1).unwrap().score(*draw);
            }
        }

        // remove winners
        boards.retain(|b| !b.winner);
    }

    // no loser board
    0
}

struct Board {
    elements: [[(u16, bool); BOARD_SIZE]; BOARD_SIZE],
    winner: bool,
}

impl Board {
    fn new(data: &[u16; BOARD_SIZE * BOARD_SIZE]) -> Board {
        let mut elements = [[(0, false); BOARD_SIZE]; BOARD_SIZE];
        let mut idx = 0;
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                elements[i][j].0 = data[idx];
                idx += 1;
            }
        }
        Board { elements, winner: false }
    }

    fn set_winner(&mut self) -> bool {
        // row winner
        for i in 0..BOARD_SIZE {
            if self.elements[i].iter().all(|(_v, b)| *b) {
                return true;
            }
        }

        // column winner
        for j in 0..BOARD_SIZE {
            let mut count = 0;
            for i in 0..BOARD_SIZE {
                if self.elements[i][j].1 {
                    count += 1;
                }
            }
            if count == BOARD_SIZE {
                self.winner = true;
                return true;
            }
        }

        // not a winner
        false
    }

    fn score(&self, draw: u16) -> usize {
        let mut sum = 0;
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if !self.elements[i][j].1 {
                    sum += self.elements[i][j].0;
                }
            }
        }
        sum as usize * draw as usize
    }
}

fn read_input(input_type: InputType) -> (Vec<u16>, Vec<Board>) {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };
    let moves = data
        .lines()
        .take(1)
        .flat_map(|xs| xs
            .split(",")
            .map(|x| x.parse().unwrap()))
        .collect();

    let bs: Vec<u16> = data
        .lines()
        .skip(2)
        .filter(|s| s.len() > 0)
        .flat_map(|xs| xs
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|x| x.parse().unwrap()))
        .collect();

    let boards: Vec<Board> = bs[..]
        .chunks(BOARD_SIZE * BOARD_SIZE)
        .map(|chunk| Board::new(chunk.try_into().expect("slice with incorrect length")))
        .collect();

    (moves, boards)
}

fn apply_draw(boards: &mut Vec<Board>, draw: u16) {
    for board in boards {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if board.elements[i][j].0 == draw {
                    board.elements[i][j].1 = true
                }
            }
        }
    }
}

fn set_winners(boards: &mut Vec<Board>) -> Option<usize> {
    for board in boards.into_iter() {
        board.winner = board.set_winner()
    }
    let winner_count = boards.iter().fold(0, |acc, b| if b.winner { acc + 1 } else { acc });
    if winner_count > 0 { Some(winner_count) } else { None }
}

#[cfg(test)]
mod tests {
    use crate::{read_input, part_a, part_b};
    use utils::InputType;

    #[test]
    fn test_part_a() {
        let (moves, mut board) = read_input(InputType::Sample);
        assert_eq!(4512, part_a(&moves, &mut board));
    }

    #[test]
    fn test_part_b() {
        let (moves, mut board) = read_input(InputType::Sample);
        assert_eq!(1924, part_b(&moves, &mut board));
    }
}