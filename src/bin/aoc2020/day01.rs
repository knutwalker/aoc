register!(
    "input/day1.txt";
    (input: input!(parse u64)) -> u64 {
        part1(&input);
        part2(&input);
    }
);

fn part1(input: &[u64]) -> u64 {
    let mut input = input.to_vec();
    input.sort_unstable();
    find_pair_and_prod(2020, &input).expect("no solution")
}

fn part2(input: &[u64]) -> u64 {
    let mut input = input.to_vec();
    input.sort_unstable();
    find_triple_and_prod(2020, &input)
}

fn find_triple_and_prod(target_sum: u64, mut items: &[u64]) -> u64 {
    while let Some((&item, rest)) = items.split_first() {
        if let Some(remainder) = target_sum.checked_sub(item) {
            if let Some(prod) = find_pair_and_prod(remainder, rest) {
                return prod * item;
            }
        };
        items = rest;
    }
    panic!("no solution")
}

fn find_pair_and_prod(target_sum: u64, mut items: &[u64]) -> Option<u64> {
    while let Some((&item, rest)) = items.split_first() {
        if let Some(remainder) = target_sum.checked_sub(item) {
            if let Ok(index) = rest.binary_search(&remainder) {
                return Some(item * rest[index]);
            }
        };
        items = rest;
    }
    None
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
        1721
        979
        366
        299
        675
        1456
        "#,
        );
        assert_eq!(res1, 514_579);
        assert_eq!(res2, 241_861_950);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 805_731);
        assert_eq!(res2, 192_684_960);
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
