type Input = Vec<u8>;
type Output = u64;

register!(
    "input/day23.txt";
    (input: input!(first input!(Input))) -> Output {
        run1(&input, 100);
        run2(&input);
    }
);

fn run1(input: &[u8], games: u32) -> Output {
    let cups = run_any(input, 10, games);

    let mut cup = cups[1];
    let mut result = String::new();
    while cup != 1 {
        result.push((b'0' + cup as u8) as char);
        cup = cups[cup as usize];
    }

    result.parse().unwrap()
}

fn run2(input: &[u8]) -> Output {
    let cups = run_any(input, 1_000_001, 10_000_000);

    let c1 = cups[1];
    let c2 = cups[c1 as usize];
    u64::from(c1) * u64::from(c2)
}

fn run_any(input: &[u8], cards: u32, games: u32) -> Vec<u32> {
    let mut cups = vec![0; cards as usize];

    let mut prev = -1;
    for value in input
        .iter()
        .copied()
        .map(|c| u32::from(c - b'0'))
        .chain(10..cards)
    {
        cups[value as usize] = 0;

        if prev >= 0 {
            cups[prev as usize] = value;
        }
        prev = value as i32;
    }

    let mut current = usize::from(input[0] - b'0') as u32;
    cups[prev as usize] = current;

    #[cfg(debug_assertions)]
    {
        for (i, &cup) in cups.iter().enumerate().skip(1) {
            debug_assert!(cup != 0, "Cup {} is not linked", i);
        }
    }

    let limit = cards - 1;
    for _ in 0..games {
        let c1 = cups[current as usize];
        let c2 = cups[c1 as usize];
        let c3 = cups[c2 as usize];

        let mut target = current - 1;
        while target == 0 || target == c1 || target == c2 || target == c3 {
            if target == 0 {
                target = limit;
            } else {
                target -= 1;
            }
        }

        let after_target = std::mem::replace(&mut cups[target as usize], c1);
        let after_current = std::mem::replace(&mut cups[c3 as usize], after_target);
        cups[current as usize] = after_current;
        current = after_current;
    }

    cups
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 97_624_853);
        assert_eq!(res2, 664_642_452_305);
    }

    #[test]
    fn test_ex() {
        assert_eq!((67_384_529, 149_245_887_792), Solver::run_on("389125467"));
    }

    #[test]
    fn test_small_ex() {
        assert_eq!(92_658_374, run1(b"389125467", 10));
    }
}
