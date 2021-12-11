use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

type Input = Vec<u8>;
type Output = usize;
type Cell = (isize, isize, isize, isize);
type Grid = HashSet<Cell>;

register!(
    "input/day17.txt";
    (input: input!(Input)) -> Output {
        run_any(&input, false);
        run_any(&input, true);
    }
);

fn run_any(input: &[Input], fourth_dim: bool) -> Output {
    let mut grid = Grid::with_capacity(4096);
    for (z, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == b'#' {
                grid.insert((x as isize, z as isize, 0, 0));
            }
        }
    }

    successors(Some(grid), |g| Some(cycle(g, fourth_dim)))
        .nth(6)
        .unwrap()
        .len()
}

fn cycle(grid: &Grid, fourth_dim: bool) -> Grid {
    #[cfg(test)]
    {
        print_grid(grid);
    }

    count_neighbors(grid, fourth_dim)
        .into_iter()
        .filter_map(|(cell, alive)| match (grid.contains(&cell), alive) {
            (true, 2..=3) | (false, 3) => Some(cell),
            _ => None,
        })
        .collect()
}

fn count_neighbors(grid: &Grid, fourth_dim: bool) -> HashMap<Cell, isize> {
    let cap = grid.len() * 3_usize.pow(if fourth_dim { 4 } else { 3 });
    let mut counts = HashMap::with_capacity(cap);
    for cell in grid {
        for cell in neighbours(cell, fourth_dim) {
            *counts.entry(cell).or_default() += 1;
        }
    }
    counts
}

fn neighbours(&(x, y, z, w): &Cell, fourth_dim: bool) -> impl Iterator<Item = Cell> {
    ((x - 1)..=(x + 1))
        .flat_map(move |x| {
            ((y - 1)..=(y + 1)).flat_map(move |y| {
                ((z - 1)..=(z + 1)).flat_map(move |z| {
                    if fourth_dim { (w - 1)..=(w + 1) } else { w..=w }.map(move |w| (x, y, z, w))
                })
            })
        })
        .filter(move |(nx, ny, nz, nw)| *nx != x || *ny != y || *nz != z || *nw != w)
}

#[cfg(test)]
fn print_grid(col: &Grid) {
    println!();
    println!("==========================");
    println!();
    for z in -1..=1 {
        for y in -1..=3 {
            for x in -1..=3 {
                print!(
                    "{}",
                    if col.contains(&(x, y, z, 0)) {
                        "#"
                    } else {
                        "."
                    }
                );
            }
            println!();
        }
        println!();
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 218);
        assert_eq!(res2, 1908);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(
            (112, 848),
            Solver::run_on(
                "
                .#.
                ..#
                ###
            ",
            )
        );
    }
}
