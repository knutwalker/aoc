use std::cmp::Reverse;

use aoc::ProcessInput;

type Input = u64;
type Output = Input;

register!(
    "input/day1.txt";
    (input: input!(blocks input!(process Calories))) -> Output {
        part1(&input);
        part2(&mut input);
    }
);

fn part1(items: &[Input]) -> Output {
    *items.iter().max().unwrap()
}

fn part2(items: &mut [Input]) -> Output {
    items
        .select_nth_unstable_by_key(3, |n| Reverse(*n))
        .0
        .iter()
        .sum()
}

pub struct Calories;

impl ProcessInput for Calories {
    type In = input!(parse Input);

    type Out = Input;

    fn process(input: <Self::In as aoc::PuzzleInput>::Out) -> Self::Out {
        input.into_iter().sum::<u64>()
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
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 24000);
        assert_eq!(res2, 45000);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 70116);
        assert_eq!(res2, 206_582);
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
        let mut input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&mut input));
    }
}
