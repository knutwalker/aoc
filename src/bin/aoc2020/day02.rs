use parse_display::FromStr;
use std::ops::RangeInclusive;

register!(
    "input/day2.txt";
    (input: input!(parse PasswordInput)) -> usize {
        part1(&input);
        part2(&input);
    }
);

fn part1(input: &[PasswordInput]) -> usize {
    input.iter().filter(|l| is_valid_01(l)).count()
}

fn part2(input: &[PasswordInput]) -> usize {
    input.iter().filter(|l| is_valid_02(l)).count()
}

#[derive(Debug, Eq, PartialEq, Clone, FromStr)]
#[display("{min}-{max} {letter}: {pass}")]
pub(super) struct PasswordInput {
    min: usize,
    max: usize,
    letter: char,
    pass: String,
}

impl PasswordInput {
    fn range(&self) -> RangeInclusive<usize> {
        self.min..=self.max
    }
}

fn is_valid_01(input: &PasswordInput) -> bool {
    let occurrences = input.pass.chars().filter(|c| *c == input.letter).count();
    input.range().contains(&occurrences)
}

fn is_valid_02(input: &PasswordInput) -> bool {
    let pos1 = input.min - 1;
    let pos2 = input.max - 1;
    let matches = input
        .pass
        .char_indices()
        .filter(|(idx, _)| *idx == pos1 || *idx == pos2)
        .filter(|(_, ch)| *ch == input.letter)
        .count();
    matches == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn parse() {
        test_parse("1-3 a: bbb", 1, 3, 'a', "bbb");
        test_parse("42-42 a: bbb", 42, 42, 'a', "bbb");
        test_parse("42-1337 a: bbb", 42, 1337, 'a', "bbb");
        test_parse("1337-42 a: bbb", 1337, 42, 'a', "bbb");
        test_parse("1-2 x: x", 1, 2, 'x', "x");
        test_parse("1-2 x: xxxxxxxxxxx", 1, 2, 'x', "xxxxxxxxxxx");
    }

    fn test_parse(input: &str, min: usize, max: usize, letter: char, pass: &str) {
        let input = input.parse::<PasswordInput>().unwrap();
        assert_eq!(
            input,
            PasswordInput {
                min,
                max,
                letter,
                pass: String::from(pass)
            }
        );
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 418);
        assert_eq!(res2, 616);
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
