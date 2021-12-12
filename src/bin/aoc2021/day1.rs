register!(
    "input/day1.txt";
    (input: input!(parse u64)) -> usize {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[u64]) -> usize {
    items.array_windows().filter(|[fst, snd]| snd > fst).count()
}

fn part2(items: &[u64]) -> usize {
    items
        .array_windows()
        // when comparing `(a + b + c) < (b + c + d)`, we can eliminate `(b + c)`
        // and only compare `a < d`
        .filter(|[a, _, _, d]| d > a)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let (res1, res2) = Solver::run_on(
            r#"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        "#,
        );
        assert_eq!(res1, 7);
        assert_eq!(res2, 5);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1665);
        assert_eq!(res2, 1702);
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
