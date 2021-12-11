register!(
    "input/day21.txt";
    (input: input!(parse u64)) -> usize {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[u64]) -> usize {
    0
}

fn part2(items: &[u64]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test_ex() {
        let input = r#""#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 0);
        assert_eq!(res2, 0);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 0);
        assert_eq!(res2, 0);
    }
}
