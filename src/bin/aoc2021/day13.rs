use fxhash::{FxBuildHasher, FxHashSet};
use std::{num::ParseIntError, str::FromStr};

type Input = In;
type Output = aoc::Output<usize, String>;

register!(
    "input/day13.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    Output::Part1(fold(items, 1).len())
}

fn part2(items: &[Input]) -> Output {
    let dots = fold(items, usize::MAX);

    let mut x = u32::MAX;
    let mut y = u32::MAX;
    let mut folds = items.iter().rev();
    while let Some(&In::Fold { axis, line }) = folds.next() {
        if axis == 0 && x == u32::MAX {
            x = line;
        } else if axis == 1 && y == u32::MAX {
            y = line;
        }
        if x != u32::MAX && y != u32::MAX {
            break;
        }
    }

    let line = ".".repeat(x as usize);
    let mut grid = vec![line; y as usize];

    for [x, y] in dots {
        let (x, y) = (x as usize, y as usize);
        grid[y].replace_range(x..=x, "#");
    }

    Output::Part2(grid.join("\n"))
}

fn fold(items: &[Input], mut amount: usize) -> FxHashSet<[u32; 2]> {
    let mut dots = FxHashSet::with_capacity_and_hasher(items.len(), FxBuildHasher::default());
    let mut folded = Vec::with_capacity(items.len());

    for &item in items {
        match item {
            In::Dot(dot) => {
                dots.insert(dot);
            }
            In::Fold { axis, line } if amount > 0 => {
                amount -= 1;
                dots.retain(|dot| {
                    if dot[axis] > line {
                        let mut dot = *dot;
                        dot[axis] = line * 2 - dot[axis];
                        folded.push(dot);
                        false
                    } else {
                        true
                    }
                });

                dots.extend(folded.drain(..));
            }
            In::Fold { .. } => break,
        }
    }

    dots
}

#[derive(Copy, Clone, Debug)]
pub enum In {
    Dot([u32; 2]),
    Fold { axis: usize, line: u32 },
}

impl FromStr for In {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some((x, y)) = s.split_once(',') {
            Self::Dot([x.parse()?, y.parse()?])
        } else {
            let (axis, line) = s.trim_start_matches("fold along ").split_once('=').unwrap();
            Self::Fold {
                axis: usize::from(axis == "y"),
                line: line.parse()?,
            }
        })
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
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0
        
        fold along y=7
        fold along x=5
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, Output::Part1(17));
        assert_eq!(
            res2,
            Output::Part2(String::from(
                r#"
#####
#...#
#...#
#...#
#####
.....
.....
                "#
                .trim()
            ))
        );
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, Output::Part1(638));
        assert_eq!(
            res2,
            Output::Part2(String::from(
                r#"
.##....##..##..#..#.###...##..###..###..
#..#....#.#..#.#.#..#..#.#..#.#..#.#..#.
#.......#.#....##...###..#..#.#..#.###..
#.......#.#....#.#..#..#.####.###..#..#.
#..#.#..#.#..#.#.#..#..#.#..#.#....#..#.
.##...##...##..#..#.###..#..#.#....###..
                "#
                .trim()
            ))
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
        b.iter(|| part1(&input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&input));
    }
}
