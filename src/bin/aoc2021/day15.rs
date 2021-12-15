use fxhash::{FxBuildHasher, FxHashSet};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

type Input = Vec<u8>;
type Output = u32;

register!(
    "input/day15.txt";
    (input: input!(Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    dijkstra(items, 1, (0, 0))
}

fn part2(items: &[Input]) -> Output {
    dijkstra(items, 5, (0, 0))
}

fn dijkstra(g: &[Vec<u8>], scale: u16, start: (u16, u16)) -> u32 {
    let mut visited = FxHashSet::default();
    let size = g.len() as u16;
    let max = size * scale;

    let end = (max - 1, max - 1);

    let mut q = PriorityQueue::with_capacity_and_hasher(g.len(), FxBuildHasher::default());
    q.push(start, Reverse(0));

    while let Some((node @ (row, col), dist)) = q.pop() {
        if node == end {
            return dist.0;
        }
        visited.insert(node);

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = row.wrapping_add_signed(dr);
            let nc = col.wrapping_add_signed(dc);
            if nr < max && nc < max {
                let target = (nr, nc);
                if !visited.contains(&target) {
                    let mut danger =
                        u16::from(g[(nr % size) as usize][(nc % size) as usize] - b'0');
                    danger += (nr / size) + (nc / size);
                    if danger > 9 {
                        danger -= 9;
                    }

                    let dist = dist.0 + u32::from(danger);
                    match q.get_priority(&target) {
                        Some(prev) if dist < prev.0 => {
                            let _ = q.change_priority(&target, Reverse(dist));
                        }
                        None => {
                            let _ = q.push(target, Reverse(dist));
                        }
                        Some(_) => {}
                    }
                }
            }
        }
    }

    unreachable!("No path")
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 40);
        assert_eq!(res2, 315);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 621);
        assert_eq!(res2, 2904);
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
