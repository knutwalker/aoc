use aoc::Parse;

type Input = Rucksack;
type Output = u32;

register!(
    "input/day3.txt";
    (input: input!(Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items
        .iter()
        .map(|Rucksack(fst, snd)| *fst & *snd)
        .map(u64::trailing_zeros)
        .sum()
}

fn part2(items: &[Input]) -> Output {
    items
        .iter()
        .map(|Rucksack(fst, snd)| *fst | *snd)
        .array_chunks()
        .map(|[a, b, c]| (a & b & c))
        .map(u64::trailing_zeros)
        .sum()
}

pub struct Rucksack(u64, u64);

impl Parse for Rucksack {
    type Out<'a> = Self;

    fn parse_from(s: &str) -> Self {
        fn to_prio(item: u8) -> u64 {
            1 << ((item & 0x1F) + 26 * (1 - u8::from(item & 0x20 == 0x20)))
        }

        let mid = s.len() / 2;
        let mut items = s.bytes().map(to_prio);
        let fst = items.by_ref().take(mid).fold(0, |acc, item| acc | item);
        let snd = items.fold(0, |acc, item| acc | item);
        Self(fst, snd)
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
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 157);
        assert_eq!(res2, 70);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 7878);
        assert_eq!(res2, 2760);
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
