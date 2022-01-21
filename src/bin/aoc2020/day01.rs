type Input = u16;
type Output = u64;

register!(
    "input/day1.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(input: &[Input]) -> Output {
    let mut seen = [false; 2021];
    for &item in input {
        assert!(item < 2020);
        let remainder = 2020 - item;
        if seen[usize::from(remainder)] {
            return Output::from(remainder) * Output::from(item);
        }
        seen[item as usize] = true;
    }

    unreachable!()
}

fn part2(mut input: &[Input]) -> Output {
    let mut seen = [false; 2021];

    while let Some(&first) = input.take_first() {
        let target = 2020 - first;

        for &second in input {
            if second < target {
                let third = target - second;
                if seen[usize::from(third)] {
                    return Output::from(third) * Output::from(second) * Output::from(first);
                }
            }
            seen[second as usize] = true;
        }
    }

    unreachable!()
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
