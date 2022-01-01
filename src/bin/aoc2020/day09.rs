register!(
    "input/day9.txt";
    (input: input!(parse u64)) -> u64 {
        run1(&input, 25);
        {
            let part1 = run1(&input, 25);
            run2(&input, part1)
        }
    }
);

fn run1(input: &[u64], pre_len: usize) -> u64 {
    input
        .windows(pre_len + 1)
        .find_map(|win| {
            let (&check, nums) = win.split_last().unwrap();

            if !nums
                .iter()
                .copied()
                .flat_map(|n| {
                    nums.iter()
                        .copied()
                        .filter(move |&m| n != m)
                        .map(move |m| n + m)
                })
                .any(|sum| sum == check)
            {
                return Some(check);
            }

            None
        })
        .unwrap()
}

fn run2(input: &[u64], needle: u64) -> u64 {
    for i in 2..input.len() {
        if let Some(xs) = input.windows(i).find(|xs| xs.iter().sum::<u64>() == needle) {
            let min = xs.iter().min().unwrap();
            let max = xs.iter().max().unwrap();
            return min + max;
        }
    }
    panic!("no solution")
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1_309_761_972);
        assert_eq!(res2, 177_989_832);
    }

    fn input() -> Vec<u64> {
        Solver::parse_input(
            "
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576
        ",
        )
    }

    #[test]
    fn test_ex1() {
        assert_eq!(127, run1(&input(), 5));
        assert_eq!(62, run2(&input(), 127));
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
        b.iter(|| run1(&input, 25));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| {
            let part1 = run1(&input, 25);
            run2(&input, part1)
        });
    }
}
