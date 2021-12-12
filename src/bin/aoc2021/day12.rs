use aoc::ProcessInput;
use indexmap::IndexSet;

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
    graph: [u16; 16],
    start: u8,
    end: u8,
    on_visit: u16,
}

impl Cave {
    fn count_paths(self, can_visit_twice: bool) -> usize {
        fn iterate(c: &Cave, node: u8, visited: u16, twice: bool) -> usize {
            //    graph is a [ u16 ; 16 ] where each u16 is a bitset of adjacent nodes
            // on_visit is a bitset of nodes that can change their visit state (small=1, big/start=0)
            //  visited is a bitset of nodes that have been visited in the current path

            // to get the nodes for the next iteration, we create a mask of nodes that still need
            // to be visted by negating the visted set. If we still have a second small cave visit
            // left, we add all small nodes to the to_visit set by or-ing it with `on_visit`
            // (which has a 1 for every small cave).
            let mut to_visit = c.graph[node as usize];
            to_visit &= !visited | [0, c.on_visit][twice as usize];

            // iterate through the to_visit set
            let mut paths = 0;
            while to_visit != 0 {
                // the next node is the lowest 1 bit, HD 2-1, `x & (-x)`
                let next = to_visit & to_visit.wrapping_neg();
                // remove the next node from the to_visit set, HD 2-1, `x & (x - 1)`
                to_visit &= to_visit - 1;

                // decode the bit position into the node id
                let next_node = next.trailing_zeros() as u8;

                // we don't need to follow the node if it is the end
                if next_node == c.end {
                    paths += 1;
                } else {
                    // if next is a small cave that we already visited it before,
                    // we need to toggle the twice flag
                    // not the double && -- we only want to toggle if twice is currently true
                    let next_twice = twice && next & visited != next;

                    // flag the next node as visited but only if it is a small cave
                    let next_visited = visited | (c.on_visit & next);

                    // dfs into the next node
                    paths += iterate(c, next_node, next_visited, next_twice);
                }
            }

            paths
        }

        iterate(
            &self,
            self.start,
            // start is always visited
            1 << self.start,
            can_visit_twice,
        )
    }
}

impl ProcessInput for Cave {
    type In = input!(parse parse::Connection);

    type Out = Self;

    fn process(input: <Self::In as aoc::PuzzleInput>::Out) -> Self::Out {
        use parse::CaveType;

        let mut ids = IndexSet::new();
        let mut graph = [0; 16];

        for path in input {
            let source = ids.insert_full(path.source).0;
            let target = ids.insert_full(path.target).0;
            graph[source] |= 1 << target;
            graph[target] |= 1 << source;
        }

        let on_visit = ids
            .iter()
            .map(|id| match id {
                CaveType::Small(_) => 1,
                _ => 0,
            })
            .enumerate()
            .fold(0, |on_visit, (id, ov)| on_visit | (ov << id));

        let start = ids.get_index_of(&CaveType::Start).unwrap() as u8;
        let end = ids.get_index_of(&CaveType::End).unwrap() as u8;

        Self {
            graph,
            start,
            end,
            on_visit,
        }
    }
}

#[allow(clippy::use_self)]
mod parse {
    use parse_display::FromStr;

    #[derive(Clone, Debug, FromStr)]
    #[display("{source}-{target}")]
    pub struct Connection {
        pub(super) source: CaveType,
        pub(super) target: CaveType,
    }

    #[derive(Clone, Debug, FromStr, PartialEq, Eq, Hash)]
    pub enum CaveType {
        #[display("start")]
        Start,
        #[display("end")]
        End,
        #[from_str(regex = "(?P<0>[a-z]+)")]
        Small(String),
        #[from_str(regex = "(?P<0>[A-Z]+)")]
        Big(String),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

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

    #[bench]
    fn bench_parsing(b: &mut Bencher) {
        let input = Solver::puzzle_input();
        b.bytes = input.len() as u64;
        b.iter(|| Solver::parse_input(input));
    }

    #[bench]
    fn bench_pt1(b: &mut Bencher) {
        let cave = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| cave.count_paths(false));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let cave = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| cave.count_paths(true));
    }
}
