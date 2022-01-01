use aoc::ProcessInput;
use std::{collections::HashMap, iter::successors, iter::FromIterator, string::String};

type Input = String;
type Output = u64;

register!(
    "input/day20.txt";
    (input: input!(process Map)) -> Output {
        part1(&input);
        part2(input);
    }
);

fn part1(input: &Map) -> Output {
    let dim = input.dim;
    let puzzle = &input.puzzle;

    (puzzle[0][0].id as u64)
        * (puzzle[0][dim - 1].id as u64)
        * (puzzle[dim - 1][0].id as u64)
        * (puzzle[dim - 1][dim - 1].id as u64)
}

fn part2(input: Map) -> Output {
    let dim = input.dim;
    let image = build_image(input.puzzle, input.blocks);

    let world_size = image
        .iter()
        .flat_map(|line| line.iter())
        .filter(|b| **b == b'#')
        .count();
    let monster_size = SEA_MONSTER
        .iter()
        .flat_map(|line| line.bytes())
        .filter(|b| *b == b'#')
        .count();

    let monsters = count_monsters(dim, image);
    let pt2 = world_size - monsters * monster_size;

    pt2 as u64
}

fn solve_puzzle(dim: usize, edges: &Edges, tiles: &Tiles) -> Vec<Vec<Tile>> {
    let mut corners = HashMap::new();
    for edge in edges.values() {
        if let Edge::Corner(id) = edge {
            *corners.entry(*id).or_insert(0) += 1;
        }
    }

    corners
        .iter()
        .find_map(|(&corner, &count)| {
            if count == 4 {
                try_solve(dim, edges, tiles, corner)
            } else {
                None
            }
        })
        .unwrap()
}

fn try_solve(dim: usize, edges: &Edges, tiles: &Tiles, corner: i16) -> Option<Vec<Vec<Tile>>> {
    let mut corner_tile = Tile::of(corner, tiles[&corner]);

    while (
        corner_tile.neighbor(Dir::Up, edges),
        corner_tile.neighbor(Dir::Left, edges),
    ) != (None, None)
    {
        corner_tile.rotate();
    }

    try_solve_from_top_left(dim, edges, tiles, corner_tile)
}

fn try_solve_from_top_left(
    dim: usize,
    edges: &Edges,
    tiles: &Tiles,
    corner: Tile,
) -> Option<Vec<Vec<Tile>>> {
    let mut image = vec![vec![Tile::empty(-1); dim]; dim];
    image[0][0] = corner;
    for i in 1..dim {
        image[i][0] = image[i - 1][0].find_next_down(edges, tiles)?;
    }
    for row in &mut image {
        for j in 1..dim {
            row[j] = row[j - 1].find_next_right(edges, tiles)?;
        }
    }

    Some(image)
}

fn build_image(puzzle: Vec<Vec<Tile>>, mut blocks: HashMap<i16, Vec<String>>) -> Vec<Vec<u8>> {
    puzzle
        .into_iter()
        .flat_map(|row| {
            row.into_iter()
                .map(|cell| {
                    let block = blocks.remove(&cell.id).unwrap();
                    // let dim = block.len() - 1;

                    let lines = block.into_iter().skip(1).map(String::into_bytes);
                    let mut block = match cell.orientation {
                        Dir::Up => lines.collect(),
                        x => {
                            let mut block = rotate(10, lines);
                            for _ in 1..(x as u8) {
                                block = rotate(10, block.into_iter());
                            }
                            block
                        }
                    };
                    if matches!(cell.flip, Flip::Horizontal | Flip::Both) {
                        flip_h(&mut block);
                    }
                    if matches!(cell.flip, Flip::Vertical | Flip::Both) {
                        flip_v(&mut block);
                    }

                    block[1..9]
                        .iter()
                        .map(|line| line[1..9].to_vec())
                        .collect::<Vec<_>>()
                })
                .fold(None::<Vec<Vec<u8>>>, |current, block| match current {
                    Some(mut c) => {
                        for (r, b) in c.iter_mut().zip(block) {
                            r.extend_from_slice(&b);
                        }
                        Some(c)
                    }
                    None => Some(block),
                })
                .unwrap()
        })
        .collect()
}

