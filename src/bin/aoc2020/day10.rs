use aoc::{ProcessInput, PuzzleInput};
use std::mem;

register!(
    "input/day10.txt";
    (input: input!(process Input)) -> u64 {
        run1(&input);
        run2(input);
    }
);

fn run1(input: &[u64]) -> u64 {
    let (diff1, diff3) = input
        .windows(2)
        .fold((0, 0), |(diff1, diff3), w| match w[1] - w[0] {
            1 => (diff1 + 1, diff3),
            3 => (diff1, diff3 + 1),
            _ => (diff1, diff3),
        });
    diff1 * diff3
}

fn run2(input: Vec<u64>) -> u64 {
    input
        .windows(2)
        .map(|w| w[1] - w[0])
        .scan(0, |run, diff| {
            Some(match diff {
                1 => {
                    *run += 1;
                    None
                }
                _ => match mem::take(run) {
                    2 => Some(2),
                    3 => Some(4),
                    4 => Some(7),
                    _ => None,
                },
            })
        })
        .filter_map(|r| r)
        .product()
}

pub struct Input;

impl ProcessInput for Input {
    type In = input!(parse u64);

    type Out = <Self::In as PuzzleInput>::Out;

    fn process(mut input: <Self::In as PuzzleInput>::Out) -> Self::Out {
        input.sort_unstable();
        input.insert(0, 0);
        input.push(3 + *input.iter().max().unwrap());
        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 2738);
        assert_eq!(res2, 74049191673856);
    }

    #[test]
    fn test_ex1() {
        assert_eq!(
            (220, 19208),
            Solver::run_on(
                "
        28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3
    "
            )
        );
    }
}
