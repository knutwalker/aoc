use self::bags::{Bag, Bags};
use aoc::{ProcessInput, PuzzleInput};
use parse_display::FromStr;
use std::collections::HashMap;

register!(
    "input/day7.txt";
    (input: input!(process RuleInput)) -> usize {
        run1(&input);
        run2(&input);
    }
);

pub type Rules = HashMap<String, Vec<Bag>>;

fn run1(input: &Rules) -> usize {
    let mut total = 0;
    for key in input.keys() {
        let mut keys = vec![key.as_str()];
        while !keys.is_empty() {
            let key = keys.pop().unwrap();
            for Bag { color, .. } in &input[key] {
                if color == "shiny gold" {
                    total += 1;
                    keys.clear();
                    break;
                }
                keys.push(color.as_str());
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
            q.push((amt, color.as_str()));
        }
    }
    total
}

#[derive(Clone, Debug, Default, FromStr)]
#[display("{outer} bags contain {inner}.")]
#[from_str(new = Self::from_parsed(outer, inner))]
pub struct Rule {
    outer: String,
    inner: Vec<Bag>,
}

impl Rule {
    #[allow(clippy::needless_pass_by_value)]
    fn from_parsed(outer: String, inner: String) -> Result<Self, parse_display::ParseError> {
        let inner = inner
            .split(", ")
            .map(str::parse::<Bags>)
            .filter_map(|bags| {
                bags.map(|bags| match bags {
                    Bags::Bag(bag) => Some(bag),
                    Bags::NoOther => None,
                })
                .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { outer, inner })
    }
}

#[allow(clippy::use_self)]
mod bags {
    use parse_display::FromStr;

    #[derive(Clone, Debug, FromStr)]
    pub enum Bags {
        #[display("no other bags")]
        NoOther,
        #[from_str(regex = "(?P<0>.+) bags?")]
        Bag(Bag),
    }

    #[derive(Clone, Debug, FromStr)]
    #[from_str(regex = "(?P<amount>[0-9]+) (?P<color>.+)")]
    pub struct Bag {
        pub(super) amount: usize,
        pub(super) color: String,
    }
}

pub struct RuleInput;

impl ProcessInput for RuleInput {
    type In = input!(parse Rule);

    type Out = Rules;

    fn process(input: <Self::In as PuzzleInput>::Out) -> Self::Out {
        input
            .into_iter()
            .map(|Rule { outer, inner }| (outer, inner))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

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
}
