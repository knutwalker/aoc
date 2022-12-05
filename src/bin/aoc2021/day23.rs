use aoc::PuzzleInput;
use fxhash::FxHashMap;
use std::{
    fmt::{Display, Write},
    hash::Hash,
};

type Output = u32;

register!(
    "input/day23.txt";
    (input: Board) -> Output {
        part1(input.0);
        part2(input.1);
    }
);

fn part1(board: Board) -> Output {
    poop!("{}", board);
    board.run()
}

fn part2(board: Board) -> Output {
    poop!("{}", board);
    board.run()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]

struct Hallway(u32);

impl Hallway {
    const SIZE: usize = 4;
    const MASK: u32 = 0b1111;

    const EMPTY: u32 = 0b0000;

    const A: u32 = 0b0001;
    const B: u32 = 0b0010;
    const C: u32 = 0b0100;
    const D: u32 = 0b1000;

    const C1: u32 = 24;
    const C2: u32 = 20;
    const C3: u32 = 28;
    const C4: u32 = 16;
    const C5: u32 = 28;
    const C6: u32 = 12;
    const C7: u32 = 28;
    const C8: u32 = 8;
    const C9: u32 = 28;
    const C10: u32 = 4;
    const C11: u32 = 0;

    const PATH_LENGTHS: [u32; 7] = [
        // column 11: _ _ 8 _ 6 _ 4 _ 2 _ x
        0b0_0010_0100_0110_1000,
        // column 10: _ _ 7 _ 5 _ 3 _ 1 x _
        0b0_0001_0011_0101_0111,
        // column  8: _ _ 5 _ 3 _ 1 x 1 _ _
        0b0_0001_0001_0011_0101,
        // column  6: _ _ 3 _ 1 x 1 _ 3 _ _
        0b0_0011_0001_0001_0011,
        // column  4: _ _ 1 x 1 _ 3 _ 5 _ _
        0b0_0101_0011_0001_0001,
        // column  2: _ x 1 _ 3 _ 5 _ 7 _ _
        0b0_0111_0101_0011_0001,
        // column  1: x _ 2 _ 4 _ 6 _ 8 _ _
        0b0_1000_0110_0100_0010,
    ];

    #[inline]
    const fn color(self, column: u32) -> u32 {
        (self.0 >> column) & Self::MASK
    }

    #[inline]
    const fn set(self, column: u32, color: u32) -> Self {
        Self(self.0 & !(Self::MASK << column) | (color << column))
    }

    #[inline]
    const fn is_empty(self) -> bool {
        self.0 == 0
    }

    const fn path_from(column: u32, color: u32) -> u32 {
        // A -> 0, B -> 4, C -> 8, D -> 12
        let offset = color.trailing_zeros() << 2;
        // 20 -> A -> 16 -> B -> 12 -> C -> 8 -> D -> 4
        let mut from_left = Self::C2 - offset;

        let (mut width, from_right) = column.overflowing_sub(from_left);
        if from_right {
            width = from_left - column - 4;
            from_left = column + 4;
        }

        ((1 << width) - 1) << from_left
    }

    #[inline]
    fn can_enter(self, column: u32, color: u32) -> bool {
        static PATHS: [u32; 7 * 4] = [
            Hallway::path_from(Hallway::C11, Hallway::A),
            Hallway::path_from(Hallway::C11, Hallway::B),
            Hallway::path_from(Hallway::C11, Hallway::C),
            Hallway::path_from(Hallway::C11, Hallway::D),
            Hallway::path_from(Hallway::C10, Hallway::A),
            Hallway::path_from(Hallway::C10, Hallway::B),
            Hallway::path_from(Hallway::C10, Hallway::C),
            Hallway::path_from(Hallway::C10, Hallway::D),
            Hallway::path_from(Hallway::C8, Hallway::A),
            Hallway::path_from(Hallway::C8, Hallway::B),
            Hallway::path_from(Hallway::C8, Hallway::C),
            Hallway::path_from(Hallway::C8, Hallway::D),
            Hallway::path_from(Hallway::C6, Hallway::A),
            Hallway::path_from(Hallway::C6, Hallway::B),
            Hallway::path_from(Hallway::C6, Hallway::C),
            Hallway::path_from(Hallway::C6, Hallway::D),
            Hallway::path_from(Hallway::C4, Hallway::A),
            Hallway::path_from(Hallway::C4, Hallway::B),
            Hallway::path_from(Hallway::C4, Hallway::C),
            Hallway::path_from(Hallway::C4, Hallway::D),
            Hallway::path_from(Hallway::C2, Hallway::A),
            Hallway::path_from(Hallway::C2, Hallway::B),
            Hallway::path_from(Hallway::C2, Hallway::C),
            Hallway::path_from(Hallway::C2, Hallway::D),
            Hallway::path_from(Hallway::C1, Hallway::A),
            Hallway::path_from(Hallway::C1, Hallway::B),
            Hallway::path_from(Hallway::C1, Hallway::C),
            Hallway::path_from(Hallway::C1, Hallway::D),
        ];

        self.0 & PATHS[(column + color.trailing_zeros()) as usize] == 0
    }

