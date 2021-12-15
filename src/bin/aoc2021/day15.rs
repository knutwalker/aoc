use fxhash::FxBuildHasher;
use graph::prelude::{
    CsrLayout, Graph, GraphBuilder, Idx, Target, UndirectedCsrGraph, UndirectedNeighborsWithValues,
};
use std::cmp::Reverse;

type Input = Vec<u8>;
type G = UndirectedCsrGraph<u32, (), u32>;
type Output = u32;

register!(
    "input/day15.txt";
    (input: input!(Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    let g = graph(items, 1);
    dijkstra(&g, 0, g.node_count() - 1)
}

fn part2(items: &[Input]) -> Output {
    let g = graph(items, 5);
    dijkstra(&g, 0, g.node_count() - 1)
}

fn graph(input: &[Vec<u8>], repeats: u32) -> G {
    let mut edges = Vec::with_capacity(input.len().pow(2) * repeats.pow(2).index() * 4);
    let h = input.len() as u32;
    let w = h;

    let max_h = h * repeats;
    let max_w = w * repeats;

    for xr in 0..repeats {
        for xc in 0..repeats {
            for (row, current_row) in input.iter().enumerate() {
                let row = xr * h + (row as u32);
                for (col, danger) in current_row.iter().map(|b| u32::from(b - b'0')).enumerate() {
                    let col = xc * w + (col as u32);
                    let mut danger = danger + xr + xc;
                    if danger > 9 {
                        danger -= 9;
                    }

                    let target = (row * max_w + col) as u32;

                    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let nr = row.wrapping_add_signed(dr);
                        let nc = col.wrapping_add_signed(dc);
                        if nr < max_h && nc < max_w {
                            let source = (nr * max_w + nc) as u32;
                            edges.push((source, target, danger));
                        }
                    }
                }
            }
        }
    }

    GraphBuilder::new()
        .csr_layout(CsrLayout::Deduplicated)
        .edges_with_values(edges)
        .build()
}

fn dijkstra(g: &G, source: u32, target: u32) -> u32 {
    let mut visited = vec![false; g.node_count().index()];

    let mut q = priority_queue::PriorityQueue::with_capacity_and_hasher(
        g.node_count().index(),
        FxBuildHasher::default(),
    );
    q.push(source, Reverse(0));

    while let Some((node, dist)) = q.pop() {
        if node == target {
            return dist.0;
        }
        visited[node.index()] = true;
        for Target { target, value } in g.neighbors_with_values(node) {
            if !visited[target.index()] {
                let dist = dist.0 + *value;
                match q.get_priority(target) {
                    Some(prev) if dist < prev.0 => {
                        let _ = q.change_priority(target, Reverse(dist));
                    }
                    None => {
                        let _ = q.push(*target, Reverse(dist));
                    }
                    Some(_) => {}
                }
            }
        }
    }

    unreachable!("No path from {} to {}", source, target)
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