fn rotate<I, T>(dim: usize, block: I) -> Vec<Vec<u8>>
where
    I: Iterator<Item = T>,
    T: AsRef<[u8]>,
{
    let mut rotated = vec![vec![0; dim]; dim];

    for (i, row) in block.enumerate() {
        for (j, &c) in row.as_ref().iter().enumerate() {
            rotated[j][dim - 1 - i] = c;
        }
    }

    rotated
}

fn flip_h(block: &mut Vec<Vec<u8>>) {
    for i in 0..5 {
        block.swap(i, 9 - i);
    }
}

fn flip_v(block: &mut Vec<Vec<u8>>) {
    for line in block.iter_mut() {
        line.reverse();
    }
}

fn flip(image: &[Vec<u8>]) -> Vec<Vec<u8>> {
    image
        .iter()
        .map(|line| line.iter().rev().copied().collect())
        .collect()
}

const SEA_MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

fn count_monsters(dim: usize, image: Vec<Vec<u8>>) -> usize {
    let monster = SEA_MONSTER
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.bytes()
                .enumerate()
                .filter(|(_, b)| *b == b'#')
                .map(move |(j, _)| (i as isize - 1, j as isize))
        })
        .collect::<Vec<_>>();

    let flipped = flip(&image);
    successors(Some(image), |img| Some(rotate(dim * 8, img.iter())))
        .take(4)
        .chain(successors(Some(flipped), |img| Some(rotate(dim * 8, img.iter()))).take(4))
        .map(|image| count_monsters_in(&image, &monster))
        .find(|count| *count > 0)
        .unwrap()
}

fn count_monsters_in(image: &[Vec<u8>], monster: &[(isize, isize)]) -> usize {
    image
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, b)| **b == b'#')
                .map(move |(j, _)| (i as isize, j as isize))
        })
        .filter(|&(i, j)| is_monster((i, j), image, monster))
        .count()
}

#[inline]
fn is_monster((i, j): (isize, isize), image: &[Vec<u8>], monster: &[(isize, isize)]) -> bool {
    projected_monster((i, j), monster).all(|(x, y)| is_set((x, y), image))
}

#[inline]
fn projected_monster(
    (i, j): (isize, isize),
    monster: &[(isize, isize)],
) -> impl Iterator<Item = (isize, isize)> + '_ {
    monster.iter().map(move |(mi, mj)| (i + mi, j + mj))
}

#[inline]
fn is_set((i, j): (isize, isize), image: &[Vec<u8>]) -> bool {
    #[inline]
    fn at((i, j): (isize, isize), image: &[Vec<u8>]) -> Option<u8> {
        let i = usize::try_from(i).ok()?;
        let j = usize::try_from(j).ok()?;
        let c = image.get(i)?.get(j)?;
        Some(*c)
    }

    at((i, j), image) == Some(b'#')
}

type Edges = HashMap<u16, Edge>;
type Tiles = HashMap<i16, TileEdges>;

#[derive(Debug, Copy, Clone)]
struct TileEdges(u16, u16, u16, u16);

#[derive(Debug, Copy, Clone)]
enum Edge {
    Corner(i16),
    Border(i16, i16),
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Flip {
    None,
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    id: i16,
    orientation: Dir,
    flip: Flip,
    edges: TileEdges,
}

impl Tile {
    fn empty(id: i16) -> Self {
        Self::of(id, TileEdges(0, 0, 0, 0))
    }

    fn of(id: i16, edges: TileEdges) -> Self {
        Self {
            id,
            edges,
            orientation: Dir::Up,
            flip: Flip::None,
        }
    }

    fn neighbor(&self, direction: Dir, edges: &Edges) -> Option<i16> {
        let edge = match direction {
            Dir::Up => self.edges.top(),
            Dir::Right => self.edges.right(),
            Dir::Down => self.edges.bottom(),
            Dir::Left => self.edges.left(),
        };
        match edges[&edge] {
            Edge::Corner(id) if id != self.id => Some(id),
            Edge::Border(id, _) if id != self.id => Some(id),
            Edge::Border(_, id) if id != self.id => Some(id),
            _ => None,
        }
    }

    fn rotate(&mut self) {
        self.edges = self.edges.rotated();
        self.orientation = match self.orientation {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        };
    }

