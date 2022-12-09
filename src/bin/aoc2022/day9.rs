use aoc::Parse;
use fxhash::{FxBuildHasher, FxHashSet};

type Output = usize;

register!(
    "input/day9.txt";
    (input: input!(Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    part::<1>(items)
}

fn part2(items: &[Input]) -> Output {
    part::<9>(items)
}

fn part<const N: usize>(items: &[Input]) -> Output {
    let mut head = (0, 0);
    let mut tails = [(0, 0); N];
    let mut visited = FxHashSet::with_capacity_and_hasher(4096, FxBuildHasher::default());

    visited.insert(tails[N - 1]);

    for item in items {
        for _ in 0..item.amount {
            head.0 += item.dir.0;
            head.1 += item.dir.1;

            let mut prev = head;
            for tail in &mut tails {
                follow(prev, tail);
                prev = *tail;
            }
            visited.insert(tails[N - 1]);
        }
    }

    visited.len()
}

fn follow(head: (i32, i32), mut tail: &mut (i32, i32)) {
    if head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1 {
        return;
    }

    tail.0 += (head.0 - tail.0).signum();
    tail.1 += (head.1 - tail.1).signum();
}

pub struct Input {
    dir: (i32, i32),
    amount: u32,
}

impl Parse for Input {
    type Out<'a> = Self;

    fn parse_from(s: &str) -> Self::Out<'_> {
        let (dir, amount) = s.split_once(' ').unwrap();
        let dir = match dir {
            "L" => (-1, 0),
            "R" => (1, 0),
            "D" => (0, -1),
            "U" => (0, 1),
            _ => unreachable!(),
        };
        let amount = amount.parse().unwrap();
        Self { dir, amount }
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
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 13);
        assert_eq!(res2, 1);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 5981);
        assert_eq!(res2, 2352);
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
