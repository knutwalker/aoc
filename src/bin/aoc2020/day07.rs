use std::collections::HashMap;

use aoc::PuzzleInput;

register!(
    "input/day7.txt";
    (input: input!(verbatim RuleInput)) -> usize {
        run1(&input);
        run2(input);
    }
);

pub type Rules = HashMap<String, Vec<(usize, String)>>;

fn run1(input: &Rules) -> usize {
    let mut total = 0;
    for key in input.keys() {
        let mut keys = vec![key.as_str()];
        while !keys.is_empty() {
            let key = keys.pop().unwrap();
            for (_, color) in &input[key] {
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

fn run2(input: Rules) -> usize {
    let mut total = 0;
    let mut q = vec![(1, "shiny gold")];
    while !q.is_empty() {
        let (mult, next) = q.pop().unwrap();
        for (amt, color) in &input[next] {
            let amt = *amt * mult;
            total += amt;
            q.push((amt, color.as_str()));
        }
    }
    total
}

#[derive(Clone, Debug, Default)]
pub struct Rule {
    outer: String,
    inner: Vec<(usize, String)>,
}

impl From<String> for Rule {
    fn from(input: String) -> Self {
        let mut parts = input.splitn(2, " bags contain ");
        let outer = parts.next().unwrap().to_string();
        let contains = parts.next().unwrap();
        let inner = contains
            .split([',', '.'].as_ref())
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .map(|p| match p {
                "no other bags" => None,
                _ => {
                    let mut p = p.splitn(2, ' ');
                    let amt = p.next().unwrap().parse::<usize>().unwrap();
                    let other = p.next().unwrap();
                    let other = other.rsplitn(2, ' ').last().unwrap();
                    Some((amt, other.to_string()))
                }
            })
            .filter_map(|r| r)
            .collect();

        Rule { outer, inner }
    }
}

pub struct RuleInput;

impl PuzzleInput for RuleInput {
    type Out = Rules;

    fn from_input(input: &str) -> Self::Out {
        let input = <aoc::As<Rule> as PuzzleInput>::from_input(input);
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
