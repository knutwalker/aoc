use std::{cmp::Ordering, slice::from_ref};

use aoc::PuzzleInput;
use atoi::FromRadix10;
use tap::Tap;

type Output = usize;

register!(
    "input/day13.txt";
    (input: input!(blocks input!(verbatim Input))) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| (item.left < item.right).then_some(idx + 1))
        .sum()
}

fn part2(items: &[Input]) -> Output {
    let divider = Input {
        left: Packet::List(vec![Packet::Int(2)]),
        right: Packet::List(vec![Packet::Int(6)]),
    };

    let packets = items
        .iter()
        .chain(Some(&divider))
        .flat_map(|item| [&item.left, &item.right])
        .collect::<Vec<_>>()
        .tap_mut(|v| v.sort());

    packets
        .iter()
        .enumerate()
        .filter_map(|(idx, &packet)| {
            (std::ptr::eq(packet, &divider.left) || std::ptr::eq(packet, &divider.right))
                .then_some(idx + 1)
        })
        .product()
}

#[derive(Debug, Eq)]
enum Packet {
    Int(u32),
    List(Vec<Self>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs.cmp(rhs),
            (Self::List(lhs), Self::List(rhs)) => lhs.cmp(rhs),
            (lhs @ Self::Int(_), Self::List(rhs)) => from_ref(lhs).cmp(rhs),
            (Self::List(lhs), rhs @ Self::Int(_)) => lhs.as_slice().cmp(from_ref(rhs)),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs == rhs,
            (Self::List(lhs), Self::List(rhs)) => lhs == rhs,
            (lhs @ Self::Int(_), Self::List(rhs)) => from_ref(lhs) == rhs,
            (Self::List(lhs), rhs @ Self::Int(_)) => lhs.as_slice() == from_ref(rhs),
        }
    }
}

impl Packet {
    fn parse(input: &str) -> (Self, &str) {
        input.strip_prefix('[').map_or_else(
            || match u32::from_radix_10(input.as_bytes()) {
                (_, 0) => unreachable!("invalid input: {input}"),
                (int, used) => (Self::Int(int), &input[used..]),
            },
            |mut input| {
                let mut list = Vec::new();
                loop {
                    if let Some(rest) = input.strip_prefix(']') {
                        break (Self::List(list), rest);
                    }
                    let (item, rest) = Self::parse(input);
                    list.push(item);
                    input = rest.trim_start_matches(',');
                }
            },
        )
    }
}

pub struct Input {
    left: Packet,
    right: Packet,
}

impl PuzzleInput for Input {
    type Out<'a> = Self;

    fn from_input(input: &str) -> Self::Out<'_> {
        let mut lines = aoc::lines(input);
        let (Some(left), Some(right), None) = (lines.next(), lines.next(), lines.next()) else {
            unreachable!("invalid input: {input}");
        };

        let (left, remainder) = Packet::parse(left);
        assert!(remainder.is_empty());

        let (right, remainder) = Packet::parse(right);
        assert!(remainder.is_empty());

        Self { left, right }
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
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 13);
        assert_eq!(res2, 140);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 5208);
        assert_eq!(res2, 25792);
    }

    #[bench]
    fn bench_parsing(b: &mut Bencher) {
        let input = Solver::puzzle_input();
        b.bytes = input.len() as u64;
        b.iter(|| Solver::parse_input(input));
    }

    #[bench]
    fn bench_pt1(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part1(&input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&input));
    }
}
