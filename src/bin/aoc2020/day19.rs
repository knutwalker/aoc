use std::collections::HashMap;

use aoc::ProcessInput;

type Input = String;
type Output = usize;
type Rules = HashMap<u8, Rule>;

register!(
    "input/day19.txt";
    (input: input!(process RulesInput)) -> Output {
        run1(&input.0, &input.1);
        run2(&input.0, &input.1);
    }
);

#[derive(Debug, Clone)]
enum Rule {
    Lit(u8),
    Or(Box<[Box<[u8]>]>),
}

fn matches_rule<'b>(rules: &Rules, rule: &Rule, bs: &'b [u8]) -> (bool, &'b [u8]) {
    match rule {
        Rule::Lit(x) => match bs.split_first() {
            Some((b, rest)) => (b == x, rest),
            None => (false, &[]),
        },
        Rule::Or(alternatives) => {
            'outer: for seq in &**alternatives {
                let mut input = bs;
                for rule in &**seq {
                    let (matches, rest) = matches_rule(rules, &rules[rule], input);
                    if !matches {
                        continue 'outer;
                    }
                    input = rest;
                }
                return (true, input);
            }
            (false, bs)
        }
    }
}

fn run_any<F>(rules: &Rules, messages: &[Input], check: F) -> Output
where
    F: for<'b> Fn(&Rules, &'b [u8]) -> (bool, &'b [u8]),
{
    messages
        .iter()
        .map(|msg| {
            let bs = msg.as_bytes();
            let (matches, rest) = check(&rules, bs);
            (matches && rest.is_empty()) as Output
        })
        .sum()
}

fn run1(rules: &Rules, messages: &[Input]) -> Output {
    run_any(rules, messages, |rs, input| {
        matches_rule(rs, &rs[&0], input)
    })
}

// 0:  8   42+    ~    11: 42{n, n>=1} ~ 31{=n}
fn matches_new_rule0<'b>(rules: &Rules, bs: &'b [u8]) -> (bool, &'b [u8]) {
    let mut input = &bs[..];
    let mut matches42 = 0;
    loop {
        let (matches, rest) = matches_rule(&rules, &rules[&42], input);
        if !matches {
            break;
        }
        matches42 += 1;
        input = rest;
    }

    let mut matches31 = 0;
    loop {
        let (matches, rest) = matches_rule(&rules, &rules[&31], input);
        if !matches {
            break;
        }
        matches31 += 1;
        input = rest;
    }

    (
        matches42 >= 2 && matches31 >= 1 && matches42 > matches31,
        input,
    )
}

fn run2(rules: &Rules, messages: &[Input]) -> Output {
    run_any(rules, messages, |rs, input| matches_new_rule0(rs, input))
}

pub struct RulesInput(Rules, Vec<Input>);

impl ProcessInput for RulesInput {
    type In = input!(chunk Input);

    type Out = Self;

    fn process(mut input: <Self::In as aoc::PuzzleInput>::Out) -> Self::Out {
        let messages = input.pop().unwrap();
        let rules = input
            .pop()
            .unwrap()
            .into_iter()
            .map(|r| {
                let mut r = r.splitn(2, ": ");
                let nr = r.next().unwrap().parse().unwrap();
                let r = r.next().unwrap();

                let rule = if r.starts_with('"') {
                    Rule::Lit(r.as_bytes()[1])
                } else {
                    Rule::Or(
                        r.split(" | ")
                            .map(|r| {
                                r.split(' ')
                                    .map(str::parse::<u8>)
                                    .map(Result::unwrap)
                                    .collect::<Vec<_>>()
                                    .into_boxed_slice()
                            })
                            .collect::<Vec<_>>()
                            .into_boxed_slice(),
                    )
                };

                (nr, rule)
            })
            .collect::<Rules>();

        Self(rules, messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 139);
        assert_eq!(res2, 289);
    }

    #[test]
    fn test_pt1() {
        let input = Solver::parse_input(
            r#"
        0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"

        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb
    "#,
        );
        assert_eq!(2, run1(&input.0, &input.1));
    }

    #[test]
    fn test_pt1_2() {
        assert_eq!(
            (3, 12),
            Solver::run_on(
                r#"
                    42: 9 14 | 10 1
                    9: 14 27 | 1 26
                    10: 23 14 | 28 1
                    1: "a"
                    11: 42 31
                    5: 1 14 | 15 1
                    19: 14 1 | 14 14
                    12: 24 14 | 19 1
                    16: 15 1 | 14 14
                    31: 14 17 | 1 13
                    6: 14 14 | 1 14
                    2: 1 24 | 14 4
                    0: 8 11
                    13: 14 3 | 1 12
                    15: 1 | 14
                    17: 14 2 | 1 7
                    23: 25 1 | 22 14
                    28: 16 1
                    4: 1 1
                    20: 14 14 | 1 15
                    3: 5 14 | 16 1
                    27: 1 6 | 14 18
                    14: "b"
                    21: 14 1 | 1 14
                    25: 1 1 | 1 14
                    22: 14 14
                    8: 42
                    26: 14 22 | 1 20
                    18: 15 15
                    7: 14 5 | 1 21
                    24: 14 1

                    abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
                    bbabbbbaabaabba
                    babbbbaabbbbbabbbbbbaabaaabaaa
                    aaabbbbbbaaaabaababaabababbabaaabbababababaaa
                    bbbbbbbaaaabbbbaaabbabaaa
                    bbbababbbbaaaaaaaabbababaaababaabab
                    ababaaaaaabaaab
                    ababaaaaabbbaba
                    baabbaaaabbaaaababbaababb
                    abbbbabbbbaaaababbbbbbaaaababb
                    aaaaabbaabaaaaababaa
                    aaaabbaaaabbaaa
                    aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
                    babaaabbbaaabaababbaabababaaab
                    aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
            "#,
            )
        );
    }
}
