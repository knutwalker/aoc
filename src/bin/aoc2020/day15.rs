type Input = String;
type Output = usize;

register!(
    "input/day15.txt";
    (input: input!(Input)) -> Output {
        run1(&input);
        run2(&input);
    }
);

fn run1(input: &[Input]) -> Output {
    run_any(input, 2020)
}

fn run2(input: &[Input]) -> Output {
    run_any(input, 30_000_000)
}

fn run_any(input: &[Input], stop_at: Output) -> Output {
    let nums = input[0]
        .split(',')
        .map(str::parse::<Output>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut mem = vec![0; stop_at];

    let mut num = *nums.last().unwrap();
    let start_at = nums.len();

    for (i, num) in nums.into_iter().enumerate().rev().skip(1) {
        mem[num] = i + 1;
    }

    for turn in start_at..stop_at {
        let ago = std::mem::replace(&mut mem[num], turn);
        num = if ago == 0 { 0 } else { turn - ago };
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1238);
        assert_eq!(res2, 3_745_954);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(436, run1([String::from("0,3,6")].as_ref()));
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
        b.iter(|| run1(&input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| run2(&input));
    }
}
