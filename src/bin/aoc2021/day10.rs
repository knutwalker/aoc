use aoc::MedianExt;

register!(
    "input/day10.txt";
    (input: input!([u8])) -> u64 {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[&[u8]]) -> u64 {
    items
        .iter()
        .filter_map(|l| parse(l).err())
        .map(|c| match c {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => unreachable!("{}", c),
        })
        .sum()
}

fn part2(items: &[&[u8]]) -> u64 {
    items
        .iter()
        .filter_map(|l| parse(l).ok())
        .map(|c| {
            c.bytes()
                .map(|c| match c {
                    b')' => 1,
                    b']' => 2,
                    b'}' => 3,
                    b'>' => 4,
                    _ => unreachable!("{}", c),
                })
                .fold(0, |total, score| total * 5 + score)
        })
        .collect::<Vec<_>>()
        .median()
}

fn parse(bytes: &[u8]) -> Result<String, u8> {
    fn parse(mut bytes: &[u8], level: usize, closer: u8) -> Result<Result<&[u8], String>, u8> {
        let mut closing = loop {
            match bytes.split_first() {
                Some((c, rest)) => match *c {
                    open @ (b'(' | b'[' | b'{' | b'<') => {
                        let closer = match open {
                            b'(' => b')',
                            b'[' => b']',
                            b'{' => b'}',
                            b'<' => b'>',
                            _ => unreachable!(),
                        };
                        match parse(rest, level + 1, closer)? {
                            Ok(rest) => bytes = rest,
                            Err(closing) => break closing,
                        }
                    }
                    close if close == closer => return Ok(Ok(rest)),
                    illegal => return Err(illegal),
                },
                None => break String::with_capacity(level),
            }
        };

        if closer != 0 {
            closing.push(char::from(closer));
        }

        Ok(Err(closing))
    }

    match parse(bytes, 0, 0)? {
        Err(closing) => Ok(closing),
        Ok(rest) => unreachable!("Incomplete parse, remaining: {:?}", rest),
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
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 26397);
        assert_eq!(res2, 288_957);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 394_647);
        assert_eq!(res2, 2_380_061_249);
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
