use std::ops::RangeInclusive;

register!(
    "input/day2.txt";
    (input: input!(PasswordInput)) -> usize {
        part1(&input);
        part2(&input);
    }
);

fn part1(input: &[PasswordInput]) -> usize {
    input.iter().filter(|l| is_valid_01(l)).count()
}

fn part2(input: &[PasswordInput]) -> usize {
    input.iter().filter(|l| is_valid_02(l)).count()
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(super) struct PasswordInput {
    range: RangeInclusive<usize>,
    letter: char,
    pass: String,
}

impl From<String> for PasswordInput {
    fn from(s: String) -> Self {
        let mut parts = s.split(['-', ':', ' '].as_ref()).filter(|s| !s.is_empty());

        let min = parts.next().expect("Missing min").parse().unwrap();
        let max = parts.next().expect("Missing max").parse().unwrap();
        let letter = parts.next().expect("Missing letter");
        let pass = parts.next().expect("Missing password");

        PasswordInput {
            range: min..=max,
            letter: letter.as_bytes()[0] as char,
            pass: pass.to_string(),
        }
    }
}

fn is_valid_01(input: &PasswordInput) -> bool {
    let occurrences = input.pass.chars().filter(|c| *c == input.letter).count();
    input.range.contains(&occurrences)
}

fn is_valid_02(input: &PasswordInput) -> bool {
    let pos1 = *input.range.start() - 1;
    let pos2 = *input.range.end() - 1;
    let matches = input
        .pass
        .char_indices()
        .filter(|(idx, _)| *idx == pos1 || *idx == pos2)
        .filter(|(_, ch)| *ch == input.letter)
        .count();
    matches == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn parse() {
        test_parse("1-3 a: bbb", 1..=3, 'a', "bbb");
        test_parse("42-42 a: bbb", 42..=42, 'a', "bbb");
        test_parse("42-1337 a: bbb", 42..=1337, 'a', "bbb");
        test_parse("1337-42 a: bbb", 1337..=42, 'a', "bbb");
        test_parse("1-2 x: x", 1..=2, 'x', "x");
        test_parse("1-2 x: xxxxxxxxxxx", 1..=2, 'x', "xxxxxxxxxxx");
    }

    fn test_parse(input: &str, range: RangeInclusive<usize>, letter: char, pass: &str) {
        let input = PasswordInput::from(input.to_string());
        assert_eq!(
            input,
            PasswordInput {
                range,
                letter,
                pass: String::from(pass)
            }
        );
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 418);
        assert_eq!(res2, 616);
    }
}
