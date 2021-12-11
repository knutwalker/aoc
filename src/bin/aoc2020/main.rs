#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms
)]
#![allow(
    clippy::missing_const_for_fn,
    clippy::redundant_pub_crate,
    unused_variables
)]

#[macro_use]
extern crate aoc;

aoc_main!(
    1 => day01,
    2 => day02,
    3 => day03,
    4 => day04,
    5 => day05,
    6 => day06,
    7 => day07,
    8 => day08,
    9 => day09,
    10 => day10,
    11 => day11,
    12 => day12,
    13 => day13,
    14 => day14,
    15 => day15,
    16 => day16,
    17 => day17,
    18 => day18,
    19 => day19,
    20 => day20,
    21 => day21,
    22 => day22,
    23 => day23,
    24 => day24,
    25 => day25,
);

// //
// #[macro_use]
// extern crate aoc;

// use aoc::{PuzzleSolution, Solution};

// mod day01;
// mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

// fn main() {
//     for day in std::env::args()
//         .skip(1)
//         .filter_map(|s| s.parse::<u8>().ok())
//     {
//         let PuzzleSolution {
//             part1,
//             part2,
//             timings,
//         } = match day {
//             1 => day01::Solver::solve(),
//             2 => day02::Solver::solve(),
//             3 => day03::Solver::solve(),
//             4 => day04::Solver::solve(),
//             5 => day05::Solver::solve(),
//             6 => day06::Solver::solve(),
//             7 => day07::Solver::solve(),
//             8 => day08::Solver::solve(),
//             9 => day09::Solver::solve(),
//             10 => day10::Solver::solve(),
//             11 => day11::Solver::solve(),
//             12 => day12::Solver::solve(),
//             13 => day13::Solver::solve(),
//             14 => day14::Solver::solve(),
//             15 => day15::Solver::solve(),
//             16 => day16::Solver::solve(),
//             17 => day17::Solver::solve(),
//             18 => day18::Solver::solve(),
//             19 => day19::Solver::solve(),
//             20 => day20::Solver::solve(),
//             21 => day21::Solver::solve(),
//             22 => day22::Solver::solve(),
//             23 => day23::Solver::solve(),
//             24 => day24::Solver::solve(),
//             25 => day25::Solver::solve(),
//             x => unimplemented!("Day {} is not yet implemented", x),
//         };

//         if let Some((time1, time2)) = timings {
//             println!(
//                 "Day {:02} Part 1:\t{} (took {})",
//                 day,
//                 part1,
//                 humantime::format_duration(time1)
//             );
//             println!(
//                 "Day {:02} Part 2:\t{} (took {})",
//                 day,
//                 part2,
//                 humantime::format_duration(time2)
//             );
//         } else {
//             println!("Day {:02} Part 1:\t{}", day, part1);
//             println!("Day {:02} Part 2:\t{}", day, part2);
//         }
//     }
// }
