use aoc::{lines, PuzzleInput};
use bitvec::prelude::{bitarr, BitArray, BitSlice, Msb0};
use std::fmt::{Display, Write};

type Output = usize;

register!(
    "input/day20.txt";
    (input: Input) -> Output {
        part1(&mut input);
        part2(&mut input);
    }
);

fn part1(input: &mut Input) -> Output {
    input.run(2);
    input.image.image.count_ones()
}

fn part2(input: &mut Input) -> Output {
    input.run(48);
    input.image.image.count_ones()
}

#[cfg(test)]
fn part1_standalone(mut input: Input) -> Output {
    input.run(2);
    input.image.image.count_ones()
}

#[cfg(test)]
fn part2_standalone(mut input: Input) -> Output {
    input.run(50);
    input.image.image.count_ones()
}

#[derive(Debug, Clone)]
pub struct Input {
    algorithm: BitArray<[u64; 8], Msb0>,
    size: usize,
    image: Image,
}

/// input image width
const N: usize = 100;
/// part 2 rounds
const ROUNDS: usize = 50;
/// 1-D size increase per round
const INCREASE: usize = 2;
/// required offset on either side of image to contain all pixel
const OFFSET: usize = ROUNDS + INCREASE;
/// required padding on one dimension to contain all pixel
const PADDING: usize = 2 * OFFSET;
/// maximum length of one dimension
const MAX_LEN: usize = N + PADDING;
/// maximum image size
const IMG_SIZE: usize = MAX_LEN * MAX_LEN;

/// image representation as bit vector on stack
type Bits = BitArray<[u64; (IMG_SIZE + 63) / 64], Msb0>;

#[derive(Debug, Clone)]
struct Image {
    start: usize,
    size: usize,
    image: Bits,
    next: Bits,
}

impl Input {
    fn run(&mut self, iterations: usize) {
        let algorithm = self.algorithm.as_bitslice();
        for _ in 0..iterations {
            self.image.iterate(algorithm, self.size);
        }
    }
}

impl Image {
    fn iterate(&mut self, algorithm: &BitSlice<u64, Msb0>, sz: usize) {
        let size = self.size + 2;

        let next_start = self.start - sz - 1;
        let mut start = next_start;

        for _row in 0..size {
            let s = start - sz - 1;
            for start in s..s + size {
                let code1 = usize::from(self.image[start]) << 8
                    | usize::from(self.image[start + 1]) << 7
                    | usize::from(self.image[start + 2]) << 6
                    | usize::from(self.image[start + sz]) << 5
                    | usize::from(self.image[start + sz + 1]) << 4
                    | usize::from(self.image[start + sz + 2]) << 3
                    | usize::from(self.image[start + sz + sz]) << 2
                    | usize::from(self.image[start + sz + sz + 1]) << 1
                    | usize::from(self.image[start + sz + sz + 2]);

                self.next.set(start + sz + 1, algorithm[code1]);
            }
            start += sz;
        }

        self.start = next_start;
        self.size = size;
        std::mem::swap(&mut self.image, &mut self.next);
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = (self.image.len() as f64).sqrt() as usize;
        for start in (0..self.image.len()).step_by(size) {
            for start in start..start + size {
                if self.image[start] {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl PuzzleInput for Input {
    type Out<'a> = Self;

    fn from_input(input: &str) -> Self::Out<'_> {
        let mut input = lines(input);
        let algorithm_input = input.next().expect("algorithm");

        let mut algorithm = bitarr![u64, Msb0; 0; 512];
        for (a, mut b) in algorithm_input.bytes().zip(algorithm.iter_mut()) {
            *b = matches!(a, b'#');
        }

        let infinity = algorithm[0];
        if infinity {
            assert!(
                !algorithm[511],
                "expecting the algorithm to toggle infinity"
            );
        }

        let mut image = bitarr![u64, Msb0; 0; IMG_SIZE];
        let next = if infinity {
            bitarr![u64, Msb0; 1; IMG_SIZE]
        } else {
            bitarr![u64, Msb0; 0; IMG_SIZE]
        };

        let first_line = input.next().unwrap();
        let size = first_line.len();
        let final_size = size + PADDING;

        let start = OFFSET * final_size + OFFSET;
        let mut offset = start;
        for line in std::iter::once(first_line).chain(input) {
            for (a, mut b) in line.bytes().zip(image[offset..offset + size].iter_mut()) {
                *b = matches!(a, b'#');
            }
            offset += final_size;
        }

        Self {
            algorithm,
            size: final_size,
            image: Image {
                start,
                size,
                image,
                next,
            },
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
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
        
        #..#.
        #....
        ##..#
        ..#..
        ..###
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 35);
        assert_eq!(res2, 3351);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 5846);
        assert_eq!(res2, 21149);
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
        b.iter(|| part1_standalone(input.clone()));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2_standalone(input.clone()));
    }
}
