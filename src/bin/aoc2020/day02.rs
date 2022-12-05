use aoc::{lines, PuzzleInput};

register!(
    "input/day2.txt";
    (input: input!(verbatim PasswordInput)) -> usize {
        part1(input);
        part2(input);
    }
);

fn part1((pt1, _): (usize, usize)) -> usize {
    pt1
}

fn part2((_, pt2): (usize, usize)) -> usize {
    pt2
}

#[derive(Copy, Clone, Debug)]
pub struct PasswordInput {
    pt1: bool,
    pt2: bool,
}

impl PuzzleInput for PasswordInput {
    type Out<'a> = (usize, usize);

    fn from_input(input: &str) -> Self::Out<'_> {
        lines(input)
            .map(Self::parse)
            .map(|p| (usize::from(p.pt1), usize::from(p.pt2)))
            .reduce(|(la, lb), (ra, rb)| (la + ra, lb + rb))
            .unwrap()
    }
}

impl PasswordInput {
    fn parse(input: &str) -> Self {
        let (min, input) = input.split_once('-').unwrap();
        let min = min.parse::<usize>().unwrap();
        let (max, input) = input.split_once(' ').unwrap();
        let max = max.parse::<usize>().unwrap();
        let (letter, input) = input.split_once(':').unwrap();
        let letter = letter.as_bytes()[0];
        let pass = input[1..].as_bytes();

        #[allow(clippy::naive_bytecount)]
        let pt1 = pass.iter().filter(|b| **b == letter).count();
        let pt1 = pt1 >= min && pt1 <= max;
        let pt2 = (pass[min - 1] == letter) ^ (pass[max - 1] == letter);

        Self { pt1, pt2 }
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
        assert_eq!(res1, 418);
        assert_eq!(res2, 616);
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
        b.iter(|| part1(input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(input));
    }
}
