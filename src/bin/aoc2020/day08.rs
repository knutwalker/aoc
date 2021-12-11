use std::collections::HashSet;

register!(
    "input/day8.txt";
    (input: input!(Op)) -> i32 {
        run1(&input);
        run2(input);
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

fn run2(mut input: Vec<Op>) -> i32 {
    for i in 0..input.len() {
        let op = input[i];
        match op {
            Op::Nop(amt) => input[i] = Op::Jmp(amt),
            Op::Jmp(amt) => input[i] = Op::Nop(amt),
            _ => {}
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

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl From<String> for Op {
    fn from(input: String) -> Self {
        let mut parts = input.split_ascii_whitespace();
        let op = parts.next().unwrap();
        let amt = parts.next().unwrap().parse::<i32>().unwrap();
        match op {
            "nop" => Op::Nop(amt),
            "acc" => Op::Acc(amt),
            "jmp" => Op::Jmp(amt),
            op => unreachable!("op : {}", op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

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
}
