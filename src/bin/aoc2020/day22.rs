use aoc::{ProcessInput, PuzzleInput};
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    iter::FromIterator,
};

type Output = usize;

register!(
    "input/day22.txt";
    (input: input!(process DeckInput)) -> Output {
        run1(input.0.clone(), input.1.clone());
        run2(input.0, input.1);
    }
);

#[derive(Debug, Clone)]
pub struct Deck {
    cards: [u8; 50],
    read: u8,
    write: u8,
    len: u8,
}

impl Deck {
    const fn new() -> Self {
        Self {
            cards: [0; 50],
            read: 0,
            write: 0,
            len: 0,
        }
    }

    const fn is_empty(&self) -> bool {
        self.len == 0
    }

    const fn len(&self) -> u8 {
        self.len
    }

    fn pop(&mut self) -> u8 {
        self.len -= 1;
        let pos = self.read;
        self.read = (pos + 1) % 50;
        self.cards[pos as usize]
    }

    fn pop_last(&mut self) -> u8 {
        self.len -= 1;
        let pos = (self.write + 49) % 50;
        self.write = pos;
        self.cards[pos as usize]
    }

    fn push(&mut self, c1: u8) {
        self.len += 1;
        let pos = self.write;
        self.cards[pos as usize] = c1;
        self.write = (pos + 1) % 50;
    }

    fn extend(&mut self, c1: u8, c2: u8) {
        self.len += 2;
        let pos = self.write;
        self.cards[pos as usize] = c1;
        let pos = (pos + 1) % 50;
        self.cards[pos as usize] = c2;
        self.write = (pos + 1) % 50;
    }

    fn slice(&self, len: u8) -> Self {
        Self {
            write: (self.read + len) % 50,
            len,
            ..*self
        }
    }
}

impl FromIterator<u8> for Deck {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        iter.into_iter().fold(Self::new(), |mut deck, card| {
            deck.push(card);
            deck
        })
    }
}

impl Iterator for Deck {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(self.pop())
        }
    }
}

impl DoubleEndedIterator for Deck {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(self.pop_last())
        }
    }
}

impl Hash for Deck {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len.hash(state);
        if self.write > self.read {
            Hash::hash_slice(&self.cards[self.read as usize..self.write as usize], state);
        } else {
            Hash::hash_slice(&self.cards[self.read as usize..], state);
            Hash::hash_slice(&self.cards[..self.write as usize], state);
        }
    }
}

fn run1(mut player1: Deck, mut player2: Deck) -> Output {
    while !player1.is_empty() && !player2.is_empty() {
        let p1 = player1.pop();
        let p2 = player2.pop();

        if p1 > p2 {
            player1.extend(p1, p2);
        } else {
            player2.extend(p2, p1);
        }
    }

    eval(if player1.is_empty() { player2 } else { player1 })
}

fn run2(mut player1: Deck, mut player2: Deck) -> Output {
    eval(if play_round(&mut player1, &mut player2) {
        player1
    } else {
        player2
    })
}

fn play_round(pl1: &mut Deck, pl2: &mut Deck) -> bool {
    let mut played = HashSet::new();
    while !pl1.is_empty() && !pl2.is_empty() {
        if !played.insert(hash(pl1, pl2)) {
            return true;
        }

        let p1 = pl1.pop();
        let p2 = pl2.pop();

        let p1_won = if pl1.len() >= p1 && pl2.len() >= p2 {
            let mut pl1r = pl1.slice(p1);
            let mut pl2r = pl2.slice(p2);
            play_round(&mut pl1r, &mut pl2r)
        } else {
            p1 > p2
        };

        if p1_won {
            pl1.extend(p1, p2);
        } else {
            pl2.extend(p2, p1);
        }
    }
    pl2.is_empty()
}

fn hash(p1: &Deck, p2: &Deck) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    p1.hash(&mut hasher);
    p2.hash(&mut hasher);
    hasher.finish()
}

fn eval(deck: Deck) -> Output {
    deck.rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * usize::from(c))
        .sum()
}

pub struct DeckInput;

impl ProcessInput for DeckInput {
    type In = input!(chunk str);

    type Out<'a> = (Deck, Deck);

    fn process(input: <Self::In as PuzzleInput>::Out<'_>) -> Self::Out<'_> {
        let mut players = input
            .into_iter()
            .map(|p| {
                p.into_iter()
                    .skip(1)
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect::<Deck>()
            })
            .collect::<Vec<_>>()
            .into_iter();

        let player1 = players.next().unwrap();
        let player2 = players.next().unwrap();
        (player1, player2)
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
        assert_eq!(res1, 29764);
        assert_eq!(res2, 32588);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(
            (306, 291),
            Solver::run_on(
                "
                Player 1:
                9
                2
                6
                3
                1

                Player 2:
                5
                8
                4
                7
                10
            ",
            )
        );
    }

    #[test]
    fn test_deck_pop() {
        let mut deck = (1..=50).collect::<Deck>();
        assert_eq!(50, deck.len());
        assert!(!deck.is_empty());

        for i in 1..=49 {
            assert_eq!(i, deck.pop());
            assert_eq!(50 - i, deck.len());
            assert!(!deck.is_empty());
        }

        assert_eq!(50, deck.pop());
        assert_eq!(0, deck.len());
        assert!(deck.is_empty());
    }

    #[test]
    fn test_deck_pop_last() {
        let mut deck = (1..=50).collect::<Deck>();
        assert_eq!(50, deck.len());
        assert!(!deck.is_empty());

        for i in 1..=49 {
            assert_eq!(50 - i + 1, deck.pop_last());
            assert_eq!(50 - i, deck.len());
            assert!(!deck.is_empty());
        }

        assert_eq!(1, deck.pop());
        assert_eq!(0, deck.len());
        assert!(deck.is_empty());
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
        b.iter(|| run1(input.0.clone(), input.1.clone()));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| run2(input.0.clone(), input.1.clone()));
    }
}