    fn flip_horizontal(&mut self) {
        self.edges = self.edges.h_flipped();
        self.flip = match self.flip {
            Flip::None => Flip::Horizontal,
            Flip::Horizontal => Flip::None,
            Flip::Vertical => Flip::Both,
            Flip::Both => Flip::Vertical,
        };
    }

    fn flip_vertical(&mut self) {
        self.edges = self.edges.v_flipped();
        self.flip = match self.flip {
            Flip::None => Flip::Vertical,
            Flip::Horizontal => Flip::Both,
            Flip::Vertical => Flip::None,
            Flip::Both => Flip::Horizontal,
        };
    }

    fn find_next_down(&self, edges: &Edges, tiles: &Tiles) -> Option<Self> {
        let other = self.neighbor(Dir::Down, edges)?;
        let mut tile = Self::of(other, tiles[&other]);

        while tile.neighbor(Dir::Up, edges) != Some(self.id) {
            tile.rotate();
        }

        if self.edges.bottom() != tile.edges.top() {
            tile.flip_vertical();
        }

        Some(tile)
    }

    fn find_next_right(&self, edges: &Edges, tiles: &Tiles) -> Option<Self> {
        let other = self.neighbor(Dir::Right, edges)?;
        let mut tile = Self::of(other, tiles[&other]);

        while tile.neighbor(Dir::Left, edges) != Some(self.id) {
            tile.rotate();
        }

        if self.edges.right() != tile.edges.left() {
            tile.flip_horizontal();
        }

        Some(tile)
    }
}

impl TileEdges {
    fn edges(self) -> TileEdgesIter {
        TileEdgesIter {
            edges: self,
            edge: 0,
        }
    }

    fn top(self) -> u16 {
        self.0
    }

    fn right(self) -> u16 {
        self.1
    }

    fn bottom(self) -> u16 {
        self.2
    }

    fn left(self) -> u16 {
        self.3
    }

    fn rotated(self) -> Self {
        let TileEdges(top, right, bottom, left) = self;
        Self(Self::swap(left), top, Self::swap(right), bottom)
    }

    fn h_flipped(self) -> Self {
        let TileEdges(top, right, bottom, left) = self;
        Self(bottom, Self::swap(right), top, Self::swap(left))
    }

    fn v_flipped(self) -> Self {
        let TileEdges(top, right, bottom, left) = self;
        Self(Self::swap(top), left, Self::swap(bottom), right)
    }

    fn swap(edge: u16) -> u16 {
        edge.reverse_bits() >> 6
    }
}

struct TileEdgesIter {
    edges: TileEdges,
    edge: u8,
}

impl Iterator for TileEdgesIter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let edge = match self.edge {
            0 => self.edges.0,
            1 => self.edges.1,
            2 => self.edges.2,
            3 => self.edges.3,
            4 => TileEdges::swap(self.edges.0),
            5 => TileEdges::swap(self.edges.1),
            6 => TileEdges::swap(self.edges.2),
            7 => TileEdges::swap(self.edges.3),
            _ => return None,
        };
        self.edge += 1;
        Some(edge)
    }
}

impl<'a> FromIterator<&'a str> for Tile {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut lines = iter.into_iter();
        let id = lines.next().unwrap()[5..9].parse::<i16>().unwrap();
        let tile = lines.fold(0_u128, |tile, line| {
            line.bytes()
                .map(|c| match c {
                    b'#' => 1,
                    b'.' => 0,
                    x => unreachable!("not # or . : {}", x),
                })
                .fold(tile, |sum, digit| sum << 1 | digit)
        });

        let top = ((tile >> 90) as u16) & 1023;
        let bottom = (tile as u16) & 1023;

        let left = ((tile >> 90) & 512) as u16
            | ((tile >> 81) & 256) as u16
            | ((tile >> 72) & 128) as u16
            | ((tile >> 63) & 64) as u16
            | ((tile >> 54) & 32) as u16
            | ((tile >> 45) & 16) as u16
            | ((tile >> 36) & 8) as u16
            | ((tile >> 27) & 4) as u16
            | ((tile >> 18) & 2) as u16
            | ((tile >> 9) & 1) as u16;

