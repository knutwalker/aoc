use aoc::ProcessInput;
use indexmap::IndexSet;
use parse_display::FromStr;
use std::simd::{u16x16, Simd};

type Input = Cave;
type Output = usize;

register!(
    "input/day12.txt";
    (cave: input!(process Input)) -> Output {
        cave.count_paths(false);
        cave.count_paths(true);
    }
);

#[derive(Clone, Copy, Debug)]
pub struct Cave {
    graph: u16x16,
    start: u8,
    end: u8,
    can_visit: u32,
}

impl Cave {
    fn count_paths(self, can_visit_twice: bool) -> usize {
        let mut paths = 0;
        let path = Path::new(self.start, self.can_visit, can_visit_twice);
        let mut queue = Vec::with_capacity(32);
        queue.push(path);
        while let Some(p) = queue.pop() {
            let p = p.visit();
            let mut neighbors = self.graph[p.node as usize];
            while neighbors != 0 {
                let n = neighbors & neighbors.wrapping_neg();
                neighbors ^= n;
                let n = n.trailing_zeros() as u8;
                if n == self.end {
                    paths += 1;
                } else if p.can_visit(n as u8) {
                    queue.push(p.append(n));
                }
            }
        }
        paths
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Path {
    node: u8,
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

    fn new(node: u8, can_visit: u32, can_visit_twice: bool) -> Self {
        let visited = can_visit | [0, Self::VISIT_TWICE][can_visit_twice as usize];
        Self { node, visited }
    }

    #[must_use]
    fn append(self, node: u8) -> Self {
        Self { node, ..self }
    }

    fn can_visit(self, node: u8) -> bool {
        let can_visit = (self.visited >> (node << 1)) & Self::MASK;
        if can_visit == Self::ZERO {
            (self.visited & Self::VISIT_TWICE) == Self::VISIT_TWICE
        } else {
            can_visit != Self::NOPE
        }
    }

    #[must_use]
    fn visit(self) -> Self {
        let Self { node, mut visited } = self;
        let can_visit = (visited >> (node << 1)) & Self::MASK;
        if can_visit == Self::ONCE {
            visited |= Self::ZERO << (node << 1);
            Self { node, visited }
        } else if can_visit == Self::ZERO {
            assert!(
                visited & Self::VISIT_TWICE == Self::VISIT_TWICE,
                "visited node too many times {}",
                node
            );
            visited &= !Self::VISIT_TWICE;
            Self { node, visited }
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
        let mut graph: u16x16 = Simd::from_array([0; 16]);

        for path in input {
            let source = ids.insert_full(path.source).0;
            let target = ids.insert_full(path.target).0;
            graph[source] |= 1 << target;
            graph[target] |= 1 << source;
        }

        let can_visit = ids
            .iter()
            .map(|id| match id.as_str() {
                "start" | "end" => Path::NOPE,
                id if id.chars().all(char::is_uppercase) => Path::FULL,
                _ => Path::ONCE,
            })
            .enumerate()
            .fold(0, |visited, (id, nv)| visited | (nv << (id << 1)));

        let start = ids.get_index_of("start").unwrap() as u8;
        let end = ids.get_index_of("end").unwrap() as u8;

        Self {
            graph,
            start,
            end,
            can_visit,
        }
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
