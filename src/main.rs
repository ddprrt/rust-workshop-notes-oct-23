use std::{io::Read, num::ParseIntError, path::PathBuf};

use ownership_example_usp::{
    clock::Clock, fibonacci::LimitedFibonacci, print::Printable, url_ordering::show_urls,
};

#[derive(Debug)]
enum FileMode {
    Append,
    Replace,
    FromStart,
}

impl Default for FileMode {
    fn default() -> Self {
        FileMode::Append
    }
}

#[derive(Debug)]
struct BufferLen(u64);

impl Default for BufferLen {
    fn default() -> Self {
        BufferLen(1024)
    }
}

#[derive(Default, Debug)]
struct Options {
    mode: FileMode,
    file: PathBuf,
    max_size: BufferLen,
}

fn write(options: Options) {
    // do something with those options
}

fn multiply_by_two_or_num<T, U>(num: T, other_num: U) -> i32
where
    T: Into<Option<i32>>,
    U: Into<Option<i32>>,
{
    num.into().unwrap_or(2) * other_num.into().unwrap_or(2)
}

fn read_username_from_file(file: &str) -> Result<String, String> {
    let mut file = std::fs::File::open(file).map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .map_err(|e| e.to_string())
        .map(|_| buffer)
}

#[derive(Debug)]
enum NumberFromFileError {
    ParseIntError(ParseIntError),
    IoError(std::io::Error),
}

impl std::fmt::Display for NumberFromFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NumberFromFileError::ParseIntError(e) => e.fmt(f),
            NumberFromFileError::IoError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for NumberFromFileError {}

impl From<ParseIntError> for NumberFromFileError {
    fn from(e: ParseIntError) -> Self {
        NumberFromFileError::ParseIntError(e)
    }
}

impl From<std::io::Error> for NumberFromFileError {
    fn from(e: std::io::Error) -> Self {
        NumberFromFileError::IoError(e)
    }
}

fn read_number_from_file(file: &str) -> Result<u32, NumberFromFileError> {
    let mut file = std::fs::File::open(file)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let num: u32 = buffer.trim().parse()?;
    Ok(num)
}

fn breaks() -> Result<(), Box<dyn std::error::Error>> {
    let num = read_number_from_file("a_file.txt")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num = match read_number_from_file("a_file.txt") {
        Ok(num) => num,
        Err(NumberFromFileError::ParseIntError(e)) => 0,
        Err(NumberFromFileError::IoError(e)) => 0,
    };

    let mut fib = vec![1, 1, 2, 3, 5, 8, 13, 21, 34];
    fib.iter_mut()
        .into_iter()
        .enumerate()
        .for_each(|(idx, el)| {
            *el = 0;
        });

    let mut iter = fib.iter_mut().into_iter();
    let item1 = iter.next();
    let item2 = iter.next();
    for (idx, i) in fib.iter_mut().into_iter().take(5).enumerate() {
        println!("{}", i);
        //fib.push(55);
    }

    multiply_by_two_or_num(2, 3);
    multiply_by_two_or_num(2, None);

    write(Options {
        mode: FileMode::Append,
        file: "main.txt".into(),
        max_size: BufferLen(1024),
    });

    let ops = Options {
        mode: FileMode::Append,
        file: "main.txt".into(),
        max_size: BufferLen(1024),
    };

    write(Options::default());
    write(Options {
        file: "main.txt".into(),
        ..Default::default()
    });

    let mut clock = Clock::new(12, 71);
    clock = clock + 100;
    let another_clock = Clock::new(1, 10);
    clock = clock + another_clock;
    println!("{clock}");

    for i in LimitedFibonacci::<i128>::default() {
        println!("{i}");
    }

    23.dbg();
    "Hello World".dbg();
    ops.dbg();

    Options::default().dbg();

    show_urls();

    Ok(())
}