    const fn path_length(column: u32, color: u32) -> u32 {
        // A -> 1111 << 0, B -> 1111 << 4, C -> 1111 << 8, D -> 1111 << 12
        (Self::PATH_LENGTHS[(column >> 2) as usize] >> (color.trailing_zeros() << 2)) & 0b1111
    }

    fn columns() -> std::iter::StepBy<std::iter::Rev<std::ops::RangeInclusive<u32>>> {
        (Self::C11..=Self::C1).rev().step_by(Self::SIZE)
    }

    const fn decode(self, column: u32) -> char {
        match self.color(column) {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            _ => '.',
        }
    }
}

impl Display for Hallway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('#')?;
        for c in [
            Self::C1,
            Self::C2,
            Self::C3,
            Self::C4,
            Self::C5,
            Self::C6,
            Self::C7,
            Self::C8,
            Self::C9,
            Self::C10,
            Self::C11,
        ] {
            f.write_char(self.decode(c))?;
        }
        f.write_char('#')
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct Room(u16);

impl Room {
    const SIZE: usize = 4;

    const MASK: u16 = 0b1111;

    const EMPTY: u16 = 0b0000;

    const A: u16 = 0b0001;
    const B: u16 = 0b0010;
    const C: u16 = 0b0100;
    const D: u16 = 0b1000;

    const S1: u32 = 12;
    const S2: u32 = 8;
    const S3: u32 = 4;
    const S4: u32 = 0;

    #[inline]
    const fn full(color: u16) -> u16 {
        color << Self::S1 | color << Self::S2 | color << Self::S3 | color << Self::S4
    }

    #[cfg(test)]
    const fn from(s1: u16, s2: u16, s3: u16, s4: u16) -> Self {
        Self(s1 << Self::S1 | s2 << Self::S2 | s3 << Self::S3 | s4 << Self::S4)
    }

    const fn new(color: u16) -> Self {
        Self(Self::full(color))
    }

    #[inline]
    const fn color(self, slot: u32) -> u16 {
        (self.0 >> slot) & Self::MASK
    }

    #[inline]
    const fn set(self, slot: u32, color: u16) -> Self {
        Self(self.0 & !(Self::MASK << slot) | (color << slot))
    }

    #[inline]
    const fn can_enter(self, color: u16) -> bool {
        (self.0 & Self::full(color) == self.0) && self.0.leading_zeros() >= 4
    }

    #[inline]
    const fn can_leave(self, color: u16) -> bool {
        let full = Self::full(color);
        // x | full -> set all empty to color
        // x ^ full -> set all color to empty
        // (x | full) ^ full -> set all (color or empty) to empty
        // if there is anything remaing, it's a item that doesn't belong
        (self.0 | full) ^ full != 0
    }

    const fn top_slot_in_use(self) -> u32 {
        Self::S1.wrapping_sub(self.0.leading_zeros() & !0b11)
    }

    const fn free_slot(self, color: u16) -> u32 {
        // assume a 'home' room, e.g. E E x x, not another mix
        // x ^ full -> toggle empty<>color slots, e.g. x x E E
        // count number of trailing slots, that are now free
        (self.0 ^ Self::full(color)).trailing_zeros() - color.trailing_zeros()
    }

    const fn is_done(self, color: u16) -> bool {
        self.0 == Self::full(color)
    }

    fn slots() -> impl Iterator<Item = u32> {
        (Self::S4..=Self::S1).rev().step_by(Self::SIZE)
    }

