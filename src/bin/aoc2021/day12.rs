use aoc::ProcessInput;
use graph::prelude::*;
use indexmap::IndexSet;
use parse_display::FromStr;

type Input = Cave;
type Output = usize;

register!(
    "input/day12.txt";
    (cave: input!(process Input)) -> Output {
        cave.all_paths(false).count();
        cave.all_paths(true).count();
    }
);

pub struct Cave {
    g: UndirectedCsrGraph<u32, u32>,
    start: u32,
    end: u32,
}

impl Cave {
    fn all_paths(&self, can_visit_twice: bool) -> AllPaths<'_> {
        let path = Path::new(self.start, can_visit_twice, &self.g);
        let mut queue = Vec::with_capacity(32);
        queue.push(path);
        AllPaths {
            end: self.end,
            g: &self.g,
            queue,
            path,
            neighbors: [].iter(),
        }
    }
}

struct AllPaths<'g> {
    end: u32,
    g: &'g UndirectedCsrGraph<u32, u32>,
    queue: Vec<Path>,
    path: Path,
    neighbors: std::slice::Iter<'g, u32>,
}

impl<'g> Iterator for AllPaths<'g> {
    type Item = Path;

    #[allow(clippy::semicolon_if_nothing_returned)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            for &n in self.neighbors.by_ref() {
                let p = self.path.append(n);
                if n == self.end {
                    return Some(p);
                }
                if p.can_visit() {
                    self.queue.push(p);
                }
            }

            let p = self.queue.pop()?.visit();
            self.neighbors = self.g.neighbors(p.head).iter();
            self.path = p;
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Path {
    head: u32,
    visited: u32,
}

impl Path {
    const VISIT_TWICE: u32 = 0x4000_0000;

    // chosen so that ONCE | ZERO = ZERO
    const MASK: u32 = 0b11;
    const ZERO: u32 = 0b11;
    const ONCE: u32 = 0b01;
    const FULL: u32 = 0b10;
    const NOPE: u32 = 0b00;

    fn new(head: u32, can_visit_twice: bool, g: &UndirectedCsrGraph<u32, u32>) -> Self {
        const MAX_NODE_COUNT: u32 = 15;

        assert!(
            g.node_count() <= MAX_NODE_COUNT,
            "can only run on {} nodes, but got {}",
            MAX_NODE_COUNT,
            g.node_count()
        );

        let visited = (0..g.node_count()).fold(
            [0, Self::VISIT_TWICE][can_visit_twice as usize],
            |visited, id| visited | ((*g.node_value(id)) << (id << 1)),
        );

        Self { head, visited }
    }

    #[must_use]
    fn append(self, head: u32) -> Self {
        Self { head, ..self }
    }

    fn can_visit(self) -> bool {
        let Self { head, visited } = self;
        let can_visit = (visited >> (head << 1)) & Self::MASK;
        if can_visit == Self::ZERO {
            (visited & Self::VISIT_TWICE) == Self::VISIT_TWICE
        } else {
            can_visit != Self::NOPE
        }
    }

    #[must_use]
    fn visit(self) -> Self {
        let Self { head, mut visited } = self;
        let can_visit = (visited >> (head << 1)) & Self::MASK;
        if can_visit == Self::ONCE {
            visited |= Self::ZERO << (head << 1);
            Self { head, visited }
        } else if can_visit == Self::ZERO {
            assert!(
                visited & Self::VISIT_TWICE == Self::VISIT_TWICE,
                "visited node too many times {}",
                head
            );
            visited &= !Self::VISIT_TWICE;
            Self { head, visited }
        } else {
            self
        }
    }
}

impl ProcessInput for Cave {
    type In = input!(parse Connection);

    type Out = Self;

    fn process(input: <Self::In as aoc::PuzzleInput>::Out) -> Self::Out {
        let mut ids = IndexSet::new();
        let mut edges = Vec::new();
        for path in input {
            let source = ids.insert_full(path.source).0 as u32;
            let target = ids.insert_full(path.target).0 as u32;
            edges.push((source, target));
        }

        let node_values = ids
            .iter()
            .map(|id| match id.as_str() {
                "start" | "end" => Path::NOPE,
                id if id.chars().all(char::is_uppercase) => Path::FULL,
                _ => Path::ONCE,
            })
            .collect::<Vec<_>>();

        let g = GraphBuilder::new()
            .csr_layout(CsrLayout::Deduplicated)
            .edges(edges)
            .node_values(node_values)
            .build();

        let start = ids.get_index_of("start").unwrap() as u32;
        let end = ids.get_index_of("end").unwrap() as u32;

        Self { g, start, end }
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
