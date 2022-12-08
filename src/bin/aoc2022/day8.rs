use aoc::PuzzleInput;
use bitvec::prelude::Lsb0;
use bitvec::view::BitView;

type Output = u32;

register!(
    "input/day8.txt";
    (input: input!(verbatim Parser)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(input: &Input) -> Output {
    input.vis.iter().copied().map(u64::count_ones).sum()
}

fn part2(input: &Input) -> Output {
    let vis = input.vis.view_bits::<Lsb0>();
    let size = input.size;
    let x_size = size + 1;

    let mut top = 0;
    for pos in vis.iter_ones() {
        let tree = input.x_grid[pos];
        let x = pos / x_size;
        let y = pos % x_size;

        if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
            continue;
        }

        let mut total_score = 1;

        // look left
        let mut score = 0;
        for t in input.x_grid[x * x_size..pos].iter().rev() {
            score += 1;
            if tree <= *t {
                break;
            }
        }
        total_score *= score;

        // look right
        let mut score = 0;
        for t in &input.x_grid[pos + 1..(x + 1) * x_size - 1] {
            score += 1;
            if tree <= *t {
                break;
            }
        }
        total_score *= score;

        let pos = y * x_size + x;

        // look up
        let mut score = 0;
        for t in input.y_grid[y * x_size..pos].iter().rev() {
            score += 1;
            if tree <= *t {
                break;
            }
        }
        total_score *= score;

        // look down
        let mut score = 0;
        for t in &input.y_grid[pos + 1..(y + 1) * x_size - 1] {
            score += 1;
            if tree <= *t {
                break;
            }
        }
        total_score *= score;

        // scenic score
        if total_score > top {
            top = total_score;
        }
    }

    top
}

pub struct Input<'a> {
    x_grid: &'a [u8],
    y_grid: Vec<u8>,
    size: usize,
    vis: Vec<u64>,
}

pub enum Parser {}

impl PuzzleInput for Parser {
    type Out<'a> = Input<'a>;

    fn from_input(input: &str) -> Self::Out<'_> {
        let x_grid = input.trim().as_bytes();
        let size = (((1 + 4 * x_grid.len()) as f64).sqrt() / 2.0) as usize;
        // what be pay for transposing we make up for in pt2 performance
        let y_grid = transpose(size, x_grid);
        let vis = visible_map(size, x_grid, &y_grid);

        Input {
            x_grid,
            y_grid,
            size,
            vis,
        }
    }
}

fn transpose(size: usize, x_grid: &[u8]) -> Vec<u8> {
    let x_size = size + 1; // because every row ends in '\n'

    let mut y_grid = Vec::with_capacity(x_grid.len());
    for y in 0..size {
        for x in 0..size {
            y_grid.push(x_grid[x * x_size + y]);
        }
        y_grid.push(b'\n');
    }

    y_grid
}

fn visible_map(size: usize, x_grid: &[u8], y_grid: &[u8]) -> Vec<u64> {
    let x_size = size + 1; // because every row ends in '\n'
    let bit_size = f64::ceil((size * x_size) as f64 / 64.0) as usize;

    let mut vis = vec![0_u64; bit_size];
    let visible = vis.view_bits_mut::<Lsb0>();

    for x in 0..size {
        let x_start = x * x_size;
        let row = &x_grid[x_start..x_start + size];

        let mut top = 0;
        for (y, tree) in row.iter().enumerate() {
            if *tree > top {
                visible.set(x_start + y, true);
                top = *tree;
                if top == b'9' {
                    break;
                }
            }
        }

        let mut top = 0;
        for (y, tree) in row.iter().enumerate().rev() {
            if *tree > top {
                visible.set(x_start + y, true);
                top = *tree;
                if top == b'9' {
                    break;
                }
            }
        }
    }

    for y in 0..size {
        let y_start = y * x_size;
        let col = &y_grid[y_start..y_start + size];

        let mut top = 0;
        for (x, tree) in col.iter().enumerate() {
            if *tree > top {
                visible.set(x * x_size + y, true);
                top = *tree;
                if top == b'9' {
                    break;
                }
            }
        }

        let mut top = 0;
        for (x, tree) in col.iter().enumerate().rev() {
            if *tree > top {
                visible.set(x * x_size + y, true);
                top = *tree;
                if top == b'9' {
                    break;
                }
            }
        }
    }

    vis
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"
30373
25512
65332
33549
35390
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 21);
        assert_eq!(res2, 8);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1688);
        assert_eq!(res2, 410_400);
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
