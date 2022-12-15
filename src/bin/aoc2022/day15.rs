use std::{
    cmp::Reverse,
    ops::{ControlFlow, RangeInclusive},
    result::Result,
};

use aoc::{Parse, ProcessInput};
use atoi::FromRadix10Signed;
use fxhash::{FxHashMap, FxHashSet};
use tap::Tap;

type Int = i32;
type UInt = u32;
type Range = RangeInclusive<Int>;
type Output = i64;

register!(
    "input/day15.txt";
    (input: input!(process Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(input: &Input) -> Output {
    fn range_len_because_somehow_exact_iterator_is_not_implemented_for_i64_ranges(
        range: Range,
    ) -> Int {
        range.end() - range.start() + 1
    }

    let target = if input.is_example() { 10 } else { 2_000_000 };

    let covered = input.all_line_coverages(target);

    let beacons = input
        .beacons_in_line(target)
        .filter(|x| covered.iter().any(|range| range.contains(x)))
        .count() as Int;

    covered
        .into_iter()
        .map(range_len_because_somehow_exact_iterator_is_not_implemented_for_i64_ranges)
        .map(Output::from)
        .sum::<Output>()
        - Output::from(beacons)
}

fn part2(input: &Input) -> Output {
    let max = if input.is_example() { 20 } else { 4_000_000 };

    for y in (0..=max).rev() {
        if let Some(gap) = Input::gap_in_line(input, y, 0..=max) {
            return Output::from(gap) * 4_000_000 + Output::from(y);
        }
    }

    unreachable!("no solution")
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Sensor {
    x: Int,
    y: Int,
    span: UInt,
}

impl Sensor {
    fn coverage_in_line(&self, line: Int) -> Option<Range> {
        let distance_from_line = self.y.abs_diff(line);

        self.span
            .checked_sub(distance_from_line)
            .map(|dist| self.x.saturating_sub_unsigned(dist)..=self.x.saturating_add_unsigned(dist))
    }
}

pub struct Input {
    sensors: Vec<Sensor>,
    beacons: FxHashMap<Int, &'static [Int]>,
}

impl Input {
    fn is_example(&self) -> bool {
        self.sensors.len() == 14
    }

    fn beacons_in_line(&self, line: Int) -> impl Iterator<Item = Int> + '_ {
        self.beacons
            .get(&line)
            .copied()
            .unwrap_or_default()
            .iter()
            .copied()
    }

    fn all_line_coverages(&self, line: Int) -> Vec<Range> {
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

        self.sensors
            .iter()
            .filter_map(|item| item.coverage_in_line(line))
            .collect::<Vec<_>>()
            .tap_mut(|c| c.sort_unstable_by_key(|range| (*range.start(), Reverse(*range.end()))))
            .into_iter()
            .chain(Some(Int::MAX..=Int::MAX))
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

    fn gap_in_line(&self, line: Int, bound: Range) -> Option<Int> {
        let ranges = self
            .sensors
            .iter()
            .filter_map(|item| item.coverage_in_line(line))
            .chain(self.beacons_in_line(line).map(|x| x..=x))
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
}

impl ProcessInput for Input {
    type In = input!(InputLine);

    type Out<'a> = Self;

    fn process(input: <Self::In as aoc::PuzzleInput>::Out<'_>) -> Self::Out<'_> {
        let mut beacons = FxHashMap::default();
        for beacon in &input {
            beacons
                .entry(beacon.beacon.1)
                .or_insert_with(FxHashSet::<Int>::default)
                .insert(beacon.beacon.0);
        }
        let beacons = beacons
            .into_iter()
            .map(|(y, xs)| (y, &*xs.into_iter().collect::<Vec<_>>().leak()))
            .collect();

        let sensors = input
            .into_iter()
            .map(|line| Sensor {
                x: line.sensor.0,
                y: line.sensor.1,
                span: line.signal_span(),
            })
            .collect();

        Self { sensors, beacons }
    }
}

pub struct InputLine {
    sensor: (Int, Int),
    beacon: (Int, Int),
}

impl InputLine {
    fn signal_span(&self) -> UInt {
        self.sensor.0.abs_diff(self.beacon.0) + self.sensor.1.abs_diff(self.beacon.1)
    }
}

impl Parse for InputLine {
    type Out<'a> = Self;

    fn parse_from(input: &str) -> Self::Out<'_> {
        let input = &input[12..];
        let (sensor_x, used) = Int::from_radix_10_signed(input.as_bytes());
        let input = &input[used + 4..];
        let (sensor_y, used) = Int::from_radix_10_signed(input.as_bytes());
        let input = &input[used + 25..];
        let (beacon_x, used) = Int::from_radix_10_signed(input.as_bytes());
        let input = &input[used + 4..];
        let (beacon_y, used) = Int::from_radix_10_signed(input.as_bytes());
        let input = &input[used..];
        assert!(input.is_empty(), "Unexpected input: {input}");
        let sensor = (sensor_x, sensor_y);
        let beacon = (beacon_x, beacon_y);
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
