use aoc::{lines, PuzzleInput};

register!(
    "input/day11.txt";
    (input: input!(verbatim Octo)) -> usize {
        part1(&mut input);
        part2(&mut input);
    }
);

fn part1(octo: &mut Octo) -> usize {
    (0..100).map(|_| octo.flash()).sum()
}

fn part2(octo: &mut Octo) -> usize {
    (101..usize::MAX)
        .find(|_| octo.flash() == SIZE * SIZE)
        .unwrap()
}

#[cfg(test)]
fn part1_standalone(mut octo: Octo) -> usize {
    (0..100).map(|_| octo.flash()).sum()
}

#[cfg(test)]
fn part2_standalone(mut octo: Octo) -> usize {
    (1..usize::MAX)
        .find(|_| octo.flash() == SIZE * SIZE)
        .unwrap()
}

const SIZE: usize = 10;

#[derive(Clone, Copy, Debug)]
pub struct Octo([[u8; SIZE]; SIZE]);

impl Octo {
    fn flash(&mut self) -> usize {
        fn iterate(map: &mut [[u8; SIZE]; SIZE], flashed: &mut usize, row: usize, col: usize) {
            map[row][col] = 0;
            *flashed += 1;

            for (r, c) in [
                (row.wrapping_sub(1), col.wrapping_sub(1)),
                (row.wrapping_sub(1), col),
                (row.wrapping_sub(1), col + 1),
                (row, col.wrapping_sub(1)),
                (row, col + 1),
                (row + 1, col.wrapping_sub(1)),
                (row + 1, col),
                (row + 1, col + 1),
            ] {
                if let Some(x) = map.get_mut(r).and_then(|r| r.get_mut(c)) {
                    match *x {
                        0 => {}
                        1..=8 => *x += 1,
                        _ => iterate(map, flashed, r, c),
                    };
                }
            }
        }

        self.0.iter_mut().flatten().for_each(|x| *x += 1);

        let mut flashed = 0;
        for row in 0..SIZE {
            for col in 0..SIZE {
                if (0..=9).contains(&self.0[row][col]) {
                    continue;
                }
                iterate(&mut self.0, &mut flashed, row, col);
            }
        }

        flashed
    }
}

impl PuzzleInput for Octo {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let octo = lines(input)
            .flat_map(|s| s.bytes().map(|b| b - b'0').collect::<Vec<_>>().try_into())
            .collect::<Vec<_>>();
        Self(octo.try_into().unwrap())
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
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 1656);
        assert_eq!(res2, 195);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1661);
        assert_eq!(res2, 334);
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
        b.iter(|| part1_standalone(input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2_standalone(input));
    }
}
