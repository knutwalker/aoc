use aoc::{MinMax, Parse, ProcessInput};
use fxhash::FxHashSet;

type Output = usize;

register!(
    "input/day14.txt";
    (input: input!(process Map)) -> Output {
        part1(&mut input);
        part2(&mut input);
    }
);

fn part1(map: &mut Map) -> Output {
    let min_max_x = map.x_bounds;
    let min_max_y = map.y_bounds;

    let blocked = &mut map.blocked;

    let mut drops = 0;

    loop {
        let mut sand = Coord { x: 500, y: 0 };

        let resting = loop {
            if sand.x < min_max_x.min || sand.x > min_max_x.max || sand.y > min_max_y.max {
                map.drops = drops;
                return drops;
            }

            // try down
            let down = sand.move_y(1);
            if !blocked.contains(&down) {
                sand = down;
                continue;
            }

            // try down-left
            let down_left = down.move_x(-1);
            if !blocked.contains(&down_left) {
                sand = down_left;
                continue;
            }

            // try down-right
            let down_right = down.move_x(1);
            if !blocked.contains(&down_right) {
                sand = down_right;
                continue;
            }

            break sand;
        };

        drops += 1;
        blocked.insert(resting);
    }
}

fn part2(map: &mut Map) -> Output {
    let bottom = map.y_bounds.max + 1;
    let blocked = &mut map.blocked;

    let mut drops = map.drops;
    let spawn = Coord { x: 500, y: 0 };

    loop {
        let mut sand = spawn;

        let resting = loop {
            if sand.y == bottom {
                break sand;
            }

            // try down
            let down = sand.move_y(1);
            if !blocked.contains(&down) {
                sand = down;
                continue;
            }

            // try down-left
            let down_left = down.move_x(-1);
            if !blocked.contains(&down_left) {
                sand = down_left;
                continue;
            }

            // try down-right
            let down_right = down.move_x(1);
            if !blocked.contains(&down_right) {
                sand = down_right;
                continue;
            }

            break sand;
        };

        drops += 1;

        if resting == spawn {
            return drops;
        }

        blocked.insert(resting);
    }
}

#[derive(Clone)]
pub struct Map {
    blocked: FxHashSet<Coord>,
    x_bounds: MinMax<u32>,
    y_bounds: MinMax<u32>,
    drops: usize,
}

impl ProcessInput for Map {
    type In = input!(Input);

    type Out<'a> = Self;

    fn process(input: <Self::In as aoc::PuzzleInput>::Out<'_>) -> Self::Out<'_> {
        let blocked = input
            .iter()
            .flat_map(|i| {
                i.path
                    .iter()
                    .scan((0, 0), |prev, end| {
                        Some({
                            if matches!(*prev, (0, 0)) {
                                *prev = (end.x, end.y);
                            }

                            let (start_x, start_y) = *prev;
                            *prev = (end.x, end.y);

                            if start_x == end.x {
                                let dy = (end.y as i32).saturating_sub(start_y as i32).signum();
                                Line::x_line(
                                    Coord {
                                        x: start_x,
                                        y: start_y,
                                    },
                                    end.y,
                                    dy,
                                )
                            } else {
                                let dx = (end.x as i32).saturating_sub(start_x as i32).signum();
                                Line::y_line(
                                    Coord {
                                        x: start_x,
                                        y: start_y,
                                    },
                                    end.x,
                                    dx,
                                )
                            }
                        })
                    })
                    .flatten()
            })
            .collect();

        let x_bounds = input
            .iter()
            .flat_map(|i| i.path.iter().map(|p| p.x))
            .collect();
        let y_bounds = input
            .iter()
            .flat_map(|i| i.path.iter().map(|p| p.y))
            .collect();

        Self {
            blocked,
            x_bounds,
            y_bounds,
            drops: 0,
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Option<Coord>,
    end: Coord,
    dx: i32,
    dy: i32,
}

impl Line {
    fn x_line(start: Coord, end_y: u32, dy: i32) -> Self {
        Self {
            start: Some(start),
            end: Coord {
                x: start.x,
                y: end_y,
            },
            dx: 0,
            dy,
        }
    }

    fn y_line(start: Coord, end_x: u32, dx: i32) -> Self {
        Self {
            start: Some(start),
            end: Coord {
                x: end_x,
                y: start.y,
            },
            dx,
            dy: 0,
        }
    }
}

impl Iterator for Line {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let mut start = self.start.as_mut()?;
        if *start == self.end {
            self.start = None;
            return Some(self.end);
        }

        start.x = start.x.saturating_add_signed(self.dx);
        start.y = start.y.saturating_add_signed(self.dy);

        self.start
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    path: Vec<Coord>,
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

impl Parse for Input {
    type Out<'a> = Self;

    fn parse_from(input: &str) -> Self::Out<'_> {
        Self {
            path: input.split(" -> ").map(Coord::parse_from).collect(),
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
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&mut input.clone()));
    }
}
