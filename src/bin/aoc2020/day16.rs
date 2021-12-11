use std::{collections::HashSet, ops::RangeInclusive};

type Input = String;
type Output = usize;

register!(
    "input/day16.txt";
    (input: input!(chunk Input)) -> Output {
        run1(&input);
        run2(input);
    }
);

#[derive(Debug, Clone)]
struct Rule {
    departure: bool,
    r1: RangeInclusive<Output>,
    r2: RangeInclusive<Output>,
}

impl Rule {
    fn valid(&self, num: &Output) -> bool {
        self.r1.contains(num) || self.r2.contains(num)
    }
}

impl From<&str> for Rule {
    fn from(r: &str) -> Self {
        let mut r = r.split(':');
        let departure = r.next().unwrap().starts_with("departure ");
        let r = r.next().unwrap().trim();
        let mut r = r.split(" or ");
        let r1 = r.next().unwrap();
        let mut r1 = r1.split('-').map(str::parse::<Output>).map(Result::unwrap);
        let r1 = r1.next().unwrap()..=r1.next().unwrap();
        let r2 = r.next().unwrap();
        let mut r2 = r2.split('-').map(str::parse::<Output>).map(Result::unwrap);
        let r2 = r2.next().unwrap()..=r2.next().unwrap();
        Self { departure, r1, r2 }
    }
}

fn run1(input: &[Vec<Input>]) -> Output {
    let mut chunks = input.iter();
    let rules = chunks.next().unwrap();
    let _my_tickets = chunks.next().unwrap();
    let other_tickets = chunks.next().unwrap();

    let rules = rules.iter().map(|s| Rule::from(&**s)).collect::<Vec<_>>();

    other_tickets
        .into_iter()
        .skip(1)
        .map(|t| {
            t.split(',')
                .map(str::parse::<Output>)
                .map(Result::unwrap)
                .filter(|n| !rules.iter().any(|r| r.valid(n)))
                .sum::<Output>()
        })
        .sum()
}

fn run2(input: Vec<Vec<Input>>) -> Output {
    let mut chunks = input.into_iter();

    let rules = chunks.next().unwrap();
    let rules = rules
        .into_iter()
        .map(|s| Rule::from(&*s))
        .collect::<Vec<_>>();

    let my = chunks.next().unwrap();
    let my = my
        .into_iter()
        .nth(1)
        .unwrap()
        .split(',')
        .map(str::parse::<Output>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let others = chunks.next().unwrap();
    let others = others
        .into_iter()
        .skip(1)
        .filter_map(|t| {
            let nums = t
                .split(',')
                .map(str::parse::<Output>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            if nums.iter().any(|n| !rules.iter().any(|r| r.valid(n))) {
                None
            } else {
                Some(nums)
            }
        })
        .collect::<Vec<_>>();

    let num_rules = rules.len();
    let mut solved = HashSet::new();
    let mut rules = rules.into_iter().map(|x| Some(x)).collect::<Vec<_>>();
    let mut rules_in_order = Vec::with_capacity(num_rules);
    rules_in_order.resize_with(rules.len(), || None::<Rule>);
    while solved.len() != num_rules {
        for rule in &mut rules {
            if let Some(r) = rule {
                let candidates = (0..num_rules)
                    .filter(|idx| !solved.contains(idx))
                    .filter(|&idx| others.iter().all(|t| r.valid(&t[idx])))
                    .collect::<Vec<_>>();

                if let &[idx] = &candidates[..] {
                    rules_in_order[idx] = rule.take();
                    solved.insert(idx);
                } else {
                    if candidates.is_empty() {
                        panic!("rule not valid anywhere: {:#?}", rule)
                    }
                }
            }
        }
    }

    rules_in_order
        .into_iter()
        .map(|r| r.unwrap())
        .enumerate()
        .filter_map(|(idx, r)| if r.departure { Some(my[idx]) } else { None })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 18227);
        assert_eq!(res2, 2355350878831);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(
            71,
            run1(&Solver::parse_input(
                "
                class: 1-3 or 5-7
                row: 6-11 or 33-44
                seat: 13-40 or 45-50

                your ticket:
                7,1,14

                nearby tickets:
                7,3,47
                40,4,50
                55,2,20
                38,6,12
            ",
            ))
        );
    }

    #[test]
    fn test_pt2() {
        assert_eq!(
            1,
            run2(Solver::parse_input(
                "
                class: 0-1 or 4-19
                row: 0-5 or 8-19
                seat: 0-13 or 16-19

                your ticket:
                11,12,13

                nearby tickets:
                3,9,18
                15,1,5
                5,14,9
            ",
            ))
        );
    }
}
