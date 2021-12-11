use std::{convert::Infallible, str::FromStr};

register!(
    "input/day6.txt";
    (input: input!(first input!(parse Fishes))) -> usize {
        part1(&input.0);
        part2(&input.0);
    }
);

fn part1(items: &[usize]) -> usize {
    model_fishes(items, 80)
}

fn part2(items: &[usize]) -> usize {
    model_fishes(items, 256)
}

fn model_fishes(initial: &[usize], days: usize) -> usize {
    let mut fishes = [0_usize; 9];

    for &timer in initial {
        fishes[timer] += 1;
    }

    for _ in 0..days {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }

    fishes.into_iter().sum()
}

pub struct Fishes(Vec<usize>);

impl FromStr for Fishes {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(',').flat_map(str::parse::<usize>).collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test_ex() {
        let input = r#"3,4,3,1,2"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 5934);
        assert_eq!(res2, 26_984_457_539);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 360_610);
        assert_eq!(res2, 1_631_629_590_423);
    }
}
