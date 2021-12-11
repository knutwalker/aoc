register!(
    "input/day3.txt";
    (input: input!(Vec<u8>)) -> usize {
        count_trees((3, 1), &input);
        part2(&input);
    }
);

fn count_trees((slope_right, slope_down): (usize, usize), lines: &[Vec<u8>]) -> usize {
    lines
        .iter()
        .step_by(slope_down)
        .zip((0_usize..).step_by(slope_right))
        .map(is_tree)
        .map(usize::from)
        .sum()
}

fn is_tree((line, index): (impl AsRef<[u8]>, usize)) -> bool {
    let line = line.as_ref();
    line[index % line.len()] == b'#'
}

fn part2(lines: &[Vec<u8>]) -> usize {
    count_trees((1, 1), lines)
        * count_trees((3, 1), lines)
        * count_trees((5, 1), lines)
        * count_trees((7, 1), lines)
        * count_trees((1, 2), lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test_ex() {
        let (res1, res2) = Solver::run_on(
            "..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        ",
        );
        assert_eq!(res1, 7);
        assert_eq!(res2, 336);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 284);
        assert_eq!(res2, 3_510_149_120);
    }
}
