use const_combinations::SliceExt;
use std::{convert::Infallible, fmt::Display, str::FromStr};
use tap::Tap;

type Input = Num;
type Output = u64;

register!(
    "input/day18.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    reduce_all(items.iter().cloned()).magnitude()
}

fn part2(items: &[Input]) -> Output {
    items
        .permutations()
        .map(|[l, r]| l.clone().fold(r.clone()).magnitude())
        .max()
        .unwrap()
}

fn reduce_all(nums: impl IntoIterator<Item = Num>) -> Num {
    nums.into_iter().reduce(Num::fold).expect("empty input")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Num {
    Reg(u8),
    Pair([Box<Self>; 2]),
}

impl Num {
    fn add(&mut self, rhs: Self) {
        let lhs = std::mem::replace(self, Self::of(0));
        let num = Self::of((lhs, rhs));
        *self = num;
    }

    fn explode(&mut self) -> bool {
        fn explode(num: &mut Num, level: u8) -> Option<(Option<u8>, Option<u8>)> {
            const LEFT: usize = 0;
            const RIGHT: usize = 1;

            fn add_to(num: &mut Num, idx: usize, val: Option<u8>) {
                fn add(num: &mut Num, idx: usize, val: u8) {
                    match num {
                        Num::Reg(v) => *v += val,
                        Num::Pair(pair) => add(&mut *pair[idx], idx, val),
                    }
                }

                if let Some(v) = val {
                    add(num, idx, v);
                }
            }

            if level == 4 {
                matches!(num, Num::Pair(_)).then(|| {
                    let Num::Pair([left, right]) =
                        std::mem::replace(num, Num::of(0)) else { unreachable!() };

                    match (*left, *right) {
                        (Num::Reg(left), Num::Reg(right)) => (Some(left), Some(right)),
                        (l, r) => panic!("Nested pair is nested: {:?}, {:?}", l, r),
                    }
                })
            } else {
                match num {
                    Num::Pair([left, right]) => explode(left, level + 1)
                        .map(|(l, r)| {
                            add_to(right, LEFT, r);
                            (l, None)
                        })
                        .or_else(|| {
                            explode(right, level + 1).map(|(l, r)| {
                                add_to(left, RIGHT, l);
                                (None, r)
                            })
                        }),
                    Num::Reg(_) => None,
                }
            }
        }

        explode(self, 0).is_some()
    }

    fn split(&mut self) -> bool {
        fn split(num: &mut Num) -> bool {
            match num {
                Num::Reg(ref v) if *v >= 10 => {
                    let l = v / 2;
                    let r = (v + 1) / 2;
                    *num = Num::of((l, r));
                    true
                }
                Num::Reg(_) => false,
                Num::Pair([left, right]) => split(&mut *left) || split(&mut *right),
            }
        }

        split(self)
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn fold(self, rhs: Self) -> Self {
        self.tap_mut(move |n| n.add(rhs)).tap_mut(Self::reduce)
    }

    fn magnitude(self) -> Output {
        match self {
            Num::Reg(v) => Output::from(v),
            Num::Pair([l, r]) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

impl Num {
    fn of(val: impl Into<Self>) -> Self {
        val.into()
    }
}

impl From<u8> for Num {
    fn from(v: u8) -> Self {
        Self::Reg(v)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<&'_ str> for Num {
    fn from(s: &'_ str) -> Self {
        s.parse().unwrap()
    }
}

impl<T: Into<Self>, U: Into<Self>> From<(T, U)> for Num {
    fn from((fst, snd): (T, U)) -> Self {
        Self::Pair([Box::new(fst.into()), Box::new(snd.into())])
    }
}

impl FromStr for Num {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(bytes: &[u8]) -> (Num, &[u8]) {
            match bytes.split_first().expect("empty input") {
                (b'[', rest) => {
                    let (fst, rest) = parse(rest);
                    let rest = expect(b',', rest);
                    let (snd, rest) = parse(rest);
                    let rest = expect(b']', rest);
                    (Num::of((fst, snd)), rest)
                }
                (c @ b'0'..=b'9', rest) => (Num::of(c - b'0'), rest),
                (illegal, _) => panic!("unexpected {}", *illegal),
            }
        }

        #[cfg(debug_assertions)]
        fn expect(expect: u8, bytes: &[u8]) -> &[u8] {
            let (actual, bytes) = bytes.split_first().expect("missing byte");
            assert_eq!(*actual, expect);
            bytes
        }

        #[cfg(not(debug_assertions))]
        fn expect(_expect: u8, bytes: &[u8]) -> &[u8] {
            &bytes[1..]
        }

        Ok(parse(s.as_bytes()).0)
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::Reg(v) => write!(f, "{}", v),
            Num::Pair([lhs, rhs]) => write!(f, "[{},{}]", lhs, rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_parse() {
        let input = r#"[1,2]
        [[1,2],3]
        [9,[8,7]]
        [[1,9],[8,5]]
        [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
        [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
        [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"#;

        let expected = vec![
            Num::of((1, 2)),
            Num::of(((1, 2), 3)),
            Num::of(((9), (8, 7))),
            Num::of(((1, 9), (8, 5))),
            Num::of(((((1, 2), (3, 4)), ((5, 6), (7, 8))), 9)),
            Num::of((((9, (3, 8)), ((0, 9), 6)), (((3, 7), (4, 9)), 3))),
            Num::of((
                (((1, 3), (5, 3)), ((1, 3), (8, 7))),
                (((4, 9), (6, 9)), ((8, 2), (7, 3))),
            )),
        ];

        let actuals = Solver::parse_input(input);

        assert_eq!(actuals, expected);
    }

    #[allow(clippy::needless_pass_by_value)]
    fn test_num<T>(mut num: Num, f: impl FnOnce(&mut Num) -> T, expected: Num) {
        drop(f(&mut num));
        assert_eq!(num, expected);
    }

    #[test]
    fn test_add() {
        test_num(
            Num::of((1, 2)),
            |num| num.add(Num::of(((3, 4), 5))),
            Num::of(((1, 2), ((3, 4), 5))),
        );
    }

    #[test]
    fn test_explode() {
        test_num(
            Num::of((((((9, 8), 1), 2), 3), 4)),
            Num::explode,
            Num::of(((((0, 9), 2), 3), 4)),
        );
        test_num(
            Num::of((7, (6, (5, (4, (3, 2)))))),
            Num::explode,
            Num::of((7, (6, (5, (7, 0))))),
        );
        test_num(
            Num::of(((6, (5, (4, (3, 2)))), 1)),
            Num::explode,
            Num::of(((6, (5, (7, 0))), 3)),
        );
        test_num(
            Num::of(((3, (2, (1, (7, 3)))), (6, (5, (4, (3, 2)))))),
            Num::explode,
            Num::of(((3, (2, (8, 0))), (9, (5, (4, (3, 2)))))),
        );
        test_num(
            Num::of(((3, (2, (8, 0))), (9, (5, (4, (3, 2)))))),
            Num::explode,
            Num::of(((3, (2, (8, 0))), (9, (5, (7, 0))))),
        );
    }

    #[test]
    fn test_explode2() {
        let mut num =
            Num::of("[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]");
        num.explode();

        let expected =
            Num::of("[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]");

        assert_eq!(num, expected);
    }

    #[test]
    fn test_split() {
        test_num(Num::of(10), Num::split, Num::of((5, 5)));
        test_num(Num::of(11), Num::split, Num::of((5, 6)));
        test_num(Num::of(12), Num::split, Num::of((6, 6)));
    }

    #[test]
    fn test_process1() {
        let mut num = Num::of(((((4, 3), 4), 4), (7, ((8, 4), 9))));

        num.add(Num::of((1, 1)));
        let after_addition = Num::of((((((4, 3), 4), 4), (7, ((8, 4), 9))), (1, 1)));
        assert_eq!(num, after_addition);

        num.explode();
        let after_explode = Num::of(((((0, 7), 4), (7, ((8, 4), 9))), (1, 1)));
        assert_eq!(num, after_explode);

        num.explode();
        let after_explode = Num::of(((((0, 7), 4), (15, (0, 13))), (1, 1)));
        assert_eq!(num, after_explode);

        num.split();
        let after_split = Num::of(((((0, 7), 4), ((7, 8), (0, 13))), (1, 1)));
        assert_eq!(num, after_split);

        num.split();
        let after_split = Num::of(((((0, 7), 4), ((7, 8), (0, (6, 7)))), (1, 1)));
        assert_eq!(num, after_split);

        num.explode();
        let after_explode = Num::of(((((0, 7), 4), ((7, 8), (6, 0))), (8, 1)));
        assert_eq!(num, after_explode);
    }

    #[test]
    fn test_reduce() {
        let num = Num::of(((((4, 3), 4), 4), (7, ((8, 4), 9))));
        let num = num.fold(Num::of((1, 1)));

        assert_eq!(num, Num::of(((((0, 7), 4), ((7, 8), (6, 0))), (8, 1))));
    }

    #[test]
    fn test_reduce2() {
        let num1 = Num::of((((0, (4, 5)), (0, 0)), (((4, 5), (2, 6)), (9, 5))));
        let num2 = Num::of((7, (((3, 7), (4, 3)), ((6, 3), (8, 8)))));

        let num = num1.fold(num2);

        assert_eq!(
            num,
            Num::of((
                (((4, 0), (5, 4)), ((7, 7), (6, 0))),
                ((8, (7, 7)), ((7, 9), (5, 0)))
            ))
        );
    }

    #[test]
    fn test_reduce_all1() {
        let nums = Solver::parse_input(
            r#"[1,1]
            [2,2]
            [3,3]
            [4,4]"#,
        );

        let num = reduce_all(nums);

        assert_eq!(num, Num::of(((((1, 1), (2, 2)), (3, 3)), (4, 4))));
    }

    #[test]
    fn test_reduce_all2() {
        let nums = Solver::parse_input(
            r#"[1,1]
            [2,2]
            [3,3]
            [4,4]
            [5,5]"#,
        );

        let num = reduce_all(nums);

        assert_eq!(num, Num::of(((((3, 0), (5, 3)), (4, 4)), (5, 5))));
    }

    #[test]
    fn test_reduce_all3() {
        let nums = Solver::parse_input(
            r#"[1,1]
            [2,2]
            [3,3]
            [4,4]
            [5,5]
            [6,6]"#,
        );

        let num = reduce_all(nums);

        assert_eq!(num, Num::of(((((5, 0), (7, 4)), (5, 5)), (6, 6))));
    }

    #[test]
    fn test_reduce_all() {
        let input = r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        [7,[5,[[3,8],[1,4]]]]
        [[2,[2,2]],[8,[8,1]]]
        [2,9]
        [1,[[[9,3],9],[[9,0],[0,7]]]]
        [[[5,[7,4]],7],1]
        [[[[4,2],2],6],[8,7]]"#;

        let nums = Solver::parse_input(input);

        let num = reduce_all(nums);

        assert_eq!(
            num,
            Num::of((
                (((8, 7), (7, 7)), ((8, 6), (7, 7))),
                (((0, 7), (6, 6)), (8, 7)),
            )),
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Num::of("[9,1]").magnitude(), 29);
        assert_eq!(Num::of("[1,9]").magnitude(), 21);
        assert_eq!(Num::of("[[9,1],[1,9]]").magnitude(), 129);

        assert_eq!(Num::of("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(
            Num::of("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),
            1384
        );
        assert_eq!(Num::of("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
        assert_eq!(Num::of("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
        assert_eq!(Num::of("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(), 1137);
        assert_eq!(
            Num::of("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
            3488
        );
    }

    #[test]
    fn test_ex() {
        let input = r#"
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        "#;

        assert_eq!(
            reduce_all(Solver::parse_input(input)),
            Num::of("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );

        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 4140);
        assert_eq!(res2, 3993);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 4480);
        assert_eq!(res2, 4676);
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
