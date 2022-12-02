type Output = u64;

register!(
    "input/day2.txt";
    (input: input!(Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items
        .iter()
        .map(|i| match i.0 {
            ['A', 'X'] => ROCK + DRAW,
            ['A', 'Y'] => PAPER + WIN,
            ['A', 'Z'] => SCISSORS + LOSE,
            ['B', 'X'] => ROCK + LOSE,
            ['B', 'Y'] => PAPER + DRAW,
            ['B', 'Z'] => SCISSORS + WIN,
            ['C', 'X'] => ROCK + WIN,
            ['C', 'Y'] => PAPER + LOSE,
            ['C', 'Z'] => SCISSORS + DRAW,
            otherwise => unreachable!("input: {otherwise:?}"),
        })
        .sum()
}

fn part2(items: &[Input]) -> Output {
    items
        .iter()
        .map(|i| match i.0 {
            ['A', 'X'] => SCISSORS + LOSE,
            ['A', 'Y'] => ROCK + DRAW,
            ['A', 'Z'] => PAPER + WIN,
            ['B', 'X'] => ROCK + LOSE,
            ['B', 'Y'] => PAPER + DRAW,
            ['B', 'Z'] => SCISSORS + WIN,
            ['C', 'X'] => PAPER + LOSE,
            ['C', 'Y'] => SCISSORS + DRAW,
            ['C', 'Z'] => ROCK + WIN,
            otherwise => unreachable!("input: {otherwise:?}"),
        })
        .sum()
}

const ROCK: Output = 1;
const PAPER: Output = 2;
const SCISSORS: Output = 3;
const WIN: Output = 6;
const DRAW: Output = 3;
const LOSE: Output = 0;

pub struct Input([char; 2]);

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let (Some(fst), Some(' '), Some(snd), None) = (
            chars.next(),
            chars.next(),
            chars.next(),
            chars.next(),
        ) else {
            unreachable!("invalid input: {value}");
        };
        Self([fst, snd])
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
        A Y
        B X
        C Z
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 15);
        assert_eq!(res2, 12);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 11063);
        assert_eq!(res2, 10349);
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
