use aoc::{lines, PuzzleInput};
use derive_more::Add;
use disjoint_sets::UnionFind;
use fxhash::{FxBuildHasher, FxHashMap};
use std::{cmp::Reverse, ops::AddAssign};

pub type Wcc = FxHashMap<usize, Basin>;

register!(
    "input/day9.txt";
    (wcc: input!(verbatim WccInput)) -> u64 {
        part1(&wcc);
        part2(&wcc);
    }
);

fn part1(hm: &Wcc) -> u64 {
    hm.values()
        .map(|basin| u64::from(basin.low_point) + 1)
        .sum()
}

fn part2(hm: &Wcc) -> u64 {
    hm.values()
        .map(|basin| basin.size)
        .collect::<Vec<_>>()
        .select_nth_unstable_by_key(3, |&k| Reverse(k))
        .0
        .iter()
        .copied()
        .product()
}

#[derive(Clone, Copy, Debug)]
pub struct Basin {
    size: u64,
    low_point: u8,
}

impl AddAssign<u8> for Basin {
    fn add_assign(&mut self, rhs: u8) {
        self.size += 1;
        if rhs < self.low_point {
            self.low_point = rhs;
        }
    }
}

impl Default for Basin {
    fn default() -> Self {
        Self {
            size: 0,
            low_point: u8::MAX,
        }
    }
}

pub struct WccInput;

impl PuzzleInput for WccInput {
    type Out = Wcc;

    fn from_input(input: &str) -> Self::Out {
        let input = lines(input).map(str::as_bytes).collect::<Vec<_>>();
        let (h, w) = (input.len(), input[0].len());
        let size = w * h;

        let mut dss = UnionFind::new(size + 1);
        for (row, current_row) in input.iter().copied().enumerate() {
            for (col, h) in current_row.iter().map(|b| *b - b'0').enumerate() {
                let idx = w * row + col;
                if h == 9 {
                    // all 9ers are in one community outside of the id range
                    dss.union(idx, size);
                    continue;
                }

                if let Some(pr) = row.checked_sub(1) {
                    if input[pr][col] - b'0' != 9 {
                        dss.union(idx, w * pr + col);
                    }
                }
                if let Some(pc) = col.checked_sub(1) {
                    if current_row[pc] - b'0' != 9 {
                        dss.union(idx, w * row + pc);
                    }
                }
            }
        }

        let mut basins = FxHashMap::with_capacity_and_hasher(64, FxBuildHasher::default());

        for idx in 0..size {
            let root = dss.find(idx);
            if root == size {
                // ignore the community of all the 9ers
                continue;
            }

            let row = idx / w;
            let col = idx % w;
            let h = input[row][col] - b'0';

            *basins.entry(root).or_default() += h;
        }

        basins
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Add)]
struct Pos(i16, i16);

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test_ex() {
        let input = r#"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 15);
        assert_eq!(res2, 1134);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 564);
        assert_eq!(res2, 1_038_240);
    }
}
