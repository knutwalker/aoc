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
    use aoc::SolutionExt;

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
}
