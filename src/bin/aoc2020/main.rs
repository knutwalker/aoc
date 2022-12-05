#![feature(slice_take)]
#![feature(test)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::fallible_impl_from,
    clippy::iter_with_drain,
    clippy::missing_const_for_fn,
    clippy::similar_names,
    elided_lifetimes_in_paths
)]

#[macro_use]
extern crate aoc;
#[allow(unused_extern_crates)]
extern crate test;

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
