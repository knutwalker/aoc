use aoc::ProcessInput;
use fxhash::{FxBuildHasher, FxHashMap};
use std::{array::TryFromSliceError, str::FromStr};
use tap::Tap;

type Output = usize;

register!(
    "input/day14.txt";
    (input: input!(process Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(input: &Input) -> Output {
    count(input, 10)
}

fn part2(input: &Input) -> Output {
    count(input, 40)
}

fn count(Input { template, pairs }: &Input, rounds: usize) -> Output {
    let counts: FxHashMap<u8, usize> =
        FxHashMap::with_capacity_and_hasher(template.len(), FxBuildHasher::default());

    let mut all_pairs: FxHashMap<[u8; 2], usize> =
        FxHashMap::with_capacity_and_hasher(template.len(), FxBuildHasher::default());

    let mut new_pairs: FxHashMap<[u8; 2], usize> =
        FxHashMap::with_capacity_and_hasher(template.len(), FxBuildHasher::default());

    for pair in template.as_bytes().array_windows() {
        *all_pairs.entry(*pair).or_default() += 1;
    }
    all_pairs.insert([template.as_bytes().last().copied().unwrap(), 0], 1);

    for _ in 0..rounds {
        new_pairs.clear();
        for (pair @ &[c1, c2], &count) in &all_pairs {
            if let Some(ins) = pairs.get(pair) {
                *new_pairs.entry([c1, *ins]).or_default() += count;
                *new_pairs.entry([*ins, c2]).or_default() += count;
            } else {
                *new_pairs.entry(*pair).or_default() += count;
            }
        }

        std::mem::swap(&mut all_pairs, &mut new_pairs);
    }

    if rounds % 2 != 0 {
        std::mem::swap(&mut all_pairs, &mut new_pairs);
    }

    let counts = all_pairs
        .into_iter()
        .fold(counts, |cs, ([b, _], count)| {
            cs.tap_mut(|c| *c.entry(b).or_default() += count)
        })
        .into_iter()
        .collect::<Vec<_>>()
        .tap_mut(|v| v.sort_unstable_by_key(|c| c.1));

    let [(_, low), .., (_, high)] = &counts[..] else { unreachable!() };
    high - low
}

#[derive(Clone, Debug)]
pub enum In {
    Template(String),
    Ins([u8; 2], u8),
}

impl FromStr for In {
    type Err = TryFromSliceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some((adj, ins)) = s.split_once(" -> ") {
            Self::Ins(adj.as_bytes().try_into()?, ins.bytes().next().unwrap())
        } else {
            Self::Template(s.to_string())
        })
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    template: String,
    pairs: FxHashMap<[u8; 2], u8>,
}

impl ProcessInput for Input {
    type In = input!(parse In);

    type Out = Self;

    fn process(input: <Self::In as aoc::PuzzleInput>::Out) -> Self::Out {
        let (tpl, in_pairs) = input.split_first().unwrap();
        let In::Template(tpl) = tpl else { unreachable!() };
        let template = tpl.clone();

        let pairs = in_pairs
            .iter()
            .map(|p| {
                let &In::Ins(pair, ins) = p else { unreachable!() };
                (pair, ins)
            })
            .collect();

        Self { template, pairs }
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
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 1588);
        assert_eq!(res2, 2_188_189_693_529);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 2874);
        assert_eq!(res2, 5_208_377_027_195);
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
