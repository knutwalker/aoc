use bitvec::{field::BitField, mem::BitMemory, order::Msb0, slice::BitSlice, vec::BitVec};
use std::{
    cmp::Ordering,
    convert::Infallible,
    ops::{Add, Mul},
    str::FromStr,
};

type Output = u64;

register!(
    "input/day16.txt";
    (input: input!(first input!(parse Packet))) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(packet: &Packet) -> Output {
    packet.version
}

fn part2(packet: &Packet) -> Output {
    packet.val
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    version: Output,
    val: Output,
}

#[derive(Debug)]
struct Input<'a>(&'a Bits);

impl<'a> Input<'a> {
    fn load<V: BitMemory>(&mut self, num_bits: usize) -> V {
        let (val, input) = self.0.split_at(num_bits);
        self.0 = input;
        val.load_be()
    }

    fn read(&mut self, num_bits: usize) -> Self {
        let (val, input) = self.0.split_at(num_bits);
        self.0 = input;
        Self(val)
    }

    fn decode(&mut self) -> Packet {
        let version = self.load(3);
        let type_id = self.load::<u8>(3);

        match type_id {
            0 => self.decode_operator(version, 0, Output::add),
            1 => self.decode_operator(version, 1, Output::mul),
            2 => self.decode_operator(version, Output::MAX, Output::min),
            3 => self.decode_operator(version, Output::MIN, Output::max),
            4 => self.decode_literal(version),
            op @ (5 | 6 | 7) => {
                let cmp = match op {
                    5 => Ordering::Greater,
                    6 => Ordering::Less,
                    _ => Ordering::Equal,
                };
                self.decode_operator(version, Output::MAX, move |a, b| {
                    if a == Output::MAX {
                        b
                    } else {
                        (a.cmp(&b) == cmp) as _
                    }
                })
            }
            op => unreachable!("invalid op: {}", op),
        }
    }

    fn decode_operator(
        &mut self,
        mut version: Output,
        mut val: Output,
        op: impl Fn(Output, Output) -> Output,
    ) -> Packet {
        if self.read(1).0[0] {
            let length = self.load::<usize>(11);

            for _ in 0..length {
                let packet = self.decode();
                version += packet.version;
                val = op(val, packet.val);
            }
        } else {
            let length = self.load::<usize>(15);
            let mut subs = self.read(length);

            while !subs.0.is_empty() {
                let packet = subs.decode();
                version += packet.version;
                val = op(val, packet.val);
            }
        };
        Packet { version, val }
    }

    fn decode_literal(&mut self, version: Output) -> Packet {
        let mut val = BitVec::<Msb0, Output>::new();

        for (idx, chunk) in self.0.chunks(5).enumerate() {
            val.extend(&chunk[1..]);
            if !chunk[0] {
                let val = val.load_be::<Output>();
                let consumed = (idx + 1) * 5;
                self.0 = &self.0[consumed..];
                return Packet { version, val };
            }
        }

        unreachable!("Invalid literal");
    }
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
        Ok(Input(&bits).decode())
    }
}

type Bits = BitSlice<Msb0, u8>;

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

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
