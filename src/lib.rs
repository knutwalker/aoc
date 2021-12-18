use std::{
    fmt::Debug,
    fmt::Display,
    marker::PhantomData,
    str::FromStr,
    time::{Duration, Instant},
};

#[macro_export]
macro_rules! poop {
    ($($arg:tt)*) => {
        {
            #[cfg(debug_assertions)]
            {
                println!($($arg)*);
            }
        }
    };
}

pub trait MedianExt<T> {
    fn median(self) -> T;
}

impl<'a, T: Ord> MedianExt<&'a T> for &'a mut [T] {
    #[inline]
    fn median(self) -> &'a T {
        let index = self.len() / 2;
        self.select_nth_unstable(index).1
    }
}

impl<T: Ord + Copy> MedianExt<T> for Vec<T> {
    #[inline]
    fn median(mut self) -> T {
        *(self.as_mut_slice().median())
    }
}

pub trait PuzzleInput
where
    Self: Sized,
{
    type Out;

    fn from_input(input: &str) -> Self::Out;
}

pub trait ProcessInput {
    type In: PuzzleInput;
    type Out;

    fn process(input: <Self::In as PuzzleInput>::Out) -> Self::Out;
}

pub struct Blocks<T>(PhantomData<T>);

impl<T> PuzzleInput for Blocks<T>
where
    T: PuzzleInput,
{
    type Out = Vec<T::Out>;

    fn from_input(input: &str) -> Self::Out {
        input.split("\n\n").map(|l| T::from_input(l)).collect()
    }
}

pub struct Parsing<T>(PhantomData<T>);

impl<T> PuzzleInput for Parsing<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Out = Vec<T>;

    fn from_input(input: &str) -> Self::Out {
        lines(input).map(|l| T::from_str(l).unwrap()).collect()
    }
}

pub struct As<T>(PhantomData<T>);

impl<T> PuzzleInput for As<T>
where
    T: From<String>,
{
    type Out = Vec<T>;

    fn from_input(input: &str) -> Self::Out {
        lines(input).map(|l| T::from(String::from(l))).collect()
    }
}

pub struct Post<T>(PhantomData<T>);

impl<T> PuzzleInput for Post<T>
where
    T: ProcessInput,
{
    type Out = T::Out;

    fn from_input(input: &str) -> Self::Out {
        let input = T::In::from_input(input);
        T::process(input)
    }
}

pub struct First<T>(PhantomData<T>);

impl<T> ProcessInput for First<T>
where
    T: PuzzleInput,
    T::Out: PopFirst,
{
    type In = T;

    type Out = <T::Out as PopFirst>::Out;

    fn process(input: <T as PuzzleInput>::Out) -> Self::Out {
        <T::Out as PopFirst>::pop_first(input)
    }
}

pub trait PopFirst {
    type Out;

    fn pop_first(self) -> Self::Out;
}

impl<T> PopFirst for Vec<T> {
    type Out = T;

    fn pop_first(self) -> Self::Out {
        self.into_iter().next().unwrap()
    }
}

pub fn lines(s: &str) -> impl Iterator<Item = &str> + '_ {
    s.lines().map(str::trim).filter(|line| !line.is_empty())
}

pub struct PuzzleSolution<T> {
    pub part1: T,
    pub part2: T,
    pub parse_time: Duration,
    pub part1_time: Duration,
    pub part2_time: Duration,
}

pub trait Solution {
    type Input: PuzzleInput;
    type Output;

    fn puzzle_input() -> &'static str;

    #[inline]
    fn parse_input(input: &str) -> <Self::Input as PuzzleInput>::Out {
        <Self::Input as PuzzleInput>::from_input(input)
    }

    fn run(
        input: <Self::Input as PuzzleInput>::Out,
        parse_time: Duration,
    ) -> PuzzleSolution<Self::Output>;

    fn solve() -> PuzzleSolution<Self::Output> {
        let input = Self::puzzle_input();
        let start = Instant::now();
        let input = Self::parse_input(input);
        let parse_time = start.elapsed();
        Self::run(input, parse_time)
    }
}

pub trait SolutionExt: Solution {
    fn run_on(input: &str) -> (Self::Output, Self::Output) {
        let input = Self::parse_input(input);
        let PuzzleSolution { part1, part2, .. } = Self::run(input, Duration::ZERO);
        (part1, part2)
    }

    fn run_on_input() -> (Self::Output, Self::Output) {
        let input = Self::puzzle_input();
        Self::run_on(input)
    }
}

impl<T: Solution> SolutionExt for T {}

pub struct ResultLine {
    prefix: String,
    duration: Duration,
    solution: Option<Box<dyn Display>>,
}

