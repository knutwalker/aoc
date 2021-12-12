use parse_display::FromStr;
use std::{convert::Infallible, str::FromStr};

register!(
    "input/day8.txt";
    (input: input!(parse Input)) -> usize {
        part1(&input);
        part2(&mut input);
    }
);

fn part1(items: &[Input]) -> usize {
    items
        .iter()
        .flat_map(|Input { test: _, output }| output.0.iter().copied())
        .filter(|d| matches!(d.segments_turned_on(), 2 | 3 | 4 | 7))
        .count()
}

fn part2(items: &mut [Input]) -> usize {
    items
        .iter_mut()
        .map(|Input { test, output }| {
            let mut num = [Digit(0); 10];

            // identifiable by number of segments alone
            num[1] = test.pop(|n| n.has_segments(2));
            num[4] = test.pop(|n| n.has_segments(4));
            num[7] = test.pop(|n| n.has_segments(3));
            num[8] = test.pop(|n| n.has_segments(7));

            // 0, 6, and 9 have 6 segments
            // 9 is the only one to cover all segments from 4
            num[9] = test.pop(|n| n.has_segments(6) && n.contains_segments_from(num[4]));
            // 0 is now the only one to cover 7
            num[0] = test.pop(|n| n.has_segments(6) && n.contains_segments_from(num[7]));
            // 6 is now identifiable by segment count alone
            num[6] = test.pop(|n| n.has_segments(6));

            // 2, 3, 5 are left and all have 5 segments
            // 3 is the only one to cover all segments from 7
            num[3] = test.pop(|n| n.contains_segments_from(num[7]));
            // 5 is the only one covered by 6
            num[5] = test.pop(|n| num[6].contains_segments_from(n));
            // 2 is remaining
            num[2] = test.pop(|_| true);

            output.decode(&num)
        })
        .sum()
}

#[derive(Clone, Debug, FromStr)]
#[display("{test} | {output}")]
pub struct Input {
    test: Digits,
    output: Digits,
}

#[derive(Clone, Debug)]
struct Digits(Vec<Digit>);

impl Digits {
    fn pop(&mut self, select: impl Fn(Digit) -> bool) -> Digit {
        let pos = self.0.iter().copied().position(select).unwrap();
        self.0.swap_remove(pos)
    }

    fn decode(&self, coding: &[Digit]) -> usize {
        self.0
            .iter()
            .filter_map(|d| coding.iter().position(|cd| cd == d))
            .fold(0_usize, |n, d| n * 10 + d)
    }
}

impl FromStr for Digits {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split_ascii_whitespace().map(Digit::from).collect()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Digit(u8);

impl Digit {
    const fn segments_turned_on(self) -> u32 {
        self.0.count_ones()
    }

    const fn has_segments(self, n: u32) -> bool {
        self.segments_turned_on() == n
    }

    const fn contains_segments_from(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl From<&'_ str> for Digit {
    fn from(s: &'_ str) -> Self {
        Self(s.bytes().map(|b| b - b'a').fold(0, |n, b| n | (1 << b)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test_small() {
        let input = r#"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 0);
        assert_eq!(res2, 5353);
    }

    #[test]
    fn test_ex() {
        let input = r#"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 26);
        assert_eq!(res2, 61229);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 534);
        assert_eq!(res2, 1_070_188);
    }
}
