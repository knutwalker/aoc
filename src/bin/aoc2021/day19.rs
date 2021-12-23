use aoc::{lines, PuzzleInput};
use const_combinations::SliceExt;
use fxhash::{FxBuildHasher, FxHashSet};
use std::{
    num::ParseIntError,
    ops::{Add, Sub},
    str::FromStr,
};
use tap::Tap;

type Output = usize;

register!(
    "input/day19.txt";
    (input: Map) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &Map) -> Output {
    items.beacons.len()
}

fn part2(items: &Map) -> Output {
    items
        .scanners
        .permutations()
        .map(|[c1, c2]| c1.distance(*c2))
        .max()
        .unwrap() as _
}

pub struct Map {
    scanners: Vec<Coord>,
    beacons: FxHashSet<Coord>,
}

impl PuzzleInput for Map {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut unmapped_scanners = input
            .split("\n\n")
            .map(|scanner| {
                let mut scanner = lines(scanner);
                scanner.next();

                let beacons = scanner.flat_map(str::parse::<Coord>).collect::<Vec<_>>();

                let beacons = beacons
                    .iter()
                    .map(|beacon| {
                        let diffs = beacons
                            .iter()
                            .filter(|other| beacon != *other)
                            .map(|other| beacon.norm_abs(*other))
                            .collect::<FxHashSet<_>>();
                        (*beacon, diffs)
                    })
                    .collect();

                Scanner { beacons }
            })
            .collect::<Vec<_>>();

        let mut scanners = Vec::with_capacity(unmapped_scanners.len());
        scanners.push(Coord([0, 0, 0]));

        let mut beacons =
            FxHashSet::with_capacity_and_hasher(scanners.capacity() * 20, FxBuildHasher::default());

        let mut root = unmapped_scanners.swap_remove(0);

        let mut frontier = Vec::with_capacity(144);
        frontier.append(&mut root.beacons);

        let mut next_frontier = Vec::with_capacity(144);

        while !unmapped_scanners.is_empty() {
            let mut idx = 0;
            while idx < unmapped_scanners.len() {
                let matches =
                    Scanner::find_matching_beacons(&frontier, &unmapped_scanners[idx].beacons);
                match Scanner::identify_center(&matches) {
                    Some(center) => {
                        let mut scanner = unmapped_scanners.swap_remove(idx);
                        scanner.realign_to(&center);
                        next_frontier.append(&mut scanner.beacons);
                        scanners.push(center.center);
                    }
                    None => {
                        idx += 1;
                    }
                }
            }

            beacons.extend(frontier.drain(..).map(|(beacon, _)| beacon));
            frontier.append(&mut next_frontier);
        }

        beacons.extend(frontier.drain(..).map(|(beacon, _)| beacon));

        Self { scanners, beacons }
    }
}

type Beacon = (Coord, FxHashSet<[u16; 3]>);

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn find_matching_beacons(lhs: &[Beacon], other: &[Beacon]) -> Vec<(Coord, Coord)> {
        lhs.iter()
            .flat_map(|(b1, diff1)| {
                other.iter().filter_map(|(b2, diff2)| {
                    (diff1.intersection(diff2).count() >= 11).then(|| (*b1, *b2))
                })
            })
            .collect()
    }

    fn identify_center(matches: &[(Coord, Coord)]) -> Option<Center> {
        matches
            .combinations()
            .find_map(|[(c1_left, c1_right), (c2_left, c2_right)]| {
                let left @ Coord([dx, dy, dz]) = *c2_left - *c1_left;

                (dx != dy && dx != dz).then(|| {
                    let Coord(right) = *c2_right - *c1_right;

                    let mapping = right.map(|v| {
                        if v == dx {
                            (Coord::X, 1)
                        } else if -v == dx {
                            (Coord::X, -1)
                        } else if v == dy {
                            (Coord::Y, 1)
                        } else if -v == dy {
                            (Coord::Y, -1)
                        } else if v == dz {
                            (Coord::Z, 1)
                        } else if -v == dz {
                            (Coord::Z, -1)
                        } else {
                            panic!(
                                "Unmatched diff: {}, available are {}, {}, {}",
                                v, dx, dy, dz
                            );
                        }
                    });

                    let to = c1_right.translate(mapping);
                    let center = to - *c1_left;

                    Center { mapping, center }
                })
            })
    }

    fn realign_to(&mut self, center: &Center) {
        for (beacon, _) in &mut self.beacons {
            let translated = beacon.translate(center.mapping);
            let moved = center.center + translated;
            *beacon = moved;
        }
    }
}

type Mapping = [(usize, i16); 3];

#[derive(Copy, Clone, Debug)]
struct Center {
    mapping: Mapping,
    center: Coord,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord([i16; 3]);

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let dx = rhs.0[Self::X] + self.0[Self::X];
        let dy = rhs.0[Self::Y] + self.0[Self::Y];
        let dz = rhs.0[Self::Z] + self.0[Self::Z];
        Self([dx, dy, dz])
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let dx = rhs.0[Self::X] - self.0[Self::X];
        let dy = rhs.0[Self::Y] - self.0[Self::Y];
        let dz = rhs.0[Self::Z] - self.0[Self::Z];
        Self([dx, dy, dz])
    }
}

impl Coord {
    const X: usize = 0;
    const Y: usize = 1;
    const Z: usize = 2;

    fn norm_abs(self, other: Self) -> [u16; 3] {
        let dx = (other.0[Self::X] - self.0[Self::X]).unsigned_abs();
        let dy = (other.0[Self::Y] - self.0[Self::Y]).unsigned_abs();
        let dz = (other.0[Self::Z] - self.0[Self::Z]).unsigned_abs();

        [dx, dy, dz].tap_mut(|d| d.sort_unstable())
    }

    fn distance(self, other: Self) -> u32 {
        let diff = self - other;

        u32::from(diff.0[Self::X].unsigned_abs())
            + u32::from(diff.0[Self::Y].unsigned_abs())
            + u32::from(diff.0[Self::Z].unsigned_abs())
    }

    fn translate(self, mapping: Mapping) -> Self {
        let mut translated = [0_i16; 3];
        for (to, (to_idx, sign)) in self.0.into_iter().zip(mapping) {
            translated[to_idx] = to * sign;
        }
        Self(translated)
    }
}
impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(',')
                .map(str::parse::<i16>)
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_ex() {
        let input = r#"--- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 79);
        assert_eq!(res2, 3621);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 308);
        assert_eq!(res2, 12124);
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
