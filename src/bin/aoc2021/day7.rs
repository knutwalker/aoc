use std::{convert::Infallible, str::FromStr};

register!(
    "input/day7.txt";
    (input: input!(first input!(parse Ferrises))) -> u32 {
        part1(&input.0);
        part2(&input.0);
    }
);

fn part1(items: &[i32]) -> u32 {
    solve(items, |n| n)
}

fn part2(items: &[i32]) -> u32 {
    solve(items, |n| (n * (n + 1)) / 2)
}

fn solve(items: &[i32], cost: impl Fn(u32) -> u32) -> u32 {
    let min = items.iter().copied().min().unwrap();
    let max = items.iter().copied().max().unwrap();
    (min..=max)
        .map(|align| {
            items
                .iter()
                .map(|&num| cost((num - align).unsigned_abs()))
                .sum()
        })
        .min()
        .unwrap()
}

pub struct Ferrises(Vec<i32>);

impl FromStr for Ferrises {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(',').flat_map(str::parse::<i32>).collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"16,1,2,0,4,2,7,1,2,14"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 37);
        assert_eq!(res2, 168);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 344_535);
        assert_eq!(res2, 95_581_659);
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
        b.iter(|| part1(&input.0));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&input.0));
    }
}
