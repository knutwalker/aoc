use aoc::ProcessInput;
use std::collections::{HashMap, HashSet};

type Output = String;

register!(
    "input/day21.txt";
    (input: input!(process AllergensInput)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(input: &AllergenList) -> Output {
    let allergenic_ingredients = input.confirmed.values().collect::<HashSet<_>>();
    let pt1 = input
        .all_ingredients
        .iter()
        .filter(|x| !allergenic_ingredients.contains(x))
        .count();

    format!("{pt1}")
}

fn part2(input: &AllergenList) -> Output {
    let mut ingredients = input
        .confirmed
        .iter()
        .map(|(&k, &v)| (k, v))
        .collect::<Vec<_>>();
    ingredients.sort_unstable_by_key(|(allergen, _)| *allergen);
    let ingredients = ingredients
        .into_iter()
        .map(|(_, ingredient)| ingredient)
        .collect::<Vec<_>>();

    ingredients.join(",")
}

pub struct AllergenList<'a> {
    all_ingredients: Vec<&'a str>,
    confirmed: HashMap<&'a str, &'a str>,
}

pub struct AllergensInput;

impl ProcessInput for AllergensInput {
    type In = input!(str);

    type Out<'a> = AllergenList<'a>;

    fn process(input: <Self::In as aoc::PuzzleInput>::Out<'_>) -> Self::Out<'_> {
        let mut all_ingredients = Vec::<&str>::new();
        let mut possible = HashMap::<_, Vec<_>>::new();

        for food in &input {
            let mut parts = food.split(" (contains ");

            let ingredients = parts.next().unwrap();
            let ingredients = ingredients.split(' ').collect::<HashSet<_>>();
            all_ingredients.extend(ingredients.iter());

            let allergenes = parts.next().unwrap().trim_end_matches(')');
            for allergen in allergenes.split(", ") {
                possible
                    .entry(allergen)
                    .or_default()
                    .push(ingredients.clone());
            }
        }

        let mut confirmed = HashMap::new();
        let mut added = Vec::new();
        while !possible.is_empty() {
            for (allergen, ingredients) in &possible {
                let mut possible = ingredients
                    .iter()
                    .fold(None::<HashSet<&str>>, |intersection, b| {
                        Some(intersection.map_or_else(|| b.clone(), |a| &a & b))
                    })
                    .unwrap_or_default();

                if possible.len() > 1 {
                    possible = &possible - &confirmed.values().copied().collect();
                }

                if possible.len() == 1 {
                    added.push(*allergen);
                    confirmed.insert(*allergen, possible.into_iter().next().unwrap());
                }
            }

            for added in added.drain(..) {
                drop(possible.remove(&added));
            }
        }

        AllergenList {
            all_ingredients,
            confirmed,
        }
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
        assert_eq!(res1.as_str(), "2170");
        assert_eq!(
            res2.as_str(),
            "nfnfk,nbgklf,clvr,fttbhdr,qjxxpr,hdsm,sjhds,xchzh"
        );
    }

    #[test]
    fn test_pt1() {
        assert_eq!(
            (String::from("5"), String::from("mxmxvkd,sqjhc,fvjkl")),
            Solver::run_on(
                "
                mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
                trh fvjkl sbzzf mxmxvkd (contains dairy)
                sqjhc fvjkl (contains soy)
                sqjhc mxmxvkd sbzzf (contains fish)
            ",
            )
        );
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
