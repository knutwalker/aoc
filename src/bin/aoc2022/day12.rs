use std::{collections::VecDeque, ops::ControlFlow};

use aoc::PuzzleInput;

type Output = u32;

register!(
    "input/day12.txt";
    (input: input!(verbatim Map)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(map: &Map) -> Output {
    let start = map.start;
    map.bfs(|item| {
        if item.item == start {
            ControlFlow::Break(item.dist)
        } else {
            ControlFlow::Continue(())
        }
    })
    .unwrap()
}

fn part2(map: &Map) -> Output {
    let mut smallest = u32::MAX;

    map.bfs(|item| {
        if item.dist < smallest && map.tiles[item.item as usize] == 0 {
            smallest = item.dist;
        }
        ControlFlow::<()>::Continue(())
    });

    smallest
}

#[derive(Debug)]
pub struct Map {
    tiles: Vec<u8>,
    stride: u32,
    start: u32,
    target: u32,
}

impl Map {
    fn bfs<T>(&self, mut action: impl FnMut(Item) -> ControlFlow<T>) -> Option<T> {
        let mut visited = vec![false; self.tiles.len()];

        let mut queue = VecDeque::new();

        visited[self.target as usize] = true;
        queue.push_back(Item {
            item: self.target,
            dist: 0,
        });

        while let Some(next) = queue.pop_front() {
            match action(next) {
                ControlFlow::Break(res) => return Some(res),
                ControlFlow::Continue(_) => {}
            }

            self.for_each_in_neighbor(next.item, |neighbor| {
                if !visited[neighbor as usize] {
                    visited[neighbor as usize] = true;
                    queue.push_back(Item {
                        item: neighbor,
                        dist: next.dist + 1,
                    });
                }
            });
        }

        None
    }
    fn for_each_in_neighbor(&self, pos: u32, mut action: impl FnMut(u32)) {
        let x_pos = pos % self.stride;
        let elevation = self.tiles[pos as usize];
        let mut call = |pos| {
            if elevation <= self.tiles[pos as usize] + 1 {
                action(pos);
            }
        };

        // left
        if x_pos > 0 {
            call(pos - 1);
        }

        // up
        if let Some(up) = pos.checked_sub(self.stride) {
            call(up);
        }

        // right
        if x_pos < self.stride - 1 {
            call(pos + 1);
        }

        // down
        if pos < self.tiles.len() as u32 - self.stride {
            call(pos + self.stride);
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Item {
    item: u32,
    dist: u32,
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