        let right = ((tile >> 81) & 512) as u16
            | ((tile >> 72) & 256) as u16
            | ((tile >> 63) & 128) as u16
            | ((tile >> 54) & 64) as u16
            | ((tile >> 45) & 32) as u16
            | ((tile >> 36) & 16) as u16
            | ((tile >> 27) & 8) as u16
            | ((tile >> 18) & 4) as u16
            | ((tile >> 9) & 2) as u16
            | (tile & 1) as u16;

        Self::of(id, TileEdges(top, right, bottom, left))
    }
}

#[derive(Clone)]
pub struct Map {
    dim: usize,
    puzzle: Vec<Vec<Tile>>,
    blocks: HashMap<i16, Vec<String>>,
}

impl ProcessInput for Map {
    type In = input!(chunk Input);

    type Out = Self;

    fn process(input: <Self::In as aoc::PuzzleInput>::Out) -> Self::Out {
        let mut edges = Edges::new();
        let mut tiles = Tiles::new();
        let mut blocks = HashMap::new();

        for block in input {
            let tile = block.iter().map(String::as_str).collect::<Tile>();
            for edge in tile.edges.edges() {
                edges
                    .entry(edge)
                    .and_modify(|e| match e {
                        Edge::Corner(id) => {
                            *e = Edge::Border(*id, tile.id);
                        }
                        Edge::Border(..) => {
                            unreachable!("non distinct edge");
                        }
                    })
                    .or_insert_with(|| Edge::Corner(tile.id));
            }
            tiles.insert(tile.id, tile.edges);
            blocks.insert(tile.id, block);
        }

        let dim = (tiles.len() as f32).sqrt() as usize;
        let puzzle = solve_puzzle(dim, &edges, &tiles);

        Self {
            dim,
            puzzle,
            blocks,
        }
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
        assert_eq!(res1, 47_213_728_755_493);
        assert_eq!(res2, 1599);
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_ex() {
        assert_eq!(
            (20_899_048_083_289, 273),
            Solver::run_on(
                r#"
    Tile 2311:
    ..##.#..#.
    ##..#.....
    #...##..#.
    ####.#...#
    ##.##.###.
    ##...#.###
    .#.#.#..##
    ..#....#..
    ###...#.#.
    ..###..###

    Tile 1951:
    #.##...##.
    #.####...#
    .....#..##
    #...######
    .##.#....#
    .###.#####
    ###.##.##.
    .###....#.
    ..#.#..#.#
    #...##.#..

    Tile 1171:
    ####...##.
    #..##.#..#
    ##.#..#.#.
    .###.####.
    ..###.####
    .##....##.
    .#...####.
    #.##.####.
    ####..#...
    .....##...

    Tile 1427:
    ###.##.#..
    .#..#.##..
    .#.##.#..#
    #.#.#.##.#
    ....#...##
    ...##..##.
    ...#.#####
    .#.####.#.
    ..#..###.#
    ..##.#..#.

    Tile 1489:
    ##.#.#....
    ..##...#..
    .##..##...
    ..#...#...
    #####...#.
    #..#.#.#.#
    ...#.#.#..
    ##.#...##.
    ..##.##.##
    ###.##.#..

    Tile 2473:
    #....####.
    #..#.##...
    #.##..#...
    ######.#.#
    .#...#.#.#
    .#########
    .###.#..#.
    ########.#
    ##...##.#.
    ..###.#.#.

    Tile 2971:
    ..#.#....#
    #...###...
    #.#.###...
    ##.##..#..
    .#####..##
    .#..####.#
    #..#.#..#.
    ..####.###
    ..#.#.###.
    ...#.#.#.#

    Tile 2729:
    ...#.#.#.#
    ####.#....
    ..#.#.....
    ....#..#.#
    .##..##.#.
    .#.####...
    ####.#.#..
    ##.####...
    ##..#.##..
    #.##...##.

    Tile 3079:
    #.#.#####.
    .#..######
    ..#.......
    ######....
    ####.#..#.
    .#...#.##.
    #.#####.##
    ..#.###...
    ..#.......
    ..#.###...
                        "#,
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
        b.iter(|| part1(&input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(input.clone()));
    }
}
