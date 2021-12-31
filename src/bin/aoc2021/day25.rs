use derive_more::{Deref, DerefMut, Index, IndexMut};
use std::{convert::Infallible, fmt::Display, str::FromStr};

type Input = Line;
type Output = usize;

register!(
    "input/day25.txt";
    (input: input!(parse Input)) -> Output {
        part1(input.clone());
        part2(&input);
    }
);

fn part1(mut grid: Vec<Input>) -> Output {
    poop!("Initial state:");
    poop!("{}", Grid(&grid));

    (1..usize::MAX)
        .find(|round| {
            let result = !step1(&mut grid);

            poop!(
                "After {} step{}:",
                round,
                if *round == 1 { "" } else { "s" }
            );
            poop!("{}", Grid(&grid));

            result
        })
        .unwrap()
}

fn part2(items: &[Input]) -> Output {
    0
}

fn step1(grid: &mut Vec<Line>) -> bool {
    step1_right(grid) | step1_down(grid)
}

fn step1_right(grid: &mut Vec<Line>) -> bool {
    let swaps = (0..grid.len())
        .flat_map(|r| {
            let line = &grid[r];
            (0..line.len()).filter_map(move |c| {
                if matches!(line[c], Cell::Right) {
                    let next = (c + 1) % line.len();
                    if matches!(line[next], Cell::Empty) {
                        return Some((r, (c, next)));
                    }
                }
                None
            })
        })
        .collect::<Vec<_>>();

    let moved = !swaps.is_empty();

    for (r, (from, to)) in swaps {
        grid[r].swap(from, to);
    }

    moved
}

fn step1_down(grid: &mut Vec<Line>) -> bool {
    let ro_grid = &*grid;
    let swaps = (0..ro_grid[0].len())
        .flat_map(|c| {
            (0..ro_grid.len()).filter_map(move |r| {
                if matches!(ro_grid[r][c], Cell::Down) {
                    let next = (r + 1) % ro_grid.len();
                    if matches!(ro_grid[next][c], Cell::Empty) {
                        return Some(((r, next), c));
                    }
                }
                None
            })
        })
        .collect::<Vec<_>>();

    let moved = !swaps.is_empty();

    for ((from, to), c) in swaps {
        grid[from][c] = Cell::Empty;
        grid[to][c] = Cell::Down;
    }

    moved
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cell {
    Empty = b'.',
    Right = b'>',
    Down = b'v',
}

impl From<u8> for Cell {
    fn from(v: u8) -> Self {
        match v {
            b'>' => Self::Right,
            b'v' => Self::Down,
            _ => Self::Empty,
        }
    }
}

#[derive(Clone, Debug, Index, IndexMut, Deref, DerefMut)]
pub struct Line(Vec<Cell>);

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.bytes().map(Cell::from).collect()))
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self
            .0
            .iter()
            .map(|cell| char::from(*cell as u8))
            .collect::<String>();
        line.fmt(f)
    }
}

struct Grid<'a>(&'a [Line]);

impl Display for Grid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0 {
            line.fmt(f)?;
            f.write_str("\n")?;
        }
        Ok(())
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
        v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 58);
        assert_eq!(res2, 0);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 471);
        assert_eq!(res2, 0);
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
        b.iter(|| part1(input.clone()));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&input));
    }
}
