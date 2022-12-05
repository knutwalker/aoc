use std::iter::successors;

register!(
    "input/day11.txt";
    (input: input!([u8])) -> usize {
        run_any(&input, 1, 4);
        run_any(&input, usize::max_value(), 5);
    }
);

fn run_any(input: &[&[u8]], dist: usize, full: usize) -> usize {
    let mut new_input = next_input(input, dist, full);
    if new_input == input {
        return score(new_input);
    }
    loop {
        let prev_input = new_input;
        new_input = next_input(&prev_input, dist, full);
        if new_input == prev_input {
            return score(new_input);
        }
    }
}

fn next_input<T>(input: &[T], dist: usize, full: usize) -> Vec<Vec<u8>>
where
    T: Input,
{
    input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, col)| match col {
                    b'L' if occupied_seats(x, y, dist, input) == 0 => b'#',
                    b'#' if occupied_seats(x, y, dist, input) >= full => b'L',
                    otherwise => otherwise,
                })
                .collect()
        })
        .collect()
}

fn score(input: Vec<Vec<u8>>) -> usize {
    input.into_iter().flatten().filter(|c| *c == b'#').count()
}

fn occupied_seats<T>(x: usize, y: usize, limit: usize, rows: &[T]) -> usize
where
    T: Input,
{
    (-1..=1)
        .flat_map(|dx| {
            (-1..=1)
                .filter(move |&dy| dx != 0 || dy != 0)
                .map(move |dy| {
                    go(x, y, dx, dy, rows[y].len(), rows.len())
                        .take(limit)
                        .map(|(x, y)| rows[y].get(x))
                        .take_while(|&s| s != b'L')
                        .any(|s| s == b'#')
                })
                .map(usize::from)
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

trait Input {
    type Iter<'a>: Iterator<Item = u8> + 'a
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;

    fn len(&self) -> usize;

    fn get(&self, index: usize) -> u8;
}

impl Input for Vec<u8> {
    type Iter<'a> = std::iter::Copied<std::slice::Iter<'a, u8>>;

    fn iter(&self) -> Self::Iter<'_> {
        (**self).iter().copied()
    }

    fn len(&self) -> usize {
        (**self).len()
    }

    fn get(&self, index: usize) -> u8 {
        self[index]
    }
}

impl<'x> Input for &'x [u8] {
    type Iter<'a> = std::iter::Copied<std::slice::Iter<'a, u8>>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        (*self).iter().copied()
    }

    fn len(&self) -> usize {
        (*self).len()
    }

    fn get(&self, index: usize) -> u8 {
        self[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

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
        .map(|s| s.trim().as_bytes())
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
        .map(|s| s.trim().as_bytes())
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
        .map(|s| s.trim().as_bytes())
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
        .map(|s| s.trim().as_bytes())
        .collect::<Vec<_>>();

        assert_eq!(0, occupied_seats(3, 0, usize::max_value(), &input));
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
        b.iter(|| run_any(&input, 1, 4));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| run_any(&input, usize::max_value(), 5));
    }
}
