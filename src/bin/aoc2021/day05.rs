use parse_display::FromStr;
use std::{collections::HashMap, iter::repeat};

register!(
    "input/day5.txt";
    (input: input!(parse VentLine)) -> usize {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[VentLine]) -> usize {
    solve(items, false)
}

fn part2(items: &[VentLine]) -> usize {
    solve(items, true)
}

fn solve(items: &[VentLine], include_diagonal: bool) -> usize {
    let mut covered = HashMap::new();
    for VentLine { x1, y1, x2, y2 } in items.iter().copied() {
        if include_diagonal || x1 == x2 || y1 == y2 {
            let xs = (x1..=x2).chain((x2..=x1).rev()).chain(repeat(x1));
            let ys = (y1..=y2).chain((y2..=y1).rev()).chain(repeat(y1));
            let len = 1 + (x2 - x1).unsigned_abs().max((y2 - y1).unsigned_abs()) as usize;
            xs.zip(ys).take(len).for_each(|p| {
                *covered.entry(p).or_insert(0) += 1;
            });
        }
    }
    covered.into_iter().filter(|(_, count)| *count >= 2).count()
}

#[derive(Clone, Copy, Debug, FromStr)]
#[display("{x1},{y1} -> {x2},{y2}")]
pub struct VentLine {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test_ex() {
        let input = r#"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 5);
        assert_eq!(res2, 12);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 4873);
        assert_eq!(res2, 19472);
    }
}
