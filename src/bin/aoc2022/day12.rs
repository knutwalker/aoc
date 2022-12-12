use std::collections::VecDeque;

use aoc::{ProcessInput, PuzzleInput};

type Output = u32;

register!(
    "input/day12.txt";
    (input: input!(process Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &Input) -> Output {
    items.depth[items.start as usize]
}

fn part2(items: &Input) -> Output {
    std::iter::zip(&items.tiles, &items.depth)
        .filter_map(|(&t, &d)| (t == 0).then_some(d))
        .min()
        .unwrap()
}

#[derive(Debug)]
pub struct Map {
    tiles: Vec<u8>,
    stride: u32,
    start: u32,
    target: u32,
}

impl PuzzleInput for Map {
    type Out<'a> = Self;

    fn from_input(input: &str) -> Self::Out<'_> {
        let mut stride = u32::MAX;
        let mut start = u32::MAX;
        let mut target = u32::MAX;

        let tiles = aoc::lines(input)
            .flat_map(|line| {
                stride = line.len() as u32;
                line.bytes()
            })
            .enumerate()
            .map(|(i, tile)| match tile {
                b'S' => {
                    start = i as u32;
                    0
                }
                b'E' => {
                    target = i as u32;
                    25
                }
                otherwise => otherwise - b'a',
            })
            .collect();

        Self {
            tiles,
            stride,
            start,
            target,
        }
    }
}

pub struct Input {
    tiles: Vec<u8>,
    depth: Vec<u32>,
    start: u32,
}

impl ProcessInput for Input {
    type In = input!(verbatim Map);

    type Out<'a> = Self;

    fn process(input: <Self::In as PuzzleInput>::Out<'_>) -> Self::Out<'_> {
        fn for_each_in_neighbor(input: &Map, pos: u32, mut action: impl FnMut(u32)) {
            let x_pos = pos % input.stride;
            let elevation = input.tiles[pos as usize];
            let mut call = |pos| {
                if elevation <= input.tiles[pos as usize] + 1 {
                    action(pos);
                }
            };

            // left
            if x_pos > 0 {
                call(pos - 1);
            }

            // up
            if let Some(up) = pos.checked_sub(input.stride) {
                call(up);
            }

            // right
            if x_pos < input.stride - 1 {
                call(pos + 1);
            }

            // down
            if pos < input.tiles.len() as u32 - input.stride {
                call(pos + input.stride);
            }
        }

        #[derive(Clone, Debug)]
        struct Item {
            item: u32,
            dist: u32,
        }

        let mut visited = vec![false; input.tiles.len()];
        let mut depth = vec![u32::MAX; input.tiles.len()];

        let mut queue = VecDeque::new();

        visited[input.target as usize] = true;
        queue.push_back(Item {
            item: input.target,
            dist: 0,
        });

        while let Some(next) = queue.pop_front() {
            depth[next.item as usize] = next.dist;

            for_each_in_neighbor(&input, next.item, |neighbor| {
                if !visited[neighbor as usize] {
                    visited[neighbor as usize] = true;
                    queue.push_back(Item {
                        item: neighbor,
                        dist: next.dist + 1,
                    });
                }
            });
        }

        Self {
            tiles: input.tiles,
            depth,
            start: input.start,
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
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 31);
        assert_eq!(res2, 29);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 456);
        assert_eq!(res2, 454);
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
