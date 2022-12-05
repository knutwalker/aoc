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
            #[allow(clippy::used_underscore_binding)]
            {
                println!($($arg)*);
            }
        }
    };
}

trait MinDefault {
    fn min_default() -> Self;
}

trait MaxDefault {
    fn max_default() -> Self;
}

macro_rules! def_impl {
    ($($t:ty),+ $(,)?) => {
        $(
            impl MinDefault for $t {
                fn min_default() -> Self {
                    Self::MAX
                }
            }

            impl MaxDefault for $t {
                fn max_default() -> Self {
                    Self::MIN
                }
            }

        )+
    };
}

def_impl!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MinMax<T> {
    pub min: T,
    pub max: T,
}

impl<T: MinDefault + MaxDefault> Default for MinMax<T> {
    fn default() -> Self {
        Self {
            min: MinDefault::min_default(),
            max: MaxDefault::max_default(),
        }
    }
}

impl<A: Copy + Ord + MinDefault + MaxDefault> FromIterator<A> for MinMax<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        iter.into_iter().fold(MinMax::default(), |mut mn, x| {
            mn.max = mn.max.max(x);
            mn.min = mn.min.min(x);
            mn
        })
    }
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

pub trait Parse {
    type Out<'a>;
    fn parse_from(s: &str) -> Self::Out<'_>;
}

impl Parse for [u8] {
    type Out<'a> = &'a [u8];

    fn parse_from(s: &str) -> Self::Out<'_> {
        s.as_bytes()
    }
}

impl Parse for str {
    type Out<'a> = &'a str;

    fn parse_from(s: &str) -> Self::Out<'_> {
        s
    }
}

pub trait PuzzleInput
where
    Self: Sized,
{
    type Out<'a>;

    fn from_input(input: &str) -> Self::Out<'_>;
}

pub trait ProcessInput {
    type In: PuzzleInput;
    type Out<'a>;

    fn process(input: <Self::In as PuzzleInput>::Out<'_>) -> Self::Out<'_>;
}

impl PuzzleInput for () {
    type Out<'a> = Self;

    fn from_input(_input: &str) -> Self::Out<'_> {}
}

pub struct Blocks<T>(PhantomData<T>);

impl<T> PuzzleInput for Blocks<T>
where
    T: PuzzleInput,
{
    type Out<'a> = Vec<T::Out<'a>>;

    fn from_input(input: &str) -> Self::Out<'_> {
        input.split("\n\n").map(|l| T::from_input(l)).collect()
    }
}

pub struct StdFromStrParsing<T>(PhantomData<T>);

impl<T> PuzzleInput for StdFromStrParsing<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Out<'a> = Vec<T>;

    fn from_input(input: &str) -> Self::Out<'_> {
        lines(input).map(|l| T::from_str(l).unwrap()).collect()
    }
}

pub struct Parsing<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> PuzzleInput for Parsing<T>
where
    T: Parse,
{
    type Out<'a> = Vec<T::Out<'a>>;

    fn from_input(input: &str) -> Self::Out<'_> {
        lines(input).map(|l| T::parse_from(l)).collect()
    }
}

pub struct Post<T>(PhantomData<T>);

impl<T> PuzzleInput for Post<T>
where
    T: ProcessInput,
{
    type Out<'a> = T::Out<'a>;

    fn from_input(input: &str) -> Self::Out<'_> {
        let input = T::In::from_input(input);
        T::process(input)
    }
}

pub struct First<T>(PhantomData<T>);

impl<T> ProcessInput for First<T>
where
    T: PuzzleInput,
    for<'x> T::Out<'x>: PopFirst,
{
    type In = T;

    type Out<'a> = <T::Out<'a> as PopFirst>::Out;

    fn process(input: <T as PuzzleInput>::Out<'_>) -> Self::Out<'_> {
        <T::Out<'_> as PopFirst>::pop_first(input)
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
    fn parse_input(input: &str) -> <Self::Input as PuzzleInput>::Out<'_> {
        <Self::Input as PuzzleInput>::from_input(input)
    }

    fn run(
        input: <Self::Input as PuzzleInput>::Out<'_>,
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
        Self::new(format!("Part {part}"), duration, Some(Box::new(solution)))
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
        $crate::Parsing<$input_ty>
    };

    (parse $input_ty:ty) => {
        $crate::StdFromStrParsing<$input_ty>
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
        #[allow(clippy::redundant_pub_crate)]
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
                mut $input: <$input_ty as $crate::PuzzleInput>::Out<'_>,
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

                            println!("Day {:02}", day);
                            println!("  - {}", $crate::ResultLine::note("Parsing", solution.parse_time));
                            println!("  - {}", $crate::ResultLine::solution(1, solution.part1_time, solution.part1));
                            println!("  - {}", $crate::ResultLine::solution(2, solution.part2_time, solution.part2));
                            println!("  - {}", $crate::ResultLine::note("Total", day_time));
                            println!();
                        }
                    ),+,
                    x => unimplemented!("Day {} is not yet implemented", x),
                });

                let total_time = ::humantime::format_duration(total_time);
                println!("Total time: {}", total_time);
        }
    };
}
