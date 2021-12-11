use std::collections::{HashMap, HashSet};

use aoc::{ProcessInput, PuzzleInput};

type Input = String;
type Output = String;

register!(
    "input/day21.txt";
    (input: input!(process AllergenList)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(input: &AllergenList) -> Output {
    let allergenic_ingredients = input
        .confirmed
        .values()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let pt1 = input
        .all_ingredients
        .iter()
        .filter(|x| !allergenic_ingredients.contains(x.as_str()))
        .count();

    format!("{}", pt1)
}

fn part2(input: &AllergenList) -> Output {
    let mut ingredients = input
        .confirmed
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect::<Vec<_>>();
    ingredients.sort_unstable_by_key(|(allergen, _)| *allergen);
    let ingredients = ingredients
        .into_iter()
        .map(|(_, ingredient)| ingredient)
        .collect::<Vec<_>>();

    ingredients.join(",")
}

pub struct AllergenList {
    all_ingredients: Vec<String>,
    confirmed: HashMap<String, String>,
}

impl ProcessInput for AllergenList {
    type In = input!(Input);

    type Out = Self;

    fn process(input: <Self::In as PuzzleInput>::Out) -> Self::Out {
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
                let _ = possible.remove(&added);
            }
        }

        let all_ingredients = all_ingredients.into_iter().map(String::from).collect();
        let confirmed = confirmed
            .into_iter()
            .map(|(k, v)| (String::from(k), String::from(v)))
            .collect();

        Self {
            all_ingredients,
            confirmed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

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
}
