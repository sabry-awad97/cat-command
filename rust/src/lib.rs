// Import the Error trait for representing error values.
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, Arg};

// Create a Result to represent an Ok value for any type T or some Err value that implements the Error trait
type MainResult<T> = Result<T, Box<dyn Error>>;

// The derive macro adds the Debug trait so the struct can be printed
#[derive(Debug)]
// Define a public struct called Config
pub struct Config {
    // The files will be a vector of strings
    files: Vec<String>,

    // This is a Boolean value to indicate whether or not to print the line numbers
    number_lines: bool,

    // This is a Boolean to control printing line numbers only for nonblank lines
    number_nonblank_lines: bool,
}

// Define a public (pub) function that returns either Ok containing the unit type () or some error Err.
pub fn run(config: Config) -> MainResult<()> {
    for filename in config.files {
        match open(&filename) {
            // Print the filename and error when there is a problem opening a file.
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                // Initialize a mutable counter variable to hold the line number.
                let mut last_num = 0;

                // Iterate over each line_result value from BufRead::lines.
                // The tuple values from Iterator::enumerate can be unpacked using pattern matching.
                for (line_num, line_result) in file.lines().enumerate() {
                    // Either unpack an Ok value from line_result or propagate an error
                    let line = line_result?;

                    // Check if the user wants line numbers.
                    if config.number_lines {
                        // If so, print the current line number in a right-justified field six characters wide
                        // followed by a tab character and then the line of text.
                        // Numbering from enumerate starts at 0, so add 1 to mimic cat, which starts at 1
                        println!("{:6}\t{}", line_num + 1, line);

                        // Handle printing line numbers for nonblank lines.
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            // If the line is not empty, increment last_num and print the output.
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line);
                        } else {
                            // If the line is empty, print a blank line.
                            println!();
                        }
                    } else {
                        // If there are no numbering options, print the line.
                        println!("{}", line);
                    }
                }
            }
        }
    }

    // Return an indication that the function ran successfully
    Ok(())
}

// The function will accept a filename and will return either an error or a boxed
// value that implements the BufRead trait.
fn open(filename: &str) -> MainResult<Box<dyn BufRead>> {
    match filename {
        // When the filename is a dash (-), read from std::io::stdin.
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),

        // Otherwise, use File::open to try to open the given file or propagate an error
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MainResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Dr Sabry")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .min_values(1)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .takes_value(false)
                // It cannot occur in conjunction with -b
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    // Return an Ok variant containing a Config using the supplied values.
    Ok(Config {
        files: matches
            .get_many::<String>("files")
            // Because at least one value is required, it should be safe to call Option::unwrap
            .unwrap()
            .map(|e| e.into())
            .collect::<Vec<_>>(),
        number_lines: matches.contains_id("number"),
        number_nonblank_lines: matches.contains_id("number_nonblank"),
    })
}

// cargo run -q -- -n (Get-ChildItem .\tests\inputs\*.txt)