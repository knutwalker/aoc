use std::ops::{BitAndAssign, BitOrAssign};

use aoc::Parse;

register!(
    "input/day6.txt";
    (input: input!(chunk Answers)) -> u32 {
        run(&input, BitOrAssign::bitor_assign);
        run(&input, BitAndAssign::bitand_assign);
    }
);

#[derive(Copy, Clone, Debug, Default)]
pub struct Answers(u32, bool);

impl Parse for Answers {
    type Out<'a> = Self;
    fn parse_from(input: &str) -> Self {
        Self(
            input
                .bytes()
                .map(|b| b - b'a')
                .fold(0_u32, |answer, b| answer | (1 << b)),
            !input.is_empty(),
        )
    }
}

impl BitOrAssign for Answers {
    fn bitor_assign(&mut self, Self(rhs, _): Self) {
        self.0 |= rhs;
    }
}

impl BitAndAssign for Answers {
    fn bitand_assign(&mut self, Self(rhs, _): Self) {
        if self.1 {
            self.0 &= rhs;
        } else {
            self.0 = rhs;
            self.1 = true;
        }
    }
}

impl Answers {
    fn count(self) -> u32 {
        self.0.count_ones()
    }
}

fn run(input: &[Vec<Answers>], op: impl Fn(&mut Answers, Answers)) -> u32 {
    input
        .iter()
        .map(|block| {
            block
                .iter()
                .fold(Answers::default(), |mut group, line| {
                    op(&mut group, *line);
                    group
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 7110);
        assert_eq!(res2, 3628);
    }

    #[test]
    fn test_ex1() {
        let input = "

            abc

            a
            b
            c

            ab
            ac


            a
            a
            a
            a

            b

";
        assert_eq!((11, 6), Solver::run_on(input));
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
        b.iter(|| run(&input, BitOrAssign::bitor_assign));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| run(&input, BitAndAssign::bitand_assign));
    }
}
