use aoc::{MinMax, Parse, PuzzleInput};
use bitvec::prelude::{bitarr, BitArray, Lsb0};

type Output = usize;

register!(
    "input/day14.txt";
    (input: input!(verbatim Map)) -> Output {
        part1(&mut input);
        part2(&mut input);
    }
);

const SPAWN: Coord = Coord { x: 500, y: 0 };

macro_rules! try_move {
    ($blocked:ident, $sand:ident) => {
        // try down
        let down = $sand.move_y(1);
        if $blocked.is_free(down) {
            $sand = down;
            continue;
        }

        // try down-left
        let down_left = down.move_x(-1);
        if $blocked.is_free(down_left) {
            $sand = down_left;
            continue;
        }

        // try down-right
        let down_right = down.move_x(1);
        if $blocked.is_free(down_right) {
            $sand = down_right;
            continue;
        }

        break $sand;
    };
}

fn part1(map: &mut Map) -> Output {
    let left = map.x_bounds.min;
    let right = map.x_bounds.max;
    let bottom = map.y_bounds.max;

    let blocked = &mut map.blocked;
    let mut drops = map.drops;

    loop {
        let mut sand = SPAWN;

        let resting = loop {
            if sand.x < left || sand.x > right || sand.y > bottom {
                map.drops = drops;
                return drops;
            }

            try_move!(blocked, sand);
        };

        drops += 1;
        blocked.block(resting);
    }
}

fn part2(map: &mut Map) -> Output {
    let bottom = map.y_bounds.max + 1;

    let blocked = &mut map.blocked;
    let mut drops = map.drops;

    loop {
        let mut sand = SPAWN;

        let resting = loop {
            if sand.y == bottom {
                break sand;
            }

            try_move!(blocked, sand);
        };

        drops += 1;

        if resting == SPAWN {
            return drops;
        }

        blocked.block(resting);
    }
}

#[derive(Clone)]
pub struct Map {
    blocked: Cave,
    x_bounds: MinMax<u32>,
    y_bounds: MinMax<u32>,
    drops: usize,
}

#[derive(Clone)]
struct Cave(BitArray<[u64; 1250]>);

impl Cave {
    // actual boundaries are about [300,700] for x and [0, 200] for y
    const MIN_X: u32 = 300;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    fn new() -> Self {
        let size = Self::WIDTH * Self::HEIGHT;
        assert_eq!(size, 80_000);

        Self(bitarr![u64, Lsb0; 0; 80_000])
    }

    fn to_index(coord: Coord) -> u32 {
        let x = coord.x - Self::MIN_X;
        coord.y * Self::WIDTH + x
    }

    fn is_free(&self, coord: Coord) -> bool {
        let index = Self::to_index(coord);
        !self.0[index as usize]
    }

    fn block(&mut self, coord: Coord) {
        let index = Self::to_index(coord);
        self.0.set(index as usize, true);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn move_x(self, dx: i32) -> Self {
        Self {
            x: self.x.saturating_add_signed(dx),
            y: self.y,
        }
    }

    fn move_y(self, dy: i32) -> Self {
        Self {
            x: self.x,
            y: self.y.saturating_add_signed(dy),
        }
    }
}

impl Parse for Coord {
    type Out<'a> = Self;

    fn parse_from(input: &str) -> Self::Out<'_> {
        let (x, y) = input.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl PuzzleInput for Map {
    type Out<'a> = Self;

    fn from_input(input: &str) -> Self::Out<'_> {
        const SENTINEL: Coord = Coord { x: 0, y: 0 };

        let mut min_x = u32::MAX;
        let mut max_x = 0;
        let mut max_y = 0;

        let mut blocked = Cave::new();

        for line in aoc::lines(input) {
            let mut prev = SENTINEL;
            for coord in line.split(" -> ") {
                let coord = Coord::parse_from(coord);

                if coord.x < min_x {
                    min_x = coord.x;
                }
                if coord.x > max_x {
                    max_x = coord.x;
                }
                if coord.y > max_y {
                    max_y = coord.y;
                }

                if prev == SENTINEL {
                    prev = coord;
                    continue;
                }

                #[allow(clippy::comparison_chain)]
                if prev.x == coord.x {
                    if prev.y <= coord.y {
                        for y in prev.y..=coord.y {
                            blocked.block(Coord { x: coord.x, y });
                        }
                    } else {
                        for y in coord.y..=prev.y {
                            blocked.block(Coord { x: coord.x, y });
                        }
                    };
                } else if prev.x < coord.x {
                    for x in prev.x..=coord.x {
                        blocked.block(Coord { x, y: coord.y });
                    }
                } else {
                    for x in coord.x..=prev.x {
                        blocked.block(Coord { x, y: coord.y });
                    }
                }

                prev = coord;
            }
        }

        let x_bounds = MinMax {
            min: min_x,
            max: max_x,
        };
        let y_bounds = MinMax { min: 0, max: max_y };

        Self {
            blocked,
            x_bounds,
            y_bounds,
            drops: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 24);
        assert_eq!(res2, 93);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1330);
        assert_eq!(res2, 26139);
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
        b.iter(|| part1(&mut input.clone()));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let mut input = Solver::parse_input(Solver::puzzle_input());
        part1(&mut input);
        b.iter(|| part2(&mut input.clone()));
    }
}
