use derive_more::Deref;

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
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(fst, snd)| (to_set(fst) & to_set(snd)).trailing_zeros())
        .sum()
}

fn part2(items: &[Input]) -> Output {
    items
        .array_chunks()
        .map(|[a, b, c]| (to_set(a) & to_set(b) & to_set(c)).trailing_zeros())
        .sum()
}

fn to_set(items: &[u8]) -> u64 {
    items.iter().fold(0, |acc, item| acc | 1 << *item)
}

fn to_prio(item: u8) -> u8 {
    (item & 0x1F) + 26 * (1 - u8::from(item & 0x20 == 0x20))
}

#[derive(Clone, Debug, Deref)]
pub struct Rucksack(Vec<u8>);

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        Self(s.bytes().map(to_prio).collect())
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
