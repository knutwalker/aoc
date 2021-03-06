type Input = usize;
type Output = usize;

register!(
    "input/day25.txt";
    (input: input!(parse Input)) -> Output {
        run1(input[0], input[1]);
        0;
    }
);

fn run1(card_pub: Output, door_pub: Output) -> Output {
    let card_loop_size = (0_usize..)
        .try_fold(1_usize, |value, loop_size| {
            if value == card_pub {
                Err(loop_size)
            } else {
                Ok((value * 7) % 20_201_227)
            }
        })
        .unwrap_err();

    (0..card_loop_size).fold(1_usize, |value, _| (value * door_pub) % 20_201_227)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 18_862_163);
        assert_eq!(res2, 0);
    }

    #[test]
    fn test_ex() {
        assert_eq!(
            (14_897_079, 0),
            Solver::run_on(
                "
                5764801
                17807724
            "
            )
        );
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
        b.iter(|| run1(input[0], input[1]));
    }
}