impl ResultLine {
    pub fn solution<T>(part: u8, duration: Duration, solution: T) -> Self
    where
        T: Display + 'static,
    {
        Self::new(format!("Part {}", part), duration, Some(Box::new(solution)))
    }

    pub fn note<T>(note: &T, duration: Duration) -> Self
    where
        T: Display + ?Sized,
    {
        Self::new(note.to_string(), duration, None)
    }

    fn new(prefix: String, duration: Duration, solution: Option<Box<dyn Display>>) -> Self {
        Self {
            prefix,
            duration,
            solution,
        }
    }
}

impl Display for ResultLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use owo_colors::{OwoColorize, Stream::Stdout};
        const DEFAULT_WIDTH: usize = 42;

        let duration = format!(" ({})", humantime::format_duration(self.duration));

        write!(
            f,
            "{}{}",
            self.prefix,
            duration.if_supports_color(Stdout, |d| d.dimmed())
        )?;

        if let Some(solution) = self.solution.as_deref() {
            let max_width = f.width().unwrap_or(DEFAULT_WIDTH);
            let printed_width = self.prefix.chars().count() + duration.chars().count();
            let dots = max_width.saturating_sub(printed_width).saturating_sub(2);
            let dots = ".".repeat(dots);

            write!(
                f,
                " {} ",
                dots.if_supports_color(Stdout, |t| t.bright_black())
            )?;

            let solution = solution.to_string();
            let mut solution = solution.lines().filter(|l| !l.is_empty());

            write!(
                f,
                "{}",
                solution
                    .next()
                    .unwrap()
                    .if_supports_color(Stdout, |t| t.bold())
            )?;

            for line in solution {
                writeln!(f)?;
                write!(
                    f,
                    "{:>w$}    {}",
                    "",
                    line.if_supports_color(Stdout, |t| t.bold()),
                    w = max_width
                )?;
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! input {
    (verbatim $input_ty:ty) => {
        $input_ty
    };

    ($input_ty:ty) => {
        $crate::As<$input_ty>
    };

    (parse $input_ty:ty) => {
        $crate::Parsing<$input_ty>
    };

    (blocks $input_ty:ty) => {
        $crate::Blocks<$input_ty>
    };

    (process $input_ty:ty) => {
        $crate::Post<$input_ty>
    };

    (chunk $input_ty:ty) => {
        input!(blocks input!($input_ty))
    };

    (first $input_ty:ty) => {
        input!(process $crate::First<$input_ty>)
    };
}

#[macro_export]
macro_rules! register {
    ($file:literal; ($input:ident: $input_ty:ty) -> $output_ty:ty { $part1:expr; $part2:expr $(;)? }) => {
        pub(crate) struct Solver;

        impl $crate::Solution for Solver {
            type Input = $input_ty;
            type Output = $output_ty;

            #[inline]
            fn puzzle_input() -> &'static str {
                ::std::include_str!($file)
            }

            #[inline]
            fn run(
                mut $input: <$input_ty as $crate::PuzzleInput>::Out,
                parse_time: ::std::time::Duration,
            ) -> $crate::PuzzleSolution<Self::Output> {
                let start = ::std::time::Instant::now();
                let part1 = $part1;
                let part1_time = start.elapsed();
                let start = ::std::time::Instant::now();
                let part2 = $part2;
                let part2_time = start.elapsed();

                $crate::PuzzleSolution {
                    part1,
                    part2,
                    part1_time,
                    part2_time,
                    parse_time,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! aoc_main {
    ($($day:literal => $md:ident),+ $(,)?) => {
        use ::aoc::{PuzzleSolution, Solution};
        $(mod $md);+;

        fn main() {
            let suppress_output = ::std::env::var_os("AOC_NO_OUTPUT").is_some();
            let mut total_time = ::std::time::Duration::ZERO;
            ::std::env::args()
                .skip(1)
                .flat_map(|s| s.parse::<u8>())
                .for_each(|day| match day {
                    $(
                        $day => {
                            let solution = $md::Solver::solve();
                            let day_time = solution.parse_time + solution.part1_time + solution.part2_time;
                            total_time += day_time;

                            if !suppress_output {
                                println!("Day {:02}", day);
                                println!("  - {}", $crate::ResultLine::note("Parsing", solution.parse_time));
                                println!("  - {}", $crate::ResultLine::solution(1, solution.part1_time, solution.part1));
                                println!("  - {}", $crate::ResultLine::solution(2, solution.part2_time, solution.part2));
                                println!("  - {}", $crate::ResultLine::note("Total", day_time));
                                println!();
                            }
                        }
                    ),+,
                    x => unimplemented!("Day {} is not yet implemented", x),
                });

                if !suppress_output {
                    let total_time = ::humantime::format_duration(total_time);
                    println!("Total time: {}", total_time);
                }
        }
    };
}
