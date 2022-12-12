#![feature(
    array_chunks,
    array_windows,
    control_flow_enum,
    drain_filter,
    get_many_mut,
    iter_array_chunks,
    iter_partition_in_place,
    portable_simd,
    slice_take,
    test
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
    clippy::fallible_impl_from,
    clippy::iter_with_drain,
    clippy::missing_const_for_fn,
    clippy::redundant_pub_crate,
    clippy::similar_names,
    elided_lifetimes_in_paths
)]

#[macro_use]
extern crate aoc;
#[cfg(test)]
extern crate test;

aoc_main!(
    1 => day1,
    2 => day2,
    3 => day3,
    4 => day4,
    5 => day5,
    6 => day6,
    7 => day7,
    8 => day8,
    9 => day9,
    10 => day10,
    11 => day11,
    12 => day12,
    // 13 => day13,
    // 14 => day14,
    // 15 => day15,
    // 16 => day16,
    // 17 => day17,
    // 18 => day18,
    // 19 => day19,
    // 20 => day20,
    // 21 => day21,
    // 22 => day22,
    // 23 => day23,
    // 24 => day24,
    // 25 => day25,
);
