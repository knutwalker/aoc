use aoc::{ProcessInput, PuzzleInput};
use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

type Output = usize;

register!(
    "input/day24.txt";
    (grid: input!(process Grids)) -> Output {
        grid.len();
        flipped(&grid).nth(100).unwrap();
    }
);

type Tile = (i32, i32);
type Grid = HashSet<Tile>;

#[derive(Debug, Copy, Clone)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

fn flipped(grid: &Grid) -> impl Iterator<Item = Output> {
    successors(Some(grid.clone()), |g| Some(cycle(g))).map(|g| g.len())
}

fn cycle(grid: &Grid) -> Grid {
    count_neighbors(grid)
        .into_iter()
        .filter_map(|(tile, flipped)| match (grid.contains(&tile), flipped) {
            (true, 1..=2) | (false, 2) => Some(tile),
            _ => None,
        })
        .collect()
}

fn count_neighbors(grid: &Grid) -> HashMap<Tile, isize> {
    let cap = grid.len() * 36;
    let mut counts = HashMap::with_capacity(cap);
    for &tile in grid {
        for tile in neighbours(tile) {
            *counts.entry(tile).or_default() += 1;
        }
    }
    counts
}

fn neighbours(tile: Tile) -> impl Iterator<Item = Tile> {
    Neighbors {
        tile,
        dir: Some(Dir::E),
    }
}

struct Neighbors {
    tile: Tile,
    dir: Option<Dir>,
}

impl Iterator for Neighbors {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let dir = self.dir.as_mut()?;
        let (x, y) = self.tile;
        let pos = match dir {
            Dir::E => {
                *dir = Dir::SE;
                (x + 2, y)
            }
            Dir::SE => {
                *dir = Dir::SW;
                (x + 1, y - 1)
            }
            Dir::SW => {
                *dir = Dir::W;
                (x - 1, y - 1)
            }
            Dir::W => {
                *dir = Dir::NW;
                (x - 2, y)
            }
            Dir::NW => {
                *dir = Dir::NE;
                (x - 1, y + 1)
            }
            Dir::NE => {
                self.dir = None;
                (x + 1, y + 1)
            }
        };
        Some(pos)
    }
}

pub struct Grids;

impl ProcessInput for Grids {
    type In = input!([u8]);

    type Out<'a> = Grid;

    fn process(input: <Self::In as PuzzleInput>::Out<'_>) -> Self::Out<'_> {
        input
            .into_iter()
            .map(|line| {
                line.iter().fold((0, 0, 0), |(x, y, step), dir| match dir {
                    b'e' => (x + (2 >> step), y, 0),
                    b'w' => (x - (2 >> step), y, 0),
                    b'n' => (x, y + 1, 1),
                    b's' => (x, y - 1, 1),
                    x => unreachable!("invalid input: {}", x),
                })
            })
            .fold(Grid::new(), |mut grid, (x, y, _)| {
                let tile = (x, y);
                if !grid.remove(&tile) {
                    grid.insert(tile);
                }
                grid
            })
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
        assert_eq!(res1, 523);
        assert_eq!(res2, 4225);
    }

    #[test]
    fn test_ex() {
        assert_eq!(
            (10, 2208),
            Solver::run_on(
                "
            sesenwnenenewseeswwswswwnenewsewsw
            neeenesenwnwwswnenewnwwsewnenwseswesw
            seswneswswsenwwnwse
            nwnwneseeswswnenewneswwnewseswneseene
            swweswneswnenwsewnwneneseenw
            eesenwseswswnenwswnwnwsewwnwsene
            sewnenenenesenwsewnenwwwse
            wenwwweseeeweswwwnwwe
            wsweesenenewnwwnwsenewsenwwsesesenwne
            neeswseenwwswnwswswnw
            nenwswwsewswnenenewsenwsenwnesesenew
            enewnwewneswsewnwswenweswnenwsenwsw
            sweneswneswneneenwnewenewwneswswnese
            swwesenesewenwneswnwwneseswwne
            enesenwswwswneneswsenwnewswseenwsese
            wnwnesenesenenwwnenwsewesewsesesew
            nenewswnwewswnenesenwnesewesw
            eneswnwswnwsenenwnwnwwseeswneewsenese
            neswnwewnwnwseenwseesewsenwsweewe
            wseweeenwnesenwwwswnew
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
        b.iter(|| input.len());
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| flipped(&input).nth(100).unwrap());
    }
}
