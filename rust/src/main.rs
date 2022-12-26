use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const DEFAULT_BUF_SIZE: usize = 4096;

fn main() -> io::Result<()> {
    let mut options = Options::new();
    let files = parse_args(&mut options, env::args().skip(1));

    let mut buffer = Vec::with_capacity(DEFAULT_BUF_SIZE);
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut line_number = 1;
    let mut prev_blank = false;

    for file in files {
        let path = Path::new(&file);
        if path.exists() {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                let blank = line.trim().is_empty();
                if options.squeeze_blank_lines && prev_blank && blank {
                    continue;
                }
                if options.number_all_lines {
                    write_line(&mut stdout, &line, &mut buffer, line_number)?;
                    line_number += 1;
                } else {
                    write_line(&mut stdout, &line, &mut buffer, 0)?;
                }
                prev_blank = blank;
            }
        } else {
            writeln!(stdout, "cat: {}: No such file or directory", file)?;
        }
    }

    Ok(())
}

struct Options {
    number_nonempty_lines: bool,
    number_all_lines: bool,
    squeeze_blank_lines: bool,
}

impl Options {
    fn new() -> Options {
        Options {
            number_nonempty_lines: false,
            number_all_lines: false,
            squeeze_blank_lines: false,
        }
    }
}

fn parse_args(options: &mut Options, args: impl Iterator<Item = String>) -> Vec<String> {
    let mut files = Vec::new();

    for arg in args {
        if arg.starts_with("-") {
            for ch in arg[1..].chars() {
                match ch {
                    'b' => options.number_nonempty_lines = true,
                    'n' => options.number_all_lines = true,
                    's' => options.squeeze_blank_lines = true,
                    _ => {}
                }
            }
        } else {
            files.push(arg);
        }
    }

    files
}

fn write_line(
    stdout: &mut impl Write,
    line: &str,
    buffer: &mut Vec<u8>,
    line_number: usize,
) -> io::Result<()> {
    buffer.clear();
    if line_number > 0 {
        writeln!(stdout, "{}: {}", line_number, line)?;
    } else {
        writeln!(stdout, "{}", line)?;
    }
    stdout.flush()
}
