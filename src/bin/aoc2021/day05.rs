use parse_display::FromStr;

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
    let mut covered = vec![0_u8; 1024 * 1024];
    for VentLine { x1, y1, x2, y2 } in items.iter().copied() {
        if include_diagonal || x1 == x2 || y1 == y2 {
            let dx = (x2 - x1).signum();
            let dy = (y2 - y1).signum();
            let (mut x, mut y) = (x1, y1);
            while x != x2 + dx || y != y2 + dy {
                covered[x as usize * 1024 + y as usize] += 1;
                x += dx;
                y += dy;
            }
        }
    }
    covered.into_iter().filter(|c| *c > 1).count()
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
