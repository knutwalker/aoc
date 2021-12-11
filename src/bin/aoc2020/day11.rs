use std::iter::successors;

register!(
    "input/day11.txt";
    (input: input!(Vec<u8>)) -> usize {
        run_any(input.clone(), 1, 4);
        run_any(input, usize::max_value(), 5);
    }
);

fn run_any(mut input: Vec<Vec<u8>>, dist: usize, full: usize) -> usize {
    loop {
        let new_input = input
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, col)| match col {
                        b'L' if occupied_seats(x, y, dist, &input) == 0 => b'#',
                        b'#' if occupied_seats(x, y, dist, &input) >= full => b'L',
                        otherwise => *otherwise,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        if new_input == input {
            break new_input
                .into_iter()
                .flatten()
                .filter(|&c| c == b'#')
                .count();
        }
        input = new_input;
    }
}

fn occupied_seats(x: usize, y: usize, limit: usize, rows: &[Vec<u8>]) -> usize {
    (-1..=1)
        .flat_map(|dx| {
            (-1..=1)
                .filter(move |&dy| dx != 0 || dy != 0)
                .map(move |dy| {
                    go(x, y, dx, dy, rows[y].len(), rows.len())
                        .take(limit)
                        .map(|(x, y)| rows[y][x])
                        .take_while(|&s| s != b'L')
                        .any(|s| s == b'#') as usize
                })
        })
        .sum()
}

fn go(
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    mx: usize,
    my: usize,
) -> impl Iterator<Item = (usize, usize)> {
    successors(Some((x as isize, y as isize)), move |&(x, y)| {
        Some((x + dx, y + dy))
            .filter(|&(x, y)| x >= 0 && x < (mx as isize) && y >= 0 && y < (my as isize))
    })
    .skip(1)
    .map(|(x, y)| (x as usize, y as usize))
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 2093);
        assert_eq!(res2, 1862);
    }

    #[test]
    fn test_ex1() {
        assert_eq!(
            (37, 26),
            Solver::run_on(
                "
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
    "
            )
        );
    }

    #[test]
    fn occupied_test1() {
        let input = "
            .......#.
            ...#.....
            .#.......
            .........
            ..#L....#
            ....#....
            .........
            #........
            ...#.....
        "
        .trim()
        .lines()
        .map(|s| s.trim().as_bytes().to_vec())
        .collect::<Vec<_>>();

        assert_eq!(8, occupied_seats(3, 4, usize::max_value(), &input));
    }

    #[test]
    fn occupied_test2() {
        let input = "
            .............
            .L.L.#.#.#.#.
            .............
        "
        .trim()
        .lines()
        .map(|s| s.trim().as_bytes().to_vec())
        .collect::<Vec<_>>();

        assert_eq!(0, occupied_seats(1, 1, usize::max_value(), &input));
    }

    #[test]
    fn occupied_test3() {
        let input = "
            .##.##.
            #.#.#.#
            ##...##
            ...L...
            ##...##
            #.#.#.#
            .##.##.
        "
        .trim()
        .lines()
        .map(|s| s.trim().as_bytes().to_vec())
        .collect::<Vec<_>>();

        assert_eq!(0, occupied_seats(3, 3, usize::max_value(), &input));
    }

    #[test]
    fn occupied_test4() {
        let input = "
            #.LL.LL.L#
            #LLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLL#
            #.LLLLLL.L
            #.LLLLL.L#
        "
        .trim()
        .lines()
        .map(|s| s.trim().as_bytes().to_vec())
        .collect::<Vec<_>>();

        assert_eq!(0, occupied_seats(3, 0, usize::max_value(), &input));
    }
}
