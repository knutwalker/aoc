use aoc::{lines, PuzzleInput};
use fxhash::{FxBuildHasher, FxHashSet};
use std::rc::Rc;

type Output = usize;

register!(
    "input/day20.txt";
    (input: Input) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(input: &Input) -> Output {
    input.run(2).image.len()
}

fn part2(input: &Input) -> Output {
    input.run(50).image.len()
}

#[derive(Debug, Clone)]
pub struct Input {
    algorithm: Rc<[bool]>,
    size: (i32, i32),
    infinity: bool,
    image: FxHashSet<(i32, i32)>,
}

impl Input {
    fn run(&self, iterations: usize) -> Self {
        let mut next = self.iterate();
        for _ in 1..iterations {
            next = next.iterate();
        }
        next
    }

    fn iterate(&self) -> Self {
        const ENHANCE: i32 = 1;
        const D: [(i32, i32); 9] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let min = self.size.0 - ENHANCE;
        let max = self.size.1 + ENHANCE;

        let mut image =
            FxHashSet::with_capacity_and_hasher(self.image.capacity(), FxBuildHasher::default());

        for x in min..=max {
            for y in min..=max {
                let code = D
                    .into_iter()
                    .map(|(dx, dy)| (x + dx, y + dy))
                    .map(|(x, y)| {
                        if x > min && x < max && y > min && y < max {
                            self.image.contains(&(x, y))
                        } else {
                            self.infinity
                        }
                    })
                    .fold(0, |code, px| (code << 1) | usize::from(px));
                if self.algorithm[code] {
                    image.insert((x, y));
                }
            }
        }

        let infinity = self.algorithm[511 * usize::from(self.infinity)];
        Self {
            algorithm: Rc::clone(&self.algorithm),
            size: (min, max),
            infinity,
            image,
        }
    }
}

impl PuzzleInput for Input {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut input = lines(input);
        let algorithm = input.next().expect("algorithm");
        let algorithm = algorithm.bytes().map(|b| matches!(b, b'#')).collect();

        let mut size = 0;

        let image = input
            .enumerate()
            .flat_map(|(y, line)| {
                let y = y as i32;
                size = y;
                line.bytes()
                    .enumerate()
                    .filter_map(move |(x, b)| matches!(b, b'#').then(|| (x as i32, y)))
            })
            .collect();

        Self {
            algorithm,
            size: (0, size),
            infinity: false,
            image,
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
        b.iter(|| part1(&input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&input));
    }
}
