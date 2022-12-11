#![allow(clippy::redundant_clone)]

use std::{cell::RefCell, cmp::Reverse};

use aoc::{Parse, PuzzleInput};

type Output = u64;

register!(
    "input/day11.txt";
    (input: input!(blocks input!(verbatim Monkey))) -> Output {
        part1(&input.clone());
        part2(&input);
    }
);

fn part1(monkeys: &[Monkey]) -> Output {
    part::<20, 3>(monkeys)
}

fn part2(monkeys: &[Monkey]) -> Output {
    part::<10_000, 1>(monkeys)
}

fn part<const ROUNDS: usize, const DROP: u64>(monkeys: &[Monkey]) -> Output {
    fn gcd(m: u64, n: u64) -> u64 {
        n.checked_rem(m).map_or(n, |n| gcd(n, m))
    }

    fn lcm(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }

    let lcm = monkeys.iter().map(|m| m.test).reduce(lcm).unwrap();
    let mut inspections = vec![0_u64; monkeys.len()];

    for _round in 1..=ROUNDS {
        for (monkey, inspections) in monkeys.iter().zip(&mut inspections) {
            monkey.play::<DROP>(monkeys, lcm, inspections);
        }
    }

    let (most_active, _, _) = inspections.select_nth_unstable_by_key(2, |m| Reverse(*m));
    most_active.iter().product()
}

#[derive(Clone)]
pub struct Monkey {
    items: RefCell<Vec<u64>>,
    op: Op,
    test: u64,
    branch: [u32; 2],
}

impl Monkey {
    fn play<const N: u64>(&self, monkeys: &[Self], lcm: u64, inspections: &mut u64) {
        let mut items = self.items.borrow_mut();
        *inspections += items.len() as u64;
        for item in items.drain(..) {
            let worry = self.op.apply(item) / N;
            let target = self.branch[usize::from(worry % self.test == 0)];
            let worry = worry % lcm;
            monkeys[target as usize].items.borrow_mut().push(worry);
        }
    }
}

#[derive(Copy, Clone)]
enum Op {
    Add(u32),
    Mul(u32),
    Square,
}

impl Op {
    fn apply(self, old: u64) -> u64 {
        match self {
            Self::Add(n) => old + u64::from(n),
            Self::Mul(n) => old * u64::from(n),
            Self::Square => old * old,
        }
    }
}

impl PuzzleInput for Monkey {
    type Out<'a> = Self;

    fn from_input(input: &str) -> Self::Out<'_> {
        let mut lines = aoc::lines(input);
        let (
            Some(_monkey),
            Some(starting),
            Some(operation),
            Some(test),
            Some(if_true),
            Some(if_false),
            None,
        ) = (
            lines.next(),
            lines.next(),
            lines.next(),
            lines.next(),
            lines.next(),
            lines.next(),
            lines.next(),
        ) else {
            unreachable!("invalid input: {input}");
        };

        let items = RefCell::new(
            starting[16..]
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect(),
        );

        let op = Op::parse_from(&operation[17..]);
        let test = test[19..].parse().unwrap();
        let if_true = if_true[25..].parse().unwrap();
        let if_false = if_false[26..].parse().unwrap();
        let branch = [if_false, if_true];

        Self {
            items,
            op,
            test,
            branch,
        }
    }
}

impl Parse for Op {
    type Out<'a> = Self;

    fn parse_from(s: &str) -> Self::Out<'_> {
        let ("old", op, new) = (&s[..3], &s[4..5], &s[6..]) else {
            unreachable!("invalid operation: {s}");
        };

        match new {
            "old" if op == "+" => Self::Mul(2),
            "old" if op == "*" => Self::Square,
            new => {
                let new = new.parse().unwrap();
                match op {
                    "+" => Self::Add(new),
                    "*" => Self::Mul(new),
                    _ => unreachable!("invalid operation: {s}"),
                }
            }
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
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
        "#;

        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 10605);
        assert_eq!(res2, 2_713_310_158);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 61503);
        assert_eq!(res2, 14_081_365_540);
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
