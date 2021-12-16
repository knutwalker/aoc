use bitvec::{field::BitField, order::Msb0, slice::BitSlice, vec::BitVec};
use std::{cmp::Ordering, convert::Infallible, str::FromStr};

type Output = u64;

register!(
    "input/day16.txt";
    (input: input!(first input!(parse Packet))) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(packet: &Packet) -> Output {
    packet.add_versions()
}

fn part2(packet: &Packet) -> Output {
    packet.eval()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    version: u8,
    val: Val,
}

impl Packet {
    fn add_versions(&self) -> Output {
        match &self.val {
            Val::Lit(_) => Output::from(self.version),
            Val::Op(_, args) => {
                args.iter().map(Self::add_versions).sum::<Output>() + Output::from(self.version)
            }
        }
    }

    fn eval(&self) -> Output {
        match &self.val {
            Val::Lit(v) => *v,
            Val::Op(op, args) => match op {
                0 => args.iter().map(Self::eval).sum(),
                1 => args.iter().map(Self::eval).product(),
                2 => args.iter().map(Self::eval).min().unwrap(),
                3 => args.iter().map(Self::eval).max().unwrap(),
                op @ (5 | 6 | 7) => {
                    let cmp = match op {
                        5 => Ordering::Greater,
                        6 => Ordering::Less,
                        _ => Ordering::Equal,
                    };
                    match &args[..] {
                        [fst, snd] => (fst.eval().cmp(&snd.eval()) == cmp) as _,
                        _ => unreachable!("invalid number of args for op {}: {}", args.len(), op),
                    }
                }
                op => unreachable!("invalid op: {}", op),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Val {
    Lit(u64),
    Op(u8, Vec<Packet>),
}

impl FromStr for Packet {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s
            .as_bytes()
            .array_chunks()
            .filter_map(|[c1, c2]| {
                let c1 = char::from(*c1).to_digit(16)? as u8;
                let c2 = char::from(*c2).to_digit(16)? as u8;
                Some(c1 << 4 | c2)
            })
            .collect::<BitVec<Msb0, _>>();
        Ok(decode(&bits).0)
    }
}

type Bits = BitSlice<Msb0, u8>;

fn decode(input: &Bits) -> (Packet, &Bits) {
    let (version, input) = input.split_at(3);
    let (type_id, input) = input.split_at(3);

    let (val, input) = match type_id.load_be::<u8>() {
        4 => {
            let (result, input) = decode_literal(input);
            (Val::Lit(result), input)
        }
        op => {
            let (result, input) = decode_operator(input);
            (Val::Op(op, result), input)
        }
    };

    (
        Packet {
            version: version.load_be(),
            val,
        },
        input,
    )
}

fn decode_literal(input: &Bits) -> (Output, &Bits) {
    let mut result = BitVec::<Msb0, Output>::new();

    for (idx, chunk) in input.chunks(5).enumerate() {
        result.extend(&chunk[1..]);
        if !chunk[0] {
            let result = result.load_be::<Output>();
            let consumed = (idx + 1) * 5;
            let input = &input[consumed..];
            return (result, input);
        }
    }

    unreachable!("Invalid literal");
}

fn decode_operator(input: &Bits) -> (Vec<Packet>, &Bits) {
    if input[0] {
        let (length, mut input) = input[1..].split_at(11);
        let length = length.load_be::<usize>();

        let mut results = Vec::with_capacity(length);

        for _ in 0..length {
            let (result, remaining) = decode(input);
            results.push(result);
            input = remaining;
        }

        (results, input)
    } else {
        let (length, input) = input[1..].split_at(15);
        let length = length.load_be::<usize>();

        let (mut subs, input) = input.split_at(length);
        let mut results = Vec::new();

        while !subs.is_empty() {
            let (result, remaining) = decode(subs);
            results.push(result);
            subs = remaining;
        }

        (results, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_parse1() {
        let result = Solver::parse_input("D2FE28");
        assert_eq!(
            Packet {
                version: 6,
                val: Val::Lit(2021)
            },
            result
        );
    }

    #[test]
    fn test_parse2() {
        let result = Solver::parse_input("38006F45291200");
        assert_eq!(
            Packet {
                version: 1,
                val: Val::Op(
                    6,
                    vec![
                        Packet {
                            version: 6,
                            val: Val::Lit(10)
                        },
                        Packet {
                            version: 2,
                            val: Val::Lit(20)
                        }
                    ]
                )
            },
            result
        );
    }

    #[test]
    fn test_parse3() {
        let result = Solver::parse_input("EE00D40C823060");
        assert_eq!(
            Packet {
                version: 7,
                val: Val::Op(
                    3,
                    vec![
                        Packet {
                            version: 2,
                            val: Val::Lit(1)
                        },
                        Packet {
                            version: 4,
                            val: Val::Lit(2)
                        },
                        Packet {
                            version: 1,
                            val: Val::Lit(3)
                        }
                    ]
                )
            },
            result
        );
    }

    #[test]
    fn test_ex_part1() {
        let (res1, _) = Solver::run_on("8A004A801A8002F478");
        assert_eq!(res1, 16);

        let (res1, _) = Solver::run_on("620080001611562C8802118E34");
        assert_eq!(res1, 12);

        let (res1, _) = Solver::run_on("C0015000016115A2E0802F182340");
        assert_eq!(res1, 23);

        let (res1, _) = Solver::run_on("A0016C880162017C3686B18A3D4780");
        assert_eq!(res1, 31);
    }

    #[test]
    fn test_ex_part2() {
        let (_, res2) = Solver::run_on("C200B40A82");
        assert_eq!(res2, 3);

        let (_, res2) = Solver::run_on("04005AC33890");
        assert_eq!(res2, 54);

        let (_, res2) = Solver::run_on("880086C3E88112");
        assert_eq!(res2, 7);

        let (_, res2) = Solver::run_on("CE00C43D881120");
        assert_eq!(res2, 9);

        let (_, res2) = Solver::run_on("D8005AC2A8F0");
        assert_eq!(res2, 1);

        let (_, res2) = Solver::run_on("F600BC2D8F");
        assert_eq!(res2, 0);

        let (_, res2) = Solver::run_on("9C005AC2F8F0");
        assert_eq!(res2, 0);

        let (_, res2) = Solver::run_on("9C0141080250320F1802104A08");
        assert_eq!(res2, 1);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 986);
        assert_eq!(res2, 18_234_816_469_452);
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
