use std::ops::RangeInclusive;

use aoc::Parse;

type Output = usize;

register!(
    "input/day4.txt";
    (input: input!(Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items
        .iter()
        .filter(|Input([r1, r2])| {
            r1.contains(r2.start()) && r1.contains(r2.end())
                || r2.contains(r1.start()) && r2.contains(r1.end())
        })
        .count()
}

fn part2(items: &[Input]) -> Output {
    items
        .iter()
        .filter(|Input([r1, r2])| r1.contains(r2.end()) || r2.contains(r1.end()))
        .count()
}

pub struct Input([RangeInclusive<u32>; 2]);

impl Parse for Input {
    type Out<'a> = Self;

    fn parse_from(input: &str) -> Self {
        (|| {
            let (start, input) = input.split_once('-')?;
            let (end, input) = input.split_once(',')?;
            let range1 = start.parse().ok()?..=end.parse().ok()?;
            let (start, end) = input.split_once('-')?;
            let range2 = start.parse().ok()?..=end.parse().ok()?;
            Some(Self([range1, range2]))
        })()
        .unwrap()
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
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 2);
        assert_eq!(res2, 4);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 444);
        assert_eq!(res2, 801);
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
