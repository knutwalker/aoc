use aoc::PuzzleInput;

type Input = Reg;
type Output = aoc::Output<Reg, String>;

register!(
    "input/day10.txt";
    (input: input!(verbatim Parser)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    Output::Part1(
        items
            .iter()
            .enumerate()
            .filter_map(|(cycle, reg)| {
                if cycle % 40 == 19 {
                    Some((cycle as i32 + 1) * reg)
                } else {
                    None
                }
            })
            .sum(),
    )
}

fn part2(items: &[Input]) -> Output {
    let chars = ['.', '#'];
    Output::Part2(
        items
            .chunks_exact(40)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .enumerate()
                    .map(|(col, pos)| chars[usize::from(pos.abs_diff(col as i32) <= 1)])
                    .chain(Some('\n'))
            })
            .collect(),
    )
}

pub enum Parser {}

type Reg = i32;

impl PuzzleInput for Parser {
    type Out<'a> = Vec<Reg>;

    fn from_input(input: &str) -> Self::Out<'_> {
        aoc::lines(input)
            .scan((0, 1), |(cycle, reg), line| {
                let r = *reg;
                Some(if line.starts_with('n') {
                    *cycle += 1;
                    [None, Some(r)]
                } else {
                    *cycle += 2;
                    let x: Reg = line[5..].parse().unwrap();
                    *reg += x;
                    [Some(r), Some(r)]
                })
            })
            .flatten()
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_ex() {
        let input = r#"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
        "#;

        // let input = r#"
        // noop
        // addx 3
        // addx -5
        // "#;

        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, Output::Part1(13140));
        assert_eq!(
            res2,
            Output::Part2(String::from(
                r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
                .trim_start()
            ))
        );
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, Output::Part1(16060));
        assert_eq!(
            res2,
            Output::Part2(String::from(
                r#"
###...##...##..####.#..#.#....#..#.####.
#..#.#..#.#..#.#....#.#..#....#..#.#....
###..#..#.#....###..##...#....####.###..
#..#.####.#....#....#.#..#....#..#.#....
#..#.#..#.#..#.#....#.#..#....#..#.#....
###..#..#..##..####.#..#.####.#..#.#....
"#
                .trim_start()
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
