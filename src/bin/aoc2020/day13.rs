type Input = String;

register!(
    "input/day13.txt";
    (input: input!(Input)) -> i64 {
        run1(&input);
        run2(input);
    }
);

fn run1(input: &[Input]) -> i64 {
    let time = input[0].parse::<i64>().unwrap();
    input[1]
        .split(',')
        .map(str::parse::<i64>)
        .filter_map(Result::ok)
        .min_by_key(|t| t - (time % t))
        .map(|bus| bus * (bus - (time % bus)))
        .unwrap()
}

fn run2(input: Vec<Input>) -> i64 {
    fn gcd(m: usize, n: usize) -> usize {
        match n.checked_rem(m) {
            None => n,
            Some(n) => gcd(n, m),
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        a * b / gcd(a, b)
    }

    input[1]
        .split(',')
        .map(str::parse::<i64>)
        .enumerate()
        .filter_map(|(pos, id)| id.ok().map(move |id| (pos as i64, id)))
        .fold((0, 1), |(time, step), (pos, bus)| {
            (
                (time..)
                    .step_by(step)
                    .find(|t| (t + pos) % bus == 0)
                    .unwrap(),
                lcm(step, bus as usize),
            )
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::SolutionExt;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 333);
        assert_eq!(res2, 690123192779524);
    }

    #[test]
    fn test_ex1() {
        assert_eq!(
            (295, 1068781),
            Solver::run_on(
                "
        939
        7,13,x,x,59,x,31,19
    "
            )
        );
    }

    #[test]
    fn test_p2_0() {
        assert_eq!(
            1068781,
            run2(vec![String::new(), String::from("7,13,x,x,59,x,31,19")])
        );
    }

    #[test]
    fn test_p2_1() {
        assert_eq!(
            754018,
            run2(vec![String::new(), String::from("67,7,59,61")])
        );
        assert_eq!(
            779210,
            run2(vec![String::new(), String::from("67,x,7,59,61")])
        );
        assert_eq!(
            1261476,
            run2(vec![String::new(), String::from("67,7,x,59,61")])
        );
    }

    #[test]
    fn test_p2_2() {
        assert_eq!(
            1202161486,
            run2(vec![String::new(), String::from("1789,37,47,1889")])
        );
    }
}
