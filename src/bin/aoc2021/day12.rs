use aoc::ProcessInput;
use graph::prelude::*;
use indexmap::IndexSet;
use parse_display::FromStr;
use tap::Tap;

type Input = Cave;
type Output = usize;

register!(
    "input/day12.txt";
    (cave: input!(process Input)) -> Output {
        part1(&cave);
        part2(&cave);
    }
);

fn part1(cave: &Input) -> Output {
    cave.all_paths("start", "end", false).count()
}

fn part2(cave: &Input) -> Output {
    cave.all_paths("start", "end", true).count()
}

pub struct Cave {
    g: UndirectedCsrGraph<usize, u32>,
    ids: IndexSet<String>,
}

impl Cave {
    fn all_paths<'g>(&'g self, start: &str, end: &str, can_visit_twice: bool) -> AllPaths<'g> {
        let g = &self.g;

        let start = self.ids.get_index_of(start).unwrap();
        let end = self.ids.get_index_of(end).unwrap();

        let mut can_visit = vec![Visit::Never; g.node_count()];
        for (id, v) in can_visit.iter_mut().enumerate() {
            let visit = if id == start || id == end {
                Visit::Never
            } else if *g.node_value(id) > 1 {
                Visit::Always
            } else {
                Visit::Once
            };
            *v = visit;
        }

        let path = Path::new(start, can_visit, can_visit_twice);

        AllPaths {
            end,
            g,
            queue: vec![path],
            path: None,
            neighbors: [].iter(),
        }
    }

    #[cfg(debug_assertions)]
    fn _debug_paths(&self, paths: &[Path]) {
        println!();
        println!();
        for p in paths.iter().rev().map(|p| {
            p.ids
                .iter()
                .map(|i| self.ids.get_index(*i).unwrap().as_str())
                .collect::<Vec<_>>()
                .join(",")
        }) {
            println!("{}", p);
        }
        println!();
        println!();
    }
}

struct AllPaths<'g> {
    end: usize,
    g: &'g UndirectedCsrGraph<usize, u32>,
    queue: Vec<Path>,
    path: Option<Path>,
    neighbors: std::slice::Iter<'g, usize>,
}

impl<'g> Iterator for AllPaths<'g> {
    type Item = Path;

    #[allow(clippy::semicolon_if_nothing_returned)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            for &n in self.neighbors.by_ref() {
                let p = self.path.as_ref().unwrap().append(n);
                if n == self.end {
                    return Some(p);
                }
                if p.can_visit(n) {
                    self.queue.push(p);
                }
            }

            let mut p = self.queue.pop()?;
            let id = *p.ids.last().unwrap();
            p.visit(id);

            self.path = Some(p);
            self.neighbors = self.g.neighbors(id).iter();
        }
    }
}

#[derive(Clone, Debug)]
struct Path {
    ids: Vec<usize>,
    visited: Vec<bool>,
    can_visit: Vec<Visit>,
    can_visit_twice: bool,
}

impl Path {
    fn new(start: usize, can_visit: Vec<Visit>, can_visit_twice: bool) -> Self {
        Self {
            ids: vec![start],
            visited: vec![false; can_visit.len()],
            can_visit,
            can_visit_twice,
        }
    }

    fn append(&self, node: usize) -> Self {
        self.clone().tap_mut(|p| p.ids.push(node))
    }

    fn can_visit(&self, node: usize) -> bool {
        !matches!(self.can_visit[node], Visit::Never)
            && (self.can_visit_twice || !self.visited[node])
    }

    fn visit(&mut self, node: usize) {
        let can_visit = self.can_visit[node];
        match can_visit {
            Visit::Never | Visit::Always => {}
            Visit::Once => {
                let was_visited = std::mem::replace(&mut self.visited[node], true);
                if was_visited {
                    if self.can_visit_twice {
                        self.can_visit_twice = false;
                    } else {
                        panic!("visited node too many times {}", node)
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum Visit {
    Never,
    Once,
    Always,
}

impl ProcessInput for Cave {
    type In = input!(parse Connection);

    type Out = Self;

    fn process(input: <Self::In as aoc::PuzzleInput>::Out) -> Self::Out {
        let mut ids = IndexSet::new();
        let mut edges = Vec::new();
        for path in input {
            let (source, _) = ids.insert_full(path.source);
            let (target, _) = ids.insert_full(path.target);
            edges.push((source, target));
        }

        let node_values = ids
            .iter()
            .map(|id| {
                let is_big = id.chars().all(char::is_uppercase);
                [1, u32::MAX][is_big as usize]
            })
            .collect::<Vec<_>>();

        let g = GraphBuilder::new()
            .csr_layout(CsrLayout::Deduplicated)
            .edges(edges)
            .node_values(node_values)
            .build();

        Self { g, ids }
    }
}

#[derive(Clone, Debug, FromStr)]
#[display("{source}-{target}")]
pub struct Connection {
    source: String,
    target: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test_ex() {
        let input = r#"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 10);
        assert_eq!(res2, 36);
    }

    #[test]
    fn test_ex2() {
        let input = r#"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 19);
        assert_eq!(res2, 103);
    }

    #[test]
    fn test_ex3() {
        let input = r#"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 226);
        assert_eq!(res2, 3509);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 5756);
        assert_eq!(res2, 144_603);
    }
}
