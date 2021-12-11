use derive_more::Deref;
use num_enum::TryFromPrimitive;
use std::ops::AddAssign;
use tap::Tap;

register!(
    "input/day3.txt";
    (input: input!(Bits)) -> u64 {
        part1(&input);
        part2(&mut input);
    }
);

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Bit {
    One = b'1',
    Zero = b'0',
}

impl AddAssign<Bit> for isize {
    fn add_assign(&mut self, rhs: Bit) {
        *self += match rhs {
            Bit::One => 1,
            Bit::Zero => -1,
        }
    }
}

#[derive(Debug, Deref)]
pub struct Bits(Vec<Bit>);

impl FromIterator<Bit> for Bits {
    fn from_iter<T: IntoIterator<Item = Bit>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<String> for Bits {
    fn from(s: String) -> Self {
        s.bytes().map(|b| Bit::try_from(b).unwrap()).collect()
    }
}

impl<'a> TryFrom<&'a Bits> for u64 {
    type Error = std::num::ParseIntError;

    fn try_from(bits: &'a Bits) -> Result<Self, Self::Error> {
        let bits = bits
            .iter()
            .map(|&b| char::from(b as u8))
            .collect::<String>();

        Self::from_str_radix(&bits, 2)
    }
}

fn part1(items: &[Bits]) -> u64 {
    let mut ones = vec![0_isize];
    for bits in items {
        for (pos, &bit) in bits.iter().enumerate() {
            if pos >= ones.len() {
                ones.resize(pos + 1, 0);
            }
            ones[pos] += bit;
        }
    }

    let bits = ones
        .into_iter()
        .map(|count| if count > 0 { Bit::One } else { Bit::Zero })
        .collect::<Bits>();

    let significant_bits = bits.len();

    let gamma = u64::try_from(&bits).unwrap();
    let epsilon = (!gamma) & ((1 << significant_bits) - 1);

    gamma * epsilon
}

fn part2(items: &mut [Bits]) -> u64 {
    let o2 = find_rating_2(items, |count| count >= 0);
    let co2 = find_rating_2(items, |count| count < 0);

    o2 * co2
}

fn find_rating_2(mut items: &mut [Bits], select: impl Fn(isize) -> bool) -> u64 {
    for pos in 0.. {
        if let [result] = items {
            return u64::try_from(&*result).unwrap();
        }

        let count = items
            .iter()
            .fold(0_isize, |count, bits| count.tap_mut(|c| *c += bits[pos]));

        let number_of_ones = items
            .iter_mut()
            .partition_in_place(|n| matches!(n[pos], Bit::One));

        let (ones, zeroes) = items.split_at_mut(number_of_ones);

        items = if select(count) { ones } else { zeroes };
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test_ex() {
        let items = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;
        let (res1, res2) = Solver::run_on(items);
        assert_eq!(res1, 198);
        assert_eq!(res2, 230);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 4_160_394);
        assert_eq!(res2, 4_125_600);
    }
}
