use aoc::ProcessInput;
use std::{cmp::Ordering, num::ParseIntError, str::FromStr};

type Input = Ins;
type Output = i64;
type Num = i32;

register!(
    "input/day24.txt";
    (input: input!(process Op)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(ops: &[Op]) -> Output {
    prog::<Max>(ops)
}

fn part2(ops: &[Op]) -> Output {
    prog::<Min>(ops)
}

struct Min;

struct Max;

trait Target {
    fn zero() -> Num;

    fn reduce(w: Num, x: Num, current: &mut Num, top: &mut Num);
}

impl Target for Min {
    fn zero() -> Num {
        1
    }

    fn reduce(w: Num, x: Num, current: &mut Num, top: &mut Num) {
        match x.cmp(&w) {
            Ordering::Less => *top += w - x,
            Ordering::Equal => {}
            Ordering::Greater => *current = x,
        }
    }
}

impl Target for Max {
    fn zero() -> Num {
        9
    }

    fn reduce(w: Num, x: Num, current: &mut Num, top: &mut Num) {
        match x.cmp(&w) {
            Ordering::Less => *current = x,
            Ordering::Equal => {}
            Ordering::Greater => *top -= x - w,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct V {
    pos: usize,
    rest: Num,
}

impl Default for V {
    fn default() -> Self {
        Self {
            pos: usize::MAX,
            rest: Num::default(),
        }
    }
}

fn prog<T: Target>(ops: &[Op]) -> Output {
    assert_eq!(ops.len(), 14);

    let mut inputs = [T::zero(); 14];
    let mut vs = [V::default(); 7];
    let mut top = 0;

    for (idx, op) in ops.iter().enumerate() {
        if op.inc {
            let v = &mut vs[top];
            v.pos = idx;
            v.rest = inputs[idx] + op.add_y;
            top += 1;
        } else {
            top -= 1;
            let v = vs[top];
            let x = v.rest + op.add_x;
            let w = inputs[idx];

            let (head, tail) = inputs.split_at_mut(idx);
            let (_, head) = head.split_at_mut(v.pos);

            T::reduce(w, x, &mut tail[0], &mut head[0]);
        }
    }

    #[cfg(debug_assertions)]
    {
        let z = test(ops, inputs);

        poop!("vs = {:#?}", vs);
        poop!("top = {:#?}", top);
        poop!("inputs = {:#?}", inputs);
        poop!("z = {}", z);

        assert_eq!(top, 0);
        assert_eq!(z, 0);
    }

    inputs
        .into_iter()
        .fold(0, |res, w| res * 10 + Output::from(w))
}

#[cfg(debug_assertions)]
fn test(ops: &[Op], input: impl IntoIterator<Item = Num>) -> Num {
    ops.iter().zip(input).fold(0, |z, (op, w)| {
        if op.inc {
            26 * z + w + op.add_y
        } else {
            let x = (z % 26) + op.add_x;
            let z = z / 26;
            if x == w {
                z
            } else {
                26 * z + w + op.add_y
            }
        }
    })
}

pub enum Reg {
    W,
    X,
    Y,
    Z,
}

pub enum Var {
    Imm(Num),
    Reg(Reg),
}

pub enum Ins {
    Inp(Reg),
    Add(Reg, Var),
    Mul(Reg, Var),
    Div(Reg, Var),
    Mod(Reg, Var),
    Eql(Reg, Var),
}

impl FromStr for Reg {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "w" => Self::W,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            _ => return Err(()),
        })
    }
}

impl FromStr for Var {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<Reg>() {
            Ok(reg) => Self::Reg(reg),
            Err(_) => Self::Imm(s.parse()?),
        })
    }
}

impl FromStr for Ins {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ins, args) = s.split_once(' ').unwrap();
        let (a, b) = args.split_once(' ').unwrap_or((args, ""));
        let a = a.parse::<Reg>().unwrap();
        let b = b.parse::<Var>();
        Ok(match ins {
            "inp" => Self::Inp(a),
            "add" => Self::Add(a, b?),
            "mul" => Self::Mul(a, b?),
            "div" => Self::Div(a, b?),
            "mod" => Self::Mod(a, b?),
            "eql" => Self::Eql(a, b?),
            _ => unreachable!("ins: {}", s),
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Op {
    inc: bool,
    add_x: Num,
    add_y: Num,
}

impl ProcessInput for Op {
    type In = input!(parse Input);

    type Out<'a> = Vec<Self>;

    fn process(input: <Self::In as aoc::PuzzleInput>::Out<'_>) -> Self::Out<'_> {
        input
            .into_iter()
            .chain(std::iter::once(Ins::Inp(Reg::W)))
            .scan((Self::default(), None::<bool>), |(cs, kind), ins| {
                match ins {
                    Ins::Inp(_) => {
                        let cs = std::mem::take(cs);
                        return Some(kind.take().map(|inc| Self { inc, ..cs }));
                    }
                    Ins::Div(Reg::Z, Var::Imm(div_z)) => *kind = Some(div_z == 1),
                    Ins::Add(Reg::X, Var::Imm(add_x)) => cs.add_x = add_x,
                    Ins::Add(Reg::Y, Var::Imm(add_y)) => cs.add_y = add_y,
                    _ => {}
                };
                Some(None)
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 98_491_959_997_994);
        assert_eq!(res2, 61_191_516_111_321);
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
        b.iter(|| part1(&input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&input));
    }
}
