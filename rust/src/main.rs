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
    // ensure that the write operation is atomic (i.e., not interrupted by another write operation).
    let mut stdout = stdout.lock();

    for file in files {
        let path = Path::new(&file);
        if path.exists() {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if options.number_nonempty_lines {
                    write_line(&mut stdout, &line, &mut buffer, true)?;
                } else {
                    write_line(&mut stdout, &line, &mut buffer, false)?;
                }
            }
        } else {
            writeln!(stdout, "cat: {}: No such file or directory", file)?;
        }
    }

    Ok(())
}

struct Options {
    number_nonempty_lines: bool,
}

impl Options {
    fn new() -> Options {
        Options {
            number_nonempty_lines: false,
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
    number_line: bool,
) -> io::Result<()> {
    buffer.clear();
    if number_line && !line.trim().is_empty() {
        writeln!(stdout, "{}: {}", 1, line)?;
    } else {
        writeln!(stdout, "{}", line)?;
    }
    stdout.flush()
}

// 