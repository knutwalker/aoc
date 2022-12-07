use aoc::{lines, PuzzleInput};

type Output = usize;

register!(
    "input/day7.txt";
    (input: input!(verbatim Parser)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[usize]) -> Output {
    items.iter().filter(|size| **size <= 100_000).sum()
}

fn part2(items: &[usize]) -> Output {
    let total = *items.last().unwrap();
    let unused = 70_000_000 - total;
    let need_to_delete = 30_000_000 - unused;

    items
        .iter()
        .copied()
        .filter(|size| *size >= need_to_delete)
        .min()
        .unwrap()
}

pub enum Parser {}

impl PuzzleInput for Parser {
    type Out<'a> = Vec<usize>;

    fn from_input(input: &str) -> Self::Out<'_> {
        let mut sizes = Vec::new();
        let mut path_stack = Vec::new();
        let mut file_size = 0;

        for line in lines(input) {
            if let Some(cd) = line.strip_prefix("$ cd ") {
                if cd == ".." {
                    sizes.push(file_size);
                    let parent_size = path_stack.pop().unwrap();
                    file_size += parent_size;
                } else {
                    path_stack.push(file_size);
                    file_size = 0;
                }
            } else {
                let (kind, _) = line.split_once(' ').unwrap();
                if kind != "dir" && kind != "$" {
                    let size = kind.parse::<usize>().unwrap();
                    file_size += size;
                }
            }
        }

        while let Some(parent_size) = path_stack.pop() {
            sizes.push(file_size);
            file_size += parent_size;
        }

        sizes
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
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 95437);
        assert_eq!(res2, 24_933_642);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1_845_346);
        assert_eq!(res2, 3_636_703);
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
