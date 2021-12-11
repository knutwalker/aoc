type Input = Vec<u8>;
type Output = u64;

register!(
    "input/day18.txt";
    (input: input!(Input)) -> Output {
        run1(&input);
        run2(input);
    }
);

fn run1(input: &[Input]) -> Output {
    input.iter().map(|l| eval1(&l)).sum()
}

fn run2(input: Vec<Input>) -> Output {
    input.into_iter().map(eval2).sum()
}

fn eval1(line: &[u8]) -> Output {
    fn ev1(ops: &mut impl Iterator<Item = u8>) -> Output {
        let mut value = 0;
        let mut op: fn(Output, Output) -> Output = std::ops::Add::add;

        while let Some(c) = ops.next() {
            match c {
                b')' => break,
                b'+' => op = std::ops::Add::add,
                b'*' => op = std::ops::Mul::mul,
                b'(' => value = op(value, ev1(ops)),
                b'0'..=b'9' => value = op(value, Output::from(c - b'0')),
                _ => {}
            }
        }

        value
    }

    let mut line = line.iter().copied();
    ev1(&mut line)
}

fn eval2(line: impl Into<Input>) -> Output {
    fn ev2(ops: &mut impl Iterator<Item = u8>) -> Output {
        let mut sum = 0;
        let mut product = 1;
        while let Some(c) = ops.next() {
            match c {
                b')' => break,
                b'*' => product *= std::mem::take(&mut sum),
                b'(' => sum += ev2(ops),
                b'0'..=b'9' => sum += Output::from(c - b'0'),
                _ => {}
            }
        }

        product * sum
    }

    let line = line.into();
    let mut line = line.into_iter();
    ev2(&mut line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 11076907812171);
        assert_eq!(res2, 283729053022731);
    }

    #[test]
    fn test_eval() {
        assert_eq!(71, eval1("1 + 2 * 3 + 4 * 5 + 6".as_bytes()));
        assert_eq!(51, eval1("1 + (2 * 3) + (4 * (5 + 6))".as_bytes()));
        assert_eq!(26, eval1("2 * 3 + (4 * 5)".as_bytes()));
        assert_eq!(437, eval1("5 + (8 * 3 + 9 + 3 * 4 * 3)".as_bytes()));
        assert_eq!(
            12240,
            eval1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".as_bytes())
        );
        assert_eq!(
            13632,
            eval1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".as_bytes())
        );
    }

    #[test]
    fn test_eval2() {
        assert_eq!(231, eval2("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(51, eval2("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(46, eval2("2 * 3 + (4 * 5)"));
        assert_eq!(1445, eval2("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(669060, eval2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(
            23340,
            eval2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
