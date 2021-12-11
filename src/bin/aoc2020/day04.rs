use aoc::{As, Blocks, ProcessInput, PuzzleInput};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

register!(
    "input/day4.txt";
    (input: input!(process Passports)) -> usize {
        part1(&input);
        part2(&input);
    }
);

fn part1(passports: &[Passport]) -> usize {
    passports.iter().map(|p| p.is_valid_pt1() as usize).sum()
}

fn part2(passports: &[Passport]) -> usize {
    passports.iter().map(|p| p.is_valid_pt2() as usize).sum()
}

#[derive(Default, Debug)]
pub struct Passport(HashMap<String, String>);

impl Deref for Passport {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Passport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Passport {
    fn is_valid_pt1(&self) -> bool {
        self.contains_key("byr")
            && self.contains_key("iyr")
            && self.contains_key("eyr")
            && self.contains_key("hgt")
            && self.contains_key("hcl")
            && self.contains_key("ecl")
            && self.contains_key("pid")
    }

    fn is_valid_pt2(&self) -> bool {
        let byr = self
            .get("byr")
            .and_then(|c| c.parse::<u16>().ok())
            .filter(|byr| (1920..=2002).contains(byr))
            .is_some();

        let iyr = self
            .get("iyr")
            .and_then(|c| c.parse::<u16>().ok())
            .filter(|iyr| (2010..=2020).contains(iyr))
            .is_some();

        let eyr = self
            .get("eyr")
            .and_then(|c| c.parse::<u16>().ok())
            .filter(|eyr| (2020..=2030).contains(eyr))
            .is_some();

        let hgt = self
            .get("hgt")
            .filter(|c| {
                let (value, unit) = c.split_at(c.len() - 2);
                matches!(
                    (value.parse::<u8>().ok(), unit),
                    (Some(59..=76), "in") | (Some(150..=193), "cm")
                )
            })
            .is_some();

        let hcl = self
            .get("hcl")
            .filter(|c| c.len() == 7)
            .filter(|c| &c[0..1] == "#" && c[1..].bytes().all(|b| b.is_ascii_hexdigit()))
            .is_some();

        let ecl = self
            .get("ecl")
            .filter(|c| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&***c))
            .is_some();

        let pid = self
            .get("pid")
            .filter(|c| c.len() == 9 && c.bytes().all(|c| c.is_ascii_digit()))
            .is_some();

        byr && iyr && eyr && hgt && hcl && ecl && pid
    }
}

pub struct Passports;

impl ProcessInput for Passports {
    type In = input!(chunk String);

    type Out = Vec<Passport>;

    fn process(input: <Blocks<As<String>> as PuzzleInput>::Out) -> Self::Out {
        input
            .into_iter()
            .map(|block| {
                block.into_iter().fold(Passport::default(), |mut pp, line| {
                    for input in line.split_ascii_whitespace() {
                        let mut parts = input.splitn(2, ':');
                        pp.insert(
                            parts.next().expect("no field").to_string(),
                            parts.next().expect("no content").to_string(),
                        );
                    }
                    pp
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn example_part1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                   byr:1937 iyr:2017 cid:147 hgt:183cm

                   iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                   hcl:#cfa07d byr:1929

                   hcl:#ae17e1 iyr:2013
                   eyr:2024
                   ecl:brn pid:760753108 byr:1931
                   hgt:179cm

                   hcl:#cfa07d eyr:2025 pid:166559648
                   iyr:2011 ecl:brn hgt:59in

                   ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                   byr:1937 iyr:2017 cid:147 hgt:183cm";
        assert_eq!(Solver::run_on(input).0, 3);
    }

    #[test]
    fn valid_part2() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
                    hcl:#623a2f

                    eyr:2029 ecl:blu cid:129 byr:1989
                    iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

                    hcl:#888785
                    hgt:164cm byr:2001 iyr:2015 cid:88
                    pid:545766238 ecl:hzl
                    eyr:2022

                    iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        ";
        assert_eq!(Solver::run_on(input).1, 4);
    }

    #[test]
    fn invalid_part2() {
        let input = "eyr:1972 cid:100
                    hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

                    iyr:2019
                    hcl:#602927 eyr:1967 hgt:170cm
                    ecl:grn pid:012533040 byr:1946

                    hcl:dab227 iyr:2012
                    ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

                    hgt:59cm ecl:zzz
                    eyr:2038 hcl:74454a iyr:2023
                    pid:3556412378 byr:2007
        ";
        assert_eq!(Solver::run_on(input).1, 0);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 230);
        assert_eq!(res2, 156);
    }
}
