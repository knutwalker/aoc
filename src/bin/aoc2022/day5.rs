use std::{collections::VecDeque, rc::Rc};

use aoc::{Parse, PuzzleInput};

type Output = String;

register!(
    "input/day5.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(input.clone());
        part2(input);
    }
);

fn part1(mut items: Input) -> Output {
    for &instruction in &*items.instructions {
        items.stacks.apply_one_by_one(instruction);
    }
    items.stacks.result()
}

fn part2(mut items: Input) -> Output {
    for &instruction in &*items.instructions {
        items.stacks.apply_all_at_once(instruction);
    }
    items.stacks.result()
}

impl Stacks {
    fn apply_one_by_one(&mut self, Instruction { amount, from, to }: Instruction) {
        let from = usize::from(from);
        let to = usize::from(to);

        for _ in 0..amount {
            let item = self.stacks[from].pop().unwrap();
            self.stacks[to].push(item);
        }
    }

    fn apply_all_at_once(&mut self, Instruction { amount, from, to }: Instruction) {
        let from = usize::from(from);
        let to = usize::from(to);

        let [from, to] = self.stacks.get_many_mut([from, to]).unwrap();

        let cutoff_point = from.len().wrapping_sub(usize::from(amount));

        to.extend(from.drain(cutoff_point..));
    }

    fn result(&self) -> String {
        self.stacks
            .iter()
            .map(|s| char::from(s.last().copied().unwrap()))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    stacks: Stacks,
    instructions: Rc<[Instruction]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Stacks {
    stacks: Vec<Vec<u8>>,
}

impl Parse for Stacks {
    type Out<'a> = Self;
    fn parse_from(value: &str) -> Self {
        let (input, ids) = value.rsplit_once('\n').unwrap();

        let ids = ids
            .char_indices()
            .filter_map(|(i, c)| c.to_digit(10).map(|d| (d as u8 - 1, i)))
            .enumerate()
            .map(|(idx, (col, pos))| {
                assert!(idx == usize::from(col));
                pos
            })
            .collect::<Vec<_>>();

        let mut stacks = vec![Vec::with_capacity(64); ids.len()];

        let input = input.trim_start_matches('\n');
        input.lines().rev().for_each(|stack| {
            let stack = stack.as_bytes();
            for (col, idx) in ids.iter().enumerate() {
                if let Some(item) = stack.get(*idx) {
                    if item.is_ascii_alphabetic() {
                        stacks[col].push(*item);
                    }
                }
            }
        });

        Self { stacks }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Instruction {
    amount: u8,
    from: u8,
    to: u8,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let mut parts = input.split_ascii_whitespace();

        let (Some("move"), Some(amount), Some("from"), Some(from), Some("to"), Some(to), None) =
        (parts.next(), parts.next(), parts.next(), parts.next(), parts.next(), parts.next(), parts.next()) else {
            unreachable!("Invalid input: {input}");
        };

        let amount = amount.parse().unwrap();
        let from = from.parse::<u8>().unwrap() - 1;
        let to = to.parse::<u8>().unwrap() - 1;

        Self { amount, from, to }
    }
}

impl PuzzleInput for Input {
    type Out<'a> = Self;

    fn from_input(input: &str) -> Self::Out<'_> {
        let (stacks, procedure) = input.split_once("\n\n").unwrap();

        let stacks = Stacks::parse_from(stacks);
        let instructions = aoc::lines(procedure).map(Instruction::from).collect();

        Self {
            stacks,
            instructions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, "CMZ");
        assert_eq!(res2, "MCD");
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, "QNHWJVJZW");
        assert_eq!(res2, "BPCZJLFJW");
    }

    #[bench]
    fn bench_parsing(b: &mut Bencher) {
        let input = Solver::puzzle_input();
        b.bytes = input.len() as u64;
        b.iter(|| Solver::parse_input(input));
    }

    #[bench]
    fn bench_pt1(b: &mut Bencher) {
        let mut input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part1(input.clone()));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let mut input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(input.clone()));
    }
}
