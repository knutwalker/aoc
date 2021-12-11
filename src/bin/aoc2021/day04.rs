use aoc::{lines, PuzzleInput};
use derive_more::{Deref, DerefMut};

register!(
    "input/day4.txt";
    (input: input!(verbatim Bingo)) -> u32 {
        part1(&input.draws, input.boards.clone());
        part2(&input.draws, input.boards);
    }
);

fn part1(draws: &[u8], boards: Vec<Board>) -> u32 {
    draws
        .iter()
        .scan(boards, |b, &n| Some(draw(b, n)))
        .flatten()
        .next()
        .unwrap()
}

fn part2(draws: &[u8], boards: Vec<Board>) -> u32 {
    draws
        .iter()
        .scan(boards, |b, &n| Some(draw(b, n)))
        .flatten()
        .last()
        .unwrap()
}

fn draw(boards: &mut Vec<Board>, number: u8) -> Vec<u32> {
    boards
        .drain_filter(move |board| board.draw(number))
        .map(move |b| b.score() * u32::from(number))
        .collect()
}

const BOARD_SIZE: usize = 5;

#[derive(Clone, Copy)]
struct Board([u8; BOARD_SIZE * BOARD_SIZE], u32);

impl Board {
    fn draw(&mut self, number: u8) -> bool {
        let Some(pos) = self.0.iter().position(|&n| n == number) else { return false };
        self.0[pos] = 0;
        self.1 |= 1 << pos;
        winners().iter().any(|&w| (self.1 & w) == w)
    }

    fn score(&self) -> u32 {
        self.0.iter().copied().map(u32::from).sum()
    }
}

#[derive(Clone, Deref, DerefMut)]
struct Boards(Vec<Board>);

impl Boards {}

#[allow(clippy::unusual_byte_groupings)]
fn winners() -> &'static [u32] {
    const fn generate() -> [u32; 10] {
        let mut result = [0; 10];
        let mut pos = 0;

        let mut row = 0b0000000_00000_00000_00000_00000_11111_u32;
        while pos < 5 {
            result[pos] = row;
            row <<= 5;
            pos += 1;
        }

        let mut col = 0b0000000_00001_00001_00001_00001_00001_u32;
        while pos < 10 {
            result[pos] = col;
            col <<= 1;
            pos += 1;
        }

        result
    }

    static WINNERS: [u32; 10] = generate();
    WINNERS.as_ref()
}

pub struct Bingo {
    draws: Vec<u8>,
    boards: Vec<Board>,
}

impl PuzzleInput for Bingo {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut blocks = input.split("\n\n");

        let draws = blocks.next().unwrap();
        let draws = lines(draws)
            .flat_map(|s| s.split(','))
            .flat_map(str::parse::<u8>)
            .collect();

        let boards = blocks
            .map(|block| {
                lines(block)
                    .flat_map(str::split_ascii_whitespace)
                    .flat_map(str::parse::<u8>)
                    .collect::<Vec<_>>()
            })
            .map(|numbers| Board(numbers.try_into().unwrap(), 0))
            .collect();

        Self { draws, boards }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test_ex() {
        let input = r#"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 4512);
        assert_eq!(res2, 1924);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 32844);
        assert_eq!(res2, 4920);
    }
}
