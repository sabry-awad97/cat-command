fn main() {
    // If the catr::get_args function returns an Ok(config) value, use Result
    // ::and_then to pass the config to catr::run.
    if let Err(e) = catr::get_args().and_then(catr::run) {
        // error print line
        eprintln!("{}", e);

        // Exit the program with a nonzero value to indicate an error.
        std::process::exit(1);
    }
}