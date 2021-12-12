use parse_display::FromStr;
use std::convert::Infallible;

register!(
    "input/day2.txt";
    (input: input!(parse Command)) -> i64 {
        part1(&input);
        part2(&input);
    }
);

#[allow(clippy::use_self)]
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl std::str::FromStr for Direction {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => unreachable!(),
        })
    }
}

#[derive(Clone, Copy, Debug, FromStr)]
#[display("{0} {1}")]
pub struct Command(Direction, i64);

fn part1(items: &[Command]) -> i64 {
    let (mut horizontal, mut depth) = (0, 0);
    for Command(direction, unit) in items {
        match direction {
            Direction::Forward => horizontal += unit,
            Direction::Down => depth += unit,
            Direction::Up => depth -= unit,
        }
    }
    horizontal * depth
}

fn part2(items: &[Command]) -> i64 {
    let (mut horizontal, mut depth, mut aim) = (0, 0, 0);
    for Command(direction, unit) in items {
        match direction {
            Direction::Forward => {
                horizontal += unit;
                depth += aim * unit;
            }
            Direction::Down => aim += unit,
            Direction::Up => aim -= unit,
        }
    }
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let items = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;
        let (res1, res2) = Solver::run_on(items);
        assert_eq!(res1, 150);
        assert_eq!(res2, 900);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1_698_735);
        assert_eq!(res2, 1_594_785_890);
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
