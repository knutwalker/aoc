use aoc::{Parse, ProcessInput, PuzzleInput};
use std::collections::HashMap;

register!(
    "input/day7.txt";
    (input: input!(process RuleInput)) -> usize {
        run1(&input);
        run2(&input);
    }
);

pub type Rules<'a> = HashMap<&'a str, Vec<Bag<'a>>>;

fn run1(input: &Rules) -> usize {
    let mut total = 0;
    for key in input.keys() {
        let mut keys = vec![key];
        while !keys.is_empty() {
            let key = keys.pop().unwrap();
            for Bag { color, .. } in &input[key] {
                if *color == "shiny gold" {
                    total += 1;
                    keys.clear();
                    break;
                }
                keys.push(color);
            }
        }
    }
    total
}

fn run2(input: &Rules) -> usize {
    let mut total = 0;
    let mut q = vec![(1, "shiny gold")];
    while !q.is_empty() {
        let (mult, next) = q.pop().unwrap();
        for Bag { amount, color } in &input[next] {
            let amt = *amount * mult;
            total += amt;
            q.push((amt, color));
        }
    }
    total
}

#[derive(Clone, Debug, Default)]
pub struct Rule<'a> {
    outer: &'a str,
    inner: Vec<Bag<'a>>,
}

pub struct RuleParser;

impl Parse for RuleParser {
    type Out<'a> = Rule<'a>;

    fn parse_from(s: &str) -> Self::Out<'_> {
        let (outer, inner) = s.split_once(" bags contain ").unwrap();
        let inner = inner
            .split(", ")
            .map(Bags::from)
            .filter_map(|bags| match bags {
                Bags::Bag(bag) => Some(bag),
                Bags::NoOther => None,
            })
            .collect();
        Rule { outer, inner }
    }
}

#[derive(Clone, Debug)]
enum Bags<'a> {
    NoOther,
    Bag(Bag<'a>),
}

impl<'a> From<&'a str> for Bags<'a> {
    fn from(s: &'a str) -> Self {
        if s == "no other bags." {
            Self::NoOther
        } else {
            Self::Bag(Bag::from(s.rsplit_once(' ').unwrap().0))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bag<'a> {
    amount: usize,
    color: &'a str,
}

impl<'a> From<&'a str> for Bag<'a> {
    fn from(s: &'a str) -> Self {
        let (amount, color) = s.split_once(' ').unwrap();
        Self {
            amount: amount.parse().unwrap(),
            color,
        }
    }
}

pub struct RuleInput;

impl ProcessInput for RuleInput {
    type In = input!(RuleParser);

    type Out<'a> = Rules<'a>;

    fn process(input: <Self::In as PuzzleInput>::Out<'_>) -> Self::Out<'_> {
        input
            .into_iter()
            .map(|Rule { outer, inner }| (outer, inner))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 222);
        assert_eq!(res2, 13264);
    }

    #[test]
    fn test_ex1() {
        let input = "
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        ";
        assert_eq!((4, 32), Solver::run_on(input));
    }

    #[test]
    fn test_ex2() {
        let input = "
            shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.
        ";
        assert_eq!(126, Solver::run_on(input).1);
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
        b.iter(|| run1(&input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| run2(&input));
    }
}
