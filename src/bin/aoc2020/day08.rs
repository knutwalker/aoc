use parse_display::FromStr;
use std::collections::HashSet;

register!(
    "input/day8.txt";
    (input: input!(parse Op)) -> i32 {
        run1(&input);
        run2(&input);
    }
);

fn run1(input: &[Op]) -> i32 {
    let mut seen = HashSet::new();
    let mut acc = 0;
    let mut cursor = 0;
    loop {
        if !seen.insert(cursor) {
            return acc;
        }
        match input[cursor as usize] {
            Op::Nop(_) => cursor += 1,
            Op::Jmp(amt) => cursor += amt,
            Op::Acc(amt) => {
                acc += amt;
                cursor += 1;
            }
        }
    }
}

fn run2(input: &[Op]) -> i32 {
    let mut input = input.to_vec();
    for i in 0..input.len() {
        let op = input[i];
        match op {
            Op::Nop(amt) => input[i] = Op::Jmp(amt),
            Op::Jmp(amt) => input[i] = Op::Nop(amt),
            Op::Acc(_) => {}
        }
        if let Some(amt) = try_run2(&input) {
            return amt;
        }
        input[i] = op;
    }
    panic!("no solution found");
}

fn try_run2(input: &[Op]) -> Option<i32> {
    let mut seen = HashSet::new();
    let mut acc = 0;
    let mut cursor = 0;
    loop {
        if cursor == input.len() as i32 {
            return Some(acc);
        }
        if !seen.insert(cursor) {
            return None;
        }
        match input[cursor as usize] {
            Op::Nop(_) => cursor += 1,
            Op::Jmp(amt) => cursor += amt,
            Op::Acc(amt) => {
                acc += amt;
                cursor += 1;
            }
        }
    }
}

#[derive(Debug, Copy, Clone, FromStr)]
#[display(style = "lowercase")]
#[display("{} {0}")]
pub enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1749);
        assert_eq!(res2, 515);
    }

    #[test]
    fn test_ex1() {
        assert_eq!(
            (5, 8),
            Solver::run_on(
                "
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
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
        b.iter(|| run1(&input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| run2(&input));
    }
}
