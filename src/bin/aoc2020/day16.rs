use parse_display::FromStr;
use std::{collections::HashSet, ops::RangeInclusive};

type Output = usize;

register!(
    "input/day16.txt";
    (input: input!(chunk str)) -> Output {
        run1(&input);
        run2(&input);
    }
);

#[derive(Debug, Clone)]
struct Rule {
    departure: bool,
    r1: RangeInclusive<Output>,
    r2: RangeInclusive<Output>,
}

impl Rule {
    fn valid(&self, num: Output) -> bool {
        self.r1.contains(&num) || self.r2.contains(&num)
    }
}

impl From<RuleAlternative> for Rule {
    fn from(rr: RuleAlternative) -> Self {
        let (departure, RangeAlternative { a, b }) = match rr {
            RuleAlternative::Departure(range) => (true, range),
            RuleAlternative::Other(range) => (false, range),
        };
        let r1 = a.min..=a.max;
        let r2 = b.min..=b.max;
        Self { departure, r1, r2 }
    }
}

#[derive(Clone, Debug, FromStr)]
enum RuleAlternative {
    #[from_str(regex = "departure [a-z]+: (?P<0>.+)")]
    Departure(RangeAlternative),
    #[from_str(regex = "[a-z ]+: (?P<0>.+)")]
    Other(RangeAlternative),
}

#[derive(Clone, Copy, Debug, FromStr)]
#[display("{a} or {b}")]
struct RangeAlternative {
    a: RuleRange,
    b: RuleRange,
}

#[derive(Clone, Copy, Debug, FromStr)]
#[display("{min}-{max}")]
struct RuleRange {
    min: Output,
    max: Output,
}

fn run1(input: &[Vec<&str>]) -> Output {
    let mut chunks = input.iter();
    let rules = chunks.next().unwrap();
    let _my_tickets = chunks.next().unwrap();
    let other_tickets = chunks.next().unwrap();

    let rules = rules
        .iter()
        .map(|s| s.parse::<RuleAlternative>().unwrap())
        .map(Rule::from)
        .collect::<Vec<_>>();

    other_tickets
        .iter()
        .skip(1)
        .map(|t| {
            t.split(',')
                .map(str::parse::<Output>)
                .map(Result::unwrap)
                .filter(|n| !rules.iter().any(|r| r.valid(*n)))
                .sum::<Output>()
        })
        .sum()
}

fn run2(input: &[Vec<&str>]) -> Output {
    let mut chunks = input.iter();

    let rules = chunks.next().unwrap();
    let rules = rules
        .iter()
        .map(|s| s.parse::<RuleAlternative>().unwrap())
        .map(Rule::from)
        .collect::<Vec<_>>();

    let my = chunks.next().unwrap();
    let my = my
        .get(1)
        .unwrap()
        .split(',')
        .map(str::parse::<Output>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let others = chunks.next().unwrap();
    let others = others
        .iter()
        .skip(1)
        .filter_map(|t| {
            let nums = t
                .split(',')
                .map(str::parse::<Output>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            if nums.iter().any(|n| !rules.iter().any(|r| r.valid(*n))) {
                None
            } else {
                Some(nums)
            }
        })
        .collect::<Vec<_>>();

    let num_rules = rules.len();
    let mut solved = HashSet::new();
    let mut rules = rules.into_iter().map(Some).collect::<Vec<_>>();
    let mut rules_in_order = Vec::with_capacity(num_rules);
    rules_in_order.resize_with(rules.len(), || None::<Rule>);
    while solved.len() != num_rules {
        for rule in &mut rules {
            if let Some(r) = rule {
                let candidates = (0..num_rules)
                    .filter(|idx| !solved.contains(idx))
                    .filter(|&idx| others.iter().all(|t| r.valid(t[idx])))
                    .collect::<Vec<_>>();

                if let &[idx] = &candidates[..] {
                    rules_in_order[idx] = rule.take();
                    solved.insert(idx);
                } else {
                    assert!(!candidates.is_empty(), "rule not valid anywhere: {rule:#?}");
                }
            }
        }
    }

    rules_in_order
        .into_iter()
        .flatten()
        .enumerate()
        .filter_map(|(idx, r)| if r.departure { Some(my[idx]) } else { None })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 18227);
        assert_eq!(res2, 2_355_350_878_831);
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
            run2(&Solver::parse_input(
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
