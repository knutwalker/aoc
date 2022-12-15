use std::{
    cmp::Reverse,
    ops::{ControlFlow, RangeInclusive},
    result::Result,
};

use aoc::Parse;
use atoi::FromRadix10Signed;
use fxhash::FxHashSet;
use tap::Tap;

type Range = RangeInclusive<i64>;
type Output = i64;

register!(
    "input/day15.txt";
    (input: input!(Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    fn range_len_because_somehow_exact_iterator_is_not_implemented_for_i64_ranges(
        range: Range,
    ) -> i64 {
        range.end() - range.start() + 1
    }

    let target = if items.len() == 14 { 10 } else { 2_000_000 };

    let covered = Input::all_line_coverages(items, target);

    let beacons = Input::beacons_in_line(items, target)
        .filter(|x| covered.iter().any(|range| range.contains(x)))
        .collect::<FxHashSet<_>>()
        .len() as i64;

    covered
        .into_iter()
        .map(range_len_because_somehow_exact_iterator_is_not_implemented_for_i64_ranges)
        .sum::<Output>()
        - beacons
}

fn part2(items: &[Input]) -> Output {
    let max = if items.len() == 14 { 20 } else { 4_000_000 };

    for y in (0..=max).rev() {
        if let Some(gap) = Input::gap_in_line(items, y, 0..=max) {
            return gap * 4_000_000 + y;
        }
    }

    unreachable!("no solution")
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Input {
    sensor: Coord,
    beacon: Coord,
}

impl Input {
    fn beacons_in_line(items: &[Self], line: i64) -> impl Iterator<Item = i64> + '_ {
        items
            .iter()
            .filter_map(move |item| (item.beacon.y == line).then_some(item.beacon.x))
    }

    fn all_line_coverages(items: &[Self], line: i64) -> Vec<Range> {
        fn merge_range(first: &mut Range, second: Range) -> Option<Range> {
            if first.end() < second.start() {
                Some(std::mem::replace(first, second))
            } else if first.end() < second.end() {
                *first = *first.start()..=*second.end();
                None
            } else {
                None
            }
        }

        items
            .iter()
            .filter_map(|item| item.coverage_in_line(line))
            .collect::<Vec<_>>()
            .tap_mut(|c| c.sort_unstable_by_key(|range| (*range.start(), Reverse(*range.end()))))
            .into_iter()
            .chain(Some(i64::MAX..=i64::MAX))
            .scan(None::<Range>, |last, range| {
                Some({
                    match last {
                        Some(last) => merge_range(last, range),
                        None => std::mem::replace(last, Some(range)),
                    }
                })
            })
            .flatten()
            .collect()
    }

    fn gap_in_line(items: &[Self], line: i64, bound: Range) -> Option<i64> {
        let ranges = items
            .iter()
            .filter_map(|item| item.coverage_in_line(line))
            .chain(Self::beacons_in_line(items, line).map(|x| x..=x))
            .filter(|range| range.start() <= bound.end() && range.end() >= bound.start())
            .collect::<Vec<_>>()
            .tap_mut(|c| c.sort_unstable_by_key(|range| (*range.start(), Reverse(*range.end()))));

        let mut ranges = ranges.into_iter();
        let first = ranges.next().expect("line is not covered at all");

        let res = ranges.try_fold(*first.end(), |end, range| {
            let gap = end + 1;
            if gap < *range.start() && bound.contains(&gap) {
                ControlFlow::Break(Ok(gap))
            } else if end > *bound.end() {
                ControlFlow::Break(Err(end))
            } else {
                ControlFlow::Continue(end.max(*range.end()))
            }
        });

        res.break_value().and_then(Result::ok)
    }

    fn signal_span(&self) -> u64 {
        self.sensor.x.abs_diff(self.beacon.x) + self.sensor.y.abs_diff(self.beacon.y)
    }

    fn coverage_in_line(&self, line: i64) -> Option<Range> {
        let span = self.signal_span();
        let distance_from_line = self.sensor.y.abs_diff(line);

        span.checked_sub(distance_from_line).map(|dist| {
            self.sensor.x.saturating_sub_unsigned(dist)
                ..=self.sensor.x.saturating_add_unsigned(dist)
        })
    }
}

impl Parse for Input {
    type Out<'a> = Self;

    fn parse_from(input: &str) -> Self::Out<'_> {
        let input = &input[12..];
        let (sensor_x, used) = i64::from_radix_10_signed(input.as_bytes());
        let input = &input[used + 4..];
        let (sensor_y, used) = i64::from_radix_10_signed(input.as_bytes());
        let input = &input[used + 25..];
        let (beacon_x, used) = i64::from_radix_10_signed(input.as_bytes());
        let input = &input[used + 4..];
        let (beacon_y, used) = i64::from_radix_10_signed(input.as_bytes());
        let input = &input[used..];
        assert!(input.is_empty(), "Unexpected input: {input}");
        let sensor = Coord {
            x: sensor_x,
            y: sensor_y,
        };
        let beacon = Coord {
            x: beacon_x,
            y: beacon_y,
        };
        Self { sensor, beacon }
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
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 26);
        assert_eq!(res2, 56_000_011);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert!(res1 < 6_070_917, "{res1} is too large");
        assert_eq!(res1, 5_511_201);
        assert_eq!(res2, 11_318_723_411_840);
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