    const fn decode(self, slot: u32) -> char {
        match self.color(slot) {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            _ => '.',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct Rooms([Room; 4]);

impl Rooms {
    const A: usize = 0;
    const B: usize = 1;
    const C: usize = 2;
    const D: usize = 3;

    const ROOM_COLORS: [u16; 4] = [Room::A, Room::B, Room::C, Room::D];

    #[inline]
    const fn room(self, room: usize) -> Room {
        self.0[room]
    }

    // #[inline]
    // const fn color(self, room: usize, slot: u32) -> u16 {
    //     self.room(room).color(slot)
    // }

    #[inline]
    const fn set(self, slot: u32, color: u32) -> Self {
        self.set_full(color.trailing_zeros() as usize, slot, color as _)
    }

    #[inline]
    const fn set_full(mut self, room: usize, slot: u32, color: u16) -> Self {
        self.0[room] = self.room(room).set(slot, color);
        self
    }

    const fn is_done(self) -> bool {
        self.room(Self::A).is_done(Room::A)
            && self.room(Self::B).is_done(Room::B)
            && self.room(Self::C).is_done(Room::C)
            && self.room(Self::D).is_done(Room::D)
        // Self::rooms().all(|r| self.room(r).is_done(Self::ROOM_COLORS[r]))
    }

    const fn can_enter(self, color: u32) -> bool {
        self.room(color.trailing_zeros() as usize)
            .can_enter(color as _)
    }

    // const fn can_leave(self, color: u32) -> bool {
    //     self.room(color.trailing_zeros() as usize)
    //         .can_leave(color as _)
    // }

    const fn free_slot(self, color: u32) -> u32 {
        self.room(color.trailing_zeros() as usize)
            .free_slot(color as _)
    }

    const fn rooms() -> std::ops::RangeInclusive<usize> {
        Self::A..=Self::D
    }

    const fn decode(self, room: usize, slot: u32) -> char {
        self.room(room).decode(slot)
    }
}

impl Default for Rooms {
    fn default() -> Self {
        Self([
            Room::new(Room::A),
            Room::new(Room::B),
            Room::new(Room::C),
            Room::new(Room::D),
        ])
    }
}

impl Display for Rooms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for slot in Room::slots() {
            writeln!(
                f,
                "{wall}#{a}#{b}#{c}#{d}#{wall}",
                a = self.decode(Self::A, slot),
                b = self.decode(Self::B, slot),
                c = self.decode(Self::C, slot),
                d = self.decode(Self::D, slot),
                wall = if slot == Room::S1 { "##" } else { "  " }
            )?;
        }
        Ok(())
    }
}

trait Score: Sized + Clone {
    fn win(board: Board) -> Self;

    fn loss() -> Self;

    fn try_finish(self, board: Board, cost: u32) -> Option<Self>;

    fn score(&self) -> u32;

    fn present_final_score(self) -> u32;
}

impl Score for u32 {
    fn win(_board: Board) -> Self {
        0
    }

    fn loss() -> Self {
        Self::MAX
    }

    fn try_finish(self, _board: Board, cost: u32) -> Option<Self> {
        let total = self.saturating_add(cost);
        (total < Self::MAX).then_some(total)
    }

    fn score(&self) -> u32 {
        *self
    }

    fn present_final_score(self) -> u32 {
        self
    }
}

impl Score for Scores {
    fn win(board: Board) -> Self {
        Self::Win(0, vec![(board, 0)])
    }

    fn loss() -> Self {
        Self::DeadEnd
    }

    fn try_finish(self, board: Board, cost: u32) -> Option<Self> {
        match self {
            Self::DeadEnd => None,
            Self::Win(score, mut path) => {
                path.push((board, cost));
                Some(Self::Win(score + cost, path))
            }
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::DeadEnd => u32::MAX,
            Self::Win(score, _) => *score,
        }
    }

