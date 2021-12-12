#![feature(
    array_windows,
    bool_to_option,
    control_flow_enum,
    drain_filter,
    iter_partition_in_place,
    let_else
)]
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
