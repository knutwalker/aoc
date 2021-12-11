// use aoc::register2;
use std::ops::{Deref, DerefMut};

register!(
    "input/day5.txt";
    (input: input!(Seat)) -> u16 {
        max_seat_id(input.iter());
        find_seat(input);
    }
);

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Seat(u16);

impl From<String> for Seat {
    fn from(input: String) -> Self {
        Seat(
            input
                .bytes()
                .map(|c| match c {
                    b'F' | b'L' => 0,
                    b'B' | b'R' => 1,
                    x => unreachable!("not F, B, L, or R: {}", x),
                })
                .fold(0, |sum, digit| sum << 1 | digit),
        )
    }
}

impl Deref for Seat {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Seat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn max_seat_id<'a>(input: impl Iterator<Item = &'a Seat>) -> u16 {
    **input.max().unwrap()
}

fn find_seat(mut seats: Vec<Seat>) -> u16 {
    seats.sort_unstable();
    let (first, rest) = seats.split_first().unwrap();
    rest.iter()
        .scan(**first, |prev, &current| {
            let seat = *prev;
            let diff = *current - seat;
            *prev = *current;
            Some((seat, diff))
        })
        .find(|(_, diff)| *diff > 1)
        .map(|(seat, _)| seat + 1)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test_parse_seat() {
        assert_eq!(357, Seat::from(String::from("FBFBBFFRLR")).0);
        assert_eq!(567, Seat::from(String::from("BFFFBBFRRR")).0);
        assert_eq!(119, Seat::from(String::from("FFFBBBFRRR")).0);
        assert_eq!(820, Seat::from(String::from("BBFFBBFRLL")).0);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 848);
        assert_eq!(res2, 682);
    }
}