    fn present_final_score(self) -> u32 {
        match self {
            Self::DeadEnd => {
                println!("no solution :/");
                u32::MAX
            }
            Self::Win(score, board) => {
                let mut total = 0;
                for (board, score) in board.into_iter().rev() {
                    total += score;
                    println!("Score: {score} == Total: {total}");
                    println!("{board}");
                }
                println!(" -- =================== -- ");
                score
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Scores {
    DeadEnd,
    Win(u32, Vec<(Board, u32)>),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Board {
    hallway: Hallway,
    rooms: Rooms,
}

impl Board {
    const fn is_done(self) -> bool {
        self.hallway.is_empty() && self.rooms.is_done()
    }

    const fn cost(column: u32, room_color: u32, slot: u32, color: u32) -> u32 {
        let dist = ((16 - slot) >> 2) + Hallway::path_length(column, room_color);
        let score = 10_u32.pow(color.trailing_zeros());
        dist * score
    }

    fn try_move_from_hallway(self) -> Option<(Self, u32)> {
        for col in Hallway::columns() {
            let c = self.hallway.color(col);
            if c != Hallway::EMPTY && self.rooms.can_enter(c) && self.hallway.can_enter(col, c) {
                let slot = self.rooms.free_slot(c);
                let cost = Self::cost(col, c, slot, c);

                let rooms = self.rooms.set(slot, c);
                let hallway = self.hallway.set(col, Hallway::EMPTY);
                let board = Self { hallway, rooms };
                return Some((board, cost));
            }
        }

        None
    }

    #[cfg(debug_assertions)]
    fn run(self) -> u32 {
        self.play_all::<Scores>()
    }

    #[cfg(not(debug_assertions))]
    fn run(self) -> u32 {
        self.play_all::<u32>()
    }

    fn play_all<S: Score>(self) -> u32 {
        let mut cache = FxHashMap::default();
        self.play::<S>(&mut cache).present_final_score()
    }

    fn play<S: Score>(self, cache: &mut FxHashMap<Self, S>) -> S {
        if self.is_done() {
            return S::win(self);
        }
        if let Some(result) = cache.get(&self) {
            return result.clone();
        }

        if let Some((next, cost)) = self.try_move_from_hallway() {
            let score = next.play(cache);
            if let Some(score) = score.try_finish(next, cost) {
                return score;
            }
        }

        let score = self
            .valid_moves()
            .filter_map(|(next, cost)| next.play(cache).try_finish(next, cost))
            .min_by_key(Score::score)
            .unwrap_or_else(S::loss);

        cache.insert(self, score.clone());
        score
    }

    fn valid_moves(self) -> impl Iterator<Item = (Self, u32)> {
        self.room_moves().flatten()
    }

    const fn room_moves(self) -> RoomMoves {
        RoomMoves {
            board: self,
            rooms: Rooms::rooms(),
        }
    }
}

struct RoomMoves {
    board: Board,
    rooms: std::ops::RangeInclusive<usize>,
}

impl Iterator for RoomMoves {
    type Item = RoomToHallwayMoves;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let r = self.rooms.next()?;
            let room = self.board.rooms.room(r);
            let room_color = Rooms::ROOM_COLORS[r];
            if room.can_leave(room_color) {
                let slot = room.top_slot_in_use();
                let color = room.color(slot);

                return Some(RoomToHallwayMoves {
                    board: self.board,
                    cols: Hallway::columns(),
                    room: r,
                    room_color: u32::from(room_color),
                    room_slot: slot,
                    color: u32::from(color),
                });
            }
        }
    }
}

type Steps<T> = std::iter::StepBy<std::iter::Rev<std::ops::RangeInclusive<T>>>;

struct RoomToHallwayMoves {
    board: Board,
    cols: Steps<u32>,
    room: usize,
    room_color: u32,
    room_slot: u32,
    color: u32,
}

impl Iterator for RoomToHallwayMoves {
    type Item = (Board, u32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let col = self.cols.next()?;
            if self.board.hallway.color(col) == Hallway::EMPTY
                && self.board.hallway.can_enter(col, self.room_color)
            {
                let cost = Board::cost(col, self.room_color, self.room_slot, self.color);

                let rooms = self
                    .board
                    .rooms
                    .set_full(self.room, self.room_slot, Room::EMPTY);
                let hallway = self.board.hallway.set(col, self.color);
                let board = Board { hallway, rooms };
                return Some((board, cost));
            }
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("#############\n")?;
        self.hallway.fmt(f)?;
        f.write_char('\n')?;
        self.rooms.fmt(f)
    }
}

impl FromIterator<u16> for Board {
    fn from_iter<T: IntoIterator<Item = u16>>(iter: T) -> Self {
        let room_slots = Room::slots().flat_map(|s| Rooms::rooms().map(move |r| (r, s)));
        let rooms = room_slots
            .zip(iter)
            .fold(Rooms::default(), |rooms, ((room, slot), color)| {
                rooms.set_full(room, slot, color)
            });

        Self {
            hallway: Hallway::default(),
            rooms,
        }
    }
}

impl PuzzleInput for Board {
    type Out<'a> = (Self, Self);

    fn from_input(input: &str) -> Self::Out<'_> {
        let colors = input.bytes().filter_map(|b| match b {
            b'A' => Some(Room::A),
            b'B' => Some(Room::B),
            b'C' => Some(Room::C),
            b'D' => Some(Room::D),
            _ => None,
        });

        let colors2 = colors
            .clone()
            .take(4)
            .chain([
                Room::D,
                Room::C,
                Room::B,
                Room::A,
                Room::D,
                Room::B,
                Room::A,
                Room::C,
            ])
            .chain(colors.clone().skip(4));

        (colors.collect(), colors2.collect())
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
        #############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 12521);
        assert_eq!(res2, 44169);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 11332);
        assert_eq!(res2, 49936);
    }

    #[test]
    fn path_from() {
        // 1, A -> column=24, fli-20, width=4
        assert_eq!(
            Hallway::path_from(Hallway::C1, Hallway::A),
            0b_0000_1111_0000_0000_0000_0000_0000
        );

        // 1, D -> column=24, fli-8,  width=16
        assert_eq!(
            Hallway::path_from(Hallway::C1, Hallway::D),
            0b_0000_1111_1111_1111_1111_0000_0000
        );

        // 4, B -> column=16, fli-16, width=0
        assert_eq!(
            Hallway::path_from(Hallway::C4, Hallway::B),
            0b_0000_0000_0000_0000_0000_0000_0000
        );

        //  6, A -> column=12, fri-16, width=4
        assert_eq!(
            Hallway::path_from(Hallway::C6, Hallway::A),
            0b_0000_0000_1111_0000_0000_0000_0000
        );

        // 11, B -> column=0,  fri-12, width=12
        assert_eq!(
            Hallway::path_from(Hallway::C11, Hallway::B),
            0b_0000_0000_0000_1111_1111_1111_0000
        );

        // 10, D -> column=4,  fri-4,  width=0
        assert_eq!(
            Hallway::path_from(Hallway::C10, Hallway::D),
            0b_0000_0000_0000_0000_0000_0000_0000
        );
    }

    #[test]
    fn play_example() {
        let mut board: Board = [
            Room::B,
            Room::C,
            Room::B,
            Room::D,
            Room::A,
            Room::D,
            Room::C,
            Room::A,
        ]
        .into_iter()
        .collect();

        let mut total = 0;
        for m in [16, 2, 2, 2, 0, 0] {
            let (next, score) = board.valid_moves().nth(m).unwrap();
            total += score;
            println!("score: {score:>8}, total: {total:>8}");
            println!("{next}");

            board = next;
            while let Some((next, score)) = board.try_move_from_hallway() {
                total += score;
                println!("score: {score:>8}, total: {total:>8}  move from hallway");
                println!("{next}");
                board = next;
            }
        }

        assert!(board.is_done());
        assert_eq!(board.try_move_from_hallway(), None);
        assert_eq!(board.valid_moves().count(), 0);

        // println!("=========================================");
        // println!("=========================================");
        // for (idx, (mov, cost)) in board.valid_moves().enumerate() {
        //     println!("cost = {} index = {}", cost, idx);
        //     println!("{}", mov);
        // }
    }

    #[test]
    fn can_enter() {
        let empty = Room::EMPTY;
        let a = Room::A;
        let b = Room::B;
        let c = Room::C;
        let d = Room::D;

        assert!(Room::from(empty, empty, empty, empty).can_enter(a));
        assert!(Room::from(empty, empty, empty, a).can_enter(a));
        assert!(Room::from(empty, empty, a, a).can_enter(a));
        assert!(Room::from(empty, a, a, a).can_enter(a));
        assert!(!Room::from(a, a, a, a).can_enter(a));
        assert!(!Room::from(empty, empty, empty, a).can_enter(b));
        assert!(!Room::from(empty, empty, empty, a).can_enter(c));
        assert!(!Room::from(empty, empty, empty, a).can_enter(d));
        assert!(Room::from(empty, d, d, d).can_enter(d));
    }

    #[test]
    fn can_leave() {
        let empty = Room::EMPTY;
        let a = Room::A;
        let b = Room::B;
        let c = Room::C;
        let d = Room::D;

        assert!(!Room::from(empty, empty, empty, empty).can_leave(a));
        assert!(!Room::from(empty, empty, empty, a).can_leave(a));
        assert!(!Room::from(empty, empty, a, a).can_leave(a));
        assert!(!Room::from(empty, a, a, a).can_leave(a));
        assert!(!Room::from(a, a, a, a).can_leave(a));
        assert!(Room::from(a, b, c, d).can_leave(a));
        assert!(Room::from(a, b, c, d).can_leave(b));
        assert!(Room::from(a, b, c, d).can_leave(c));
        assert!(Room::from(a, b, c, d).can_leave(d));
        assert!(Room::from(empty, empty, empty, a).can_leave(b));
        assert!(Room::from(empty, empty, empty, a).can_leave(c));
        assert!(Room::from(empty, empty, empty, a).can_leave(d));
    }

    #[test]
    fn top_slot_in_use() {
        let x = Room::EMPTY;
        let a = Room::A;

        assert_eq!(
            Room::from(x, x, x, x).top_slot_in_use(),
            0_u32.wrapping_sub(4)
        );
        assert_eq!(Room::from(x, x, x, a).top_slot_in_use(), Room::S4);
        assert_eq!(Room::from(x, x, a, a).top_slot_in_use(), Room::S3);
        assert_eq!(Room::from(x, a, a, a).top_slot_in_use(), Room::S2);
        assert_eq!(Room::from(a, a, a, a).top_slot_in_use(), Room::S1);
    }

    #[test]
    fn free_slot() {
        let x = Room::EMPTY;
        let a = Room::A;

        assert_eq!(Room::from(x, x, a, a).free_slot(a), Room::S2);
        assert_eq!(Room::from(x, x, x, x).free_slot(a), Room::S4);
        assert_eq!(Room::from(x, a, x, x).free_slot(a), Room::S4);
        assert_eq!(Room::from(a, a, a, a).free_slot(a), 16);
    }

    #[test]
    #[cfg(not(debug_assertions))] // release mode only, otherwise it will take too long
    fn test_correctness() {
        struct Case {
            input: String,
            top: String,
            bottom: String,
            p1: Output,
            p2: Output,
        }

        let cases = (include_str!("d23-tests.txt"))
            .lines()
            .map(|l| {
                let mut l = l.split_ascii_whitespace();
                let top = l.next().unwrap();
                let bottom = l.next().unwrap();
                let input = format!(
                    "#############\n#...........#\n###{}#{}#{}#{}###\n  #{}#{}#{}#{}#\n  #########\n",
                    char::from(top.as_bytes()[0]),
                    char::from(top.as_bytes()[1]),
                    char::from(top.as_bytes()[2]),
                    char::from(top.as_bytes()[3]),
                    char::from(bottom.as_bytes()[0]),
                    char::from(bottom.as_bytes()[1]),
                    char::from(bottom.as_bytes()[2]),
                    char::from(bottom.as_bytes()[3]),
                );
                Case {
                    input,
                    top: top.to_string(),
                    bottom: bottom.to_string(),
                    p1: l.next().unwrap().parse().unwrap(),
                    p2: l.next().unwrap().parse().unwrap(),
                }
            })
            .collect::<Vec<_>>();

        let t0 = std::time::Instant::now();
        let mut n_fail = 0;
        for Case {
            input: s,
            top,
            bottom,
            p1: expected1,
            p2: expected2,
        } in &cases
        {
            let (answer1, answer2) = Solver::run_on(s);
            if answer1 != *expected1 || answer2 != *expected2 {
                println!(
                    "Test fail: {} {}: answer1={} answer2={} (expected1={} expected2={})",
                    top, bottom, answer1, answer2, expected1, expected2
                );
                n_fail += 1;
            }
        }
        let t1 = std::time::Instant::now();
        let took = t1 - t0;
        println!(
            "Total time: {:?}. Per test: {:?}",
            took,
            took / cases.len().try_into().unwrap()
        );
        if n_fail == 0 {
            println!("All {} tests have passed.", cases.len());
        } else {
            println!("{} failed out of {} tests.", n_fail, cases.len());
        }
        assert_eq!(n_fail, 0);
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
        b.iter(|| part1(input.0));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(input.1));
    }
}
