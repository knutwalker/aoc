use self::dir::{Dir, Op};
use std::ops::{AddAssign, SubAssign};

type Input = Dir;
type Pos = (isize, isize);

register!(
    "input/day12.txt";
    (input: input!(parse Input)) -> usize {
        Ship1::run(input.iter().copied());
        Ship2::run(input);
    }
);

trait Ship: Default + Into<Pos> {
    fn apply(&mut self, dir: Dir);

    fn run(input: impl IntoIterator<Item = Input>) -> usize {
        let mut s = Self::default();
        for dir in input {
            s.apply(dir);
        }
        s.dist()
    }

    fn dist(self) -> usize {
        let (x, y) = self.into();
        x.abs() as usize + y.abs() as usize
    }
}

struct Ship1 {
    head: Heading,
    pos: Pos,
}

impl Default for Ship1 {
    fn default() -> Self {
        Self {
            head: Heading::East,
            pos: (0, 0),
        }
    }
}

impl From<Ship1> for Pos {
    fn from(s: Ship1) -> Self {
        s.pos
    }
}

impl Ship for Ship1 {
    fn apply(&mut self, dir: Dir) {
        match dir.op {
            Op::N => Heading::North.apply(dir.num, &mut self.pos),
            Op::E => Heading::East.apply(dir.num, &mut self.pos),
            Op::S => Heading::South.apply(dir.num, &mut self.pos),
            Op::W => Heading::West.apply(dir.num, &mut self.pos),
            Op::L => self.head -= dir.num,
            Op::R => self.head += dir.num,
            Op::F => self.head.apply(dir.num, &mut self.pos),
        }
    }
}

#[derive(Debug)]
struct Ship2 {
    pos: (isize, isize),
    wp: (isize, isize),
}

impl Default for Ship2 {
    fn default() -> Self {
        Self {
            pos: (0, 0),
            wp: (10, -1),
        }
    }
}

impl From<Ship2> for Pos {
    fn from(s: Ship2) -> Self {
        s.pos
    }
}

impl Ship for Ship2 {
    fn apply(&mut self, dir: Dir) {
        match dir.op {
            Op::N => Heading::North.apply(dir.num, &mut self.wp),
            Op::E => Heading::East.apply(dir.num, &mut self.wp),
            Op::S => Heading::South.apply(dir.num, &mut self.wp),
            Op::W => Heading::West.apply(dir.num, &mut self.wp),
            Op::L => Heading::from(dir.num).rotate_left(&mut self.wp),
            Op::R => Heading::from(dir.num).rotate_right(&mut self.wp),
            Op::F => {
                self.pos.0 += self.wp.0 * dir.num;
                self.pos.1 += self.wp.1 * dir.num;
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn apply(self, dist: isize, (x, y): &mut Pos) {
        match self {
            Heading::North => *y -= dist,
            Heading::East => *x += dist,
            Heading::South => *y += dist,
            Heading::West => *x -= dist,
        }
    }

    // E+X => N-Y
    // N-Y => W-X
    // W-X => S+Y
    // S+Y => E+X
    fn rotate_left(self, pos: &mut Pos) {
        for _ in 0..(self as u8) {
            let (x, y) = *pos;
            *pos = (y, -x);
        }
    }

    // E+X => S+Y
    // S+Y => W-X
    // W-X => N-Y
    // N-Y => E+X
    fn rotate_right(self, pos: &mut Pos) {
        for _ in 0..(self as u8) {
            let (x, y) = *pos;
            *pos = (-y, x);
        }
    }
}

impl From<isize> for Heading {
    fn from(x: isize) -> Self {
        Self::from((x / 90) as u8)
    }
}

impl From<u8> for Heading {
    fn from(x: u8) -> Self {
        match x % 4 {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => unreachable!(),
        }
    }
}

impl AddAssign<isize> for Heading {
    fn add_assign(&mut self, rhs: isize) {
        *self = Self::from((*self as u8) + (Self::from(rhs) as u8));
    }
}

impl SubAssign<isize> for Heading {
    fn sub_assign(&mut self, rhs: isize) {
        *self = Self::from(((*self as u8) + 4) - (Self::from(rhs) as u8));
    }
}

#[allow(clippy::use_self)]
mod dir {
    use parse_display::FromStr;

    #[derive(Debug, Copy, Clone, FromStr)]
    #[display("{}")]
    pub enum Op {
        N,
        E,
        S,
        W,
        L,
        R,
        F,
    }

    #[derive(Debug, Copy, Clone, FromStr)]
    #[from_str(regex = "(?P<op>[NESWLRF])(?P<num>[0-9]+)")]
    pub struct Dir {
        pub(super) op: Op,
        pub(super) num: isize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 820);
        assert_eq!(res2, 66614);
    }

    #[test]
    fn test_ex1() {
        assert_eq!(
            (25, 286),
            Solver::run_on(
                "
        F10
        N3
        F7
        R90
        F11
        "
            )
        );
    }
}
