use aoc::Parse;
use const_combinations::SliceExt;

type Input = u8;
type Output = usize;

register!(
    "input/day6.txt";
    (input: input!(first input!([u8]))) -> Output {
        part1(input);
        part2(input);
    }
);

fn part1(items: &[Input]) -> Output {
    part::<4>(items)
}

fn part2(items: &[Input]) -> Output {
    part::<14>(items)
}

fn part<const N: usize>(items: &[Input]) -> Output {
    items
        .array_windows::<N>()
        .position(|chars| chars.combinations().all(|[a, b]| a != b))
        .unwrap()
        + N
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"
        mjqjpqmgbljsphdztnvjfqwrcgsmlb
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 7);
        assert_eq!(res2, 19);
    }

    #[test]
    fn test_more_ex1() {
        for (input, part1, part2) in [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
        ] {
            let (res1, res2) = Solver::run_on(input);
            assert_eq!(res1, part1);
            assert_eq!(res2, part2);
        }
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1896);
        assert_eq!(res2, 3452);
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
        b.iter(|| part1(input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(input));
    }
}
