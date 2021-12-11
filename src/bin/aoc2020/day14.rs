use parse_display::FromStr;
use std::collections::HashMap;

type Input = Op;
type Output = u64;

register!(
    "input/day14.txt";
    (input: input!(parse Input)) -> Output {
        run1(&input);
        run2(input);
    }
);

#[derive(Debug, Default)]
struct State1 {
    memory: HashMap<u64, u64>,
    keep_mask: u64,
    set_mask: u64,
}

fn run1(input: &[Input]) -> Output {
    input
        .iter()
        .fold(State1::default(), |mut s, op| {
            match op {
                Op::Mask(mask) => {
                    s.keep_mask = mask
                        .bytes()
                        .rev()
                        .enumerate()
                        .filter_map(|(i, b)| if b == b'X' { Some(1 << i) } else { None })
                        .fold(0_u64, |sum, digit| sum | digit);

                    s.set_mask = mask
                        .bytes()
                        .rev()
                        .enumerate()
                        .filter_map(|(i, b)| if b == b'1' { Some(1 << i) } else { None })
                        .fold(0_u64, |sum, digit| sum | digit);
                }
                Op::Mem(addr, value) => {
                    let value = (value & s.keep_mask) | s.set_mask;
                    s.memory.insert(*addr, value);
                }
            };
            s
        })
        .memory
        .values()
        .sum()
}

#[derive(Debug, Default)]
struct State2 {
    memory: HashMap<u64, u64>,
    set_mask: u64,
    keep_masks: Vec<u64>,
}

fn run2(input: Vec<Input>) -> Output {
    input
        .into_iter()
        .fold(State2::default(), |mut s, op| {
            match op {
                Op::Mask(mask) => {
                    s.keep_masks = mask
                        .bytes()
                        .rev()
                        .enumerate()
                        .filter_map(|(i, b)| if b == b'X' { Some(i) } else { None })
                        .fold(vec![u64::max_value()], |ms, i| {
                            ms.into_iter()
                                .flat_map(|m| vec![m & !(1 << i), m | (1 << i)])
                                .collect()
                        });
                    s.set_mask = mask
                        .bytes()
                        .rev()
                        .enumerate()
                        .filter_map(|(i, b)| if b == b'0' { None } else { Some(1 << i) })
                        .fold(0_u64, |sum, digit| sum | digit);
                }
                Op::Mem(addr, value) => {
                    let addr = addr | s.set_mask;
                    for &mask in &s.keep_masks {
                        let addr = addr & mask;
                        s.memory.insert(addr, value);
                    }
                }
            };
            s
        })
        .memory
        .values()
        .sum()
}

#[derive(Debug, Clone, FromStr)]
pub enum Op {
    #[display("mask = {0}")]
    Mask(String),
    #[display("mem[{0}] = {1}")]
    Mem(u64, u64),
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 8_332_632_930_672);
        assert_eq!(res2, 4_753_238_784_664);
    }

    #[test]
    fn test_pt1() {
        let input = "
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0
        ";
        assert_eq!(165, run1(&Solver::parse_input(input)));
    }

    #[test]
    fn test_pt2() {
        let input = "
            mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1
        ";
        assert_eq!(208, run2(Solver::parse_input(input)));
    }
}
