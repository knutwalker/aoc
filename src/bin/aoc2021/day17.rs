use aoc::ProcessInput;
use derive_more::{Add, AddAssign, Sub, SubAssign};
use std::{num::ParseIntError, ops::RangeInclusive, str::FromStr};

type Output = i32;

register!(
    "input/day17.txt";
    (input: input!(process Target)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(input: &[Output]) -> Output {
    input.iter().copied().max().unwrap()
}

const fn part2(input: &[Output]) -> Output {
    input.len() as i32
}

#[derive(Clone, Copy, Debug, Add, AddAssign, Sub, SubAssign)]
struct Pos(i32, i32);

#[derive(Clone, Debug)]
struct Probe {
    pos: Pos,
    vel: Pos,
}

impl Probe {
    fn test(vel: Pos, target: &Target) -> Option<Output> {
        let mut probe = Self {
            pos: Pos(0, 0),
            vel,
        };

        let mut max = 0;
        loop {
            if target.x.contains(&probe.pos.0) && target.y.contains(&probe.pos.1) {
                break Some(max);
            }
            if probe.pos.0 > *target.x.end() || probe.pos.1 < *target.y.start() {
                break None;
            }
            probe.fly();
            max = max.max(probe.pos.1);
        }
    }

    fn fly(&mut self) {
        self.pos += self.vel;
        self.vel -= Pos(self.vel.0.signum(), 1);
    }
}

#[derive(Clone, Debug)]
pub struct Target {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl ProcessInput for Target {
    type In = input!(first input!(parse Self));

    type Out<'a> = Vec<Output>;

    fn process(input: <Self::In as aoc::PuzzleInput>::Out<'_>) -> Self::Out<'_> {
        let input = &input;
        (0..=*input.x.end())
            .flat_map(|x| {
                (*input.y.start()..=input.y.start().abs())
                    .filter_map(move |y| Probe::test(Pos(x, y), input))
            })
            .collect()
    }
}

impl FromStr for Target {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .trim_start_matches("target area: x=")
            .split_once(", y=")
            .unwrap();

        let (start, end) = x.split_once("..").unwrap();
        let x = start.parse()?..=end.parse()?;

        let (start, end) = y.split_once("..").unwrap();
        let y = start.parse()?..=end.parse()?;

        Ok(Self { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"target area: x=20..30, y=-10..-5"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 45);
        assert_eq!(res2, 112);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 12561);
        assert_eq!(res2, 3785);
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
