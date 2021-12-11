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
    (0..usize::MAX)
        .find(|round| octo.flash() == SIZE * SIZE)
        .unwrap()
        + 101
}

const SIZE: usize = 10;

#[derive(Clone, Copy, Debug)]
pub struct Octo([[u8; SIZE]; SIZE]);

impl Octo {
    fn flash(&mut self) -> usize {
        for xs in &mut self.0 {
            for o in xs {
                *o += 1;
            }
        }

        loop {
            let mut is_flashing = false;
            for row in 0..SIZE {
                for col in 0..SIZE {
                    if self.0[row][col] == 10 {
                        is_flashing = true;
                        // this one goes to 11
                        self.0[row][col] = 11;
                        self.adjacent(row, col, |nb| {
                            if *nb < 10 {
                                *nb += 1;
                            }
                        });
                    }
                }
            }

            if !is_flashing {
                break;
            }
        }

        self.0
            .iter_mut()
            .flat_map(|xs| xs.iter_mut())
            .filter(|x| **x >= 10)
            .map(|x| {
                *x = 0;
            })
            .count()
    }

    fn adjacent(&mut self, row: usize, col: usize, action: impl Fn(&mut u8)) {
        if let Some(r) = row.checked_sub(1) {
            let row = &mut self.0[r];
            if let Some(c) = col.checked_sub(1) {
                action(&mut row[c]);
            }
            action(&mut row[col]);
            if let Some(v) = row.get_mut(col + 1) {
                action(v);
            }
        }

        {
            let row = &mut self.0[row];
            if let Some(c) = col.checked_sub(1) {
                action(&mut row[c]);
            }
            action(&mut row[col]);
            if let Some(v) = row.get_mut(col + 1) {
                action(v);
            }
        }

        if let Some(row) = self.0.get_mut(row + 1) {
            if let Some(c) = col.checked_sub(1) {
                action(&mut row[c]);
            }
            action(&mut row[col]);
            if let Some(v) = row.get_mut(col + 1) {
                action(v);
            }
        }
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
    use aoc::SolutionExt;

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
}
