use std::{num::ParseIntError, str::FromStr};

use tap::Tap;

type Input = Pos;
type Output = u64;

register!(
    "input/day21.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    let p1 = items[0].0 - 1;
    let p2 = items[1].0 - 1;

    let mut counting_die = deterministic().enumerate();
    let mut die = counting_die.by_ref().map(|(_, r)| r);

    let loser_points = u64::from(play_normal((p1, 0), (p2, 0), &mut die));
    let rolls = counting_die.next().unwrap().0 as u64;

    poop!("loser_points = {}", loser_points);
    poop!("rolls = {}", rolls);

    loser_points * rolls
}

fn deterministic() -> impl Iterator<Item = u16> {
    std::iter::successors(Some(1), |num| Some(*num % 100 + 1))
}

fn play_normal(mut p1: (u16, u16), mut p2: (u16, u16), die: &mut impl Iterator<Item = u16>) -> u16 {
    while p1.1 < 1000 && p2.1 < 1000 {
        let roll = roll_die(die);
        p1 = move_player(p1.0, p1.1, roll);
        std::mem::swap(&mut p1, &mut p2);
    }
    u16::min(p1.1, p2.1)
}

fn roll_die(die: &mut impl Iterator<Item = u16>) -> u16 {
    die.take(3).sum()
}

#[inline]
const fn move_player(pos: u16, score: u16, roll: u16) -> (u16, u16) {
    let pos = (pos + roll) % 10;
    let score = score + pos + 1;
    (pos, score)
}

fn part2(items: &[Input]) -> Output {
    // there are 1 ways to roll a 3, 3 ways to roll a 4, etc
    const ROLL_AMOUNT: [(u16, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut p1 = Counts::new(items[0].0 - 1);
    let mut p2 = Counts::new(items[1].0 - 1);

    loop {
        let opponent_still_playing = p2.playing_universes();
        if opponent_still_playing == 0 {
            break;
        }

        // record the universes still in play for the next player
        // if the player wins in every move, this will remain empty
        // and we can determin the overall winner in the next iteration
        let mut counts = Counts::empty();

        // iterate through every (pos, score) combination
        for score in 0..21 {
            for pos in 0..10 {
                // in how many universes is this player on (pos, score)
                let universes = p1.get(pos, score);
                for (roll, amount) in ROLL_AMOUNT {
                    let (pos, score) = move_player(pos, score, roll);
                    if score >= 21 {
                        // this is a winning (pos, score) with _amount_ many rolls
                        // for every of those universes, we also win for every other universe
                        // that the opponent hasn't won
                        p1.wins += universes * amount * opponent_still_playing;
                    } else {
                        // not a win, record in how many universes we will arrive here
                        counts.add(pos, score, amount * universes);
                    }
                }
            }
        }

        // ignore all previous counts, store only the new counts without any won universes
        p1.playing = counts.playing;
        std::mem::swap(&mut p1, &mut p2);
    }

    u64::max(p1.wins, p2.wins)
}

#[derive(Debug, Clone)]
struct Counts {
    /// there are only 210 non winning combinations of (pos, score)
    /// given that pos is in 1..=10 and score is in 0..=20
    /// this counts in how many universes a player is on a certain position with a certain score
    playing: [u64; 210],
    /// In how many universes the player has won
    wins: u64,
}

impl Counts {
    const fn empty() -> Self {
        Self {
            playing: [0; 210],
            wins: 0,
        }
    }

    fn new(pos: u16) -> Self {
        Self::empty().tap_mut(|c| c.add(pos, 0, 1))
    }

    fn get(&self, pos: u16, score: u16) -> u64 {
        self.playing[usize::from(pos * 21 + score)]
    }

    fn add(&mut self, pos: u16, score: u16, value: u64) {
        self.playing[usize::from(pos * 21 + score)] += value;
    }

    fn playing_universes(&self) -> u64 {
        self.playing.into_iter().sum()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Pos(u16);

impl FromStr for Pos {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.rsplit_once(' ').unwrap().1.parse()?))
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
        Player 1 starting position: 4
        Player 2 starting position: 8
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 739_785);
        assert_eq!(res2, 444_356_092_776_315);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 913_560);
        assert_eq!(res2, 110_271_560_863_819);
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
