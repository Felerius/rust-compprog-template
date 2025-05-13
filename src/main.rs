use std::{
    fmt::Debug,
    io::{self, BufWriter, StdoutLock, Write},
    str::FromStr,
};

fn main() {
    let mut io = io();

    // Example solution, adapt to your needs. The read functions from `io` read
    // whitespace separated values from stdin. Newlines are ignored/treated as
    // just another form of whitespace.
    let n: usize = io.read();
    let ans: u64 = io.read_n::<u64>(n).sum();

    // The `put!` and `putln!` macros correspond to `print!` and `println!`
    // macros but are more efficient when printing lots of output. If CLion
    // highlights them macros as errors, you can move the macro definitions from
    // the end of the file to the top. Both compile fine, it's just CLion being
    // weird.
    putln!(io, "{ans}");
}

fn io() -> Io<impl Iterator<Item = &'static str>> {
    // Leaking the input lines is a bit wasteful but generally fine in a
    // competitive programming context and avoids having to deal with lifetimes
    // in the input reading.
    let input = io::stdin().lines().flat_map(|line| {
        line.expect("failed to read input line")
            .leak()
            .split_whitespace()
    });
    Io(input, BufWriter::new(io::stdout().lock()))
}

struct Io<I>(I, BufWriter<StdoutLock<'static>>);

#[allow(dead_code)]
impl<I: Iterator<Item = &'static str>> Io<I> {
    fn read_str(&mut self) -> &'static str {
        self.0.next().expect("unexpected end-of-file")
    }

    fn read<T: FromStr<Err: Debug>>(&mut self) -> T {
        self.read_str().parse().expect("failed to parse input")
    }

    fn read_n<T: FromStr<Err: Debug>>(&mut self, n: usize) -> impl Iterator<Item = T> + '_ {
        (0..n).map(|_| self.read())
    }

    fn collect_n<T: FromStr<Err: Debug>, C: FromIterator<T>>(&mut self, n: usize) -> C {
        self.read_n(n).collect()
    }
}

#[macro_export]
macro_rules! putln {
    ($io:expr $(, $($args:tt)*)?) => {
        writeln!($io.1 $(, $($args)*)?).expect("failed to write output")
    };
}

#[macro_export]
macro_rules! put {
    ($io:expr, $($args:tt)*) => {
        write!($io.1, $($args)*).expect("failed to write output")
    };
}
