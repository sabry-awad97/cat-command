import sys


def cat(filenames, options):
    # Iterate over each file
    for filename in filenames:
        # Open file or use stdin
        if filename == "-":
            f = sys.stdin
        else:
            try:
                f = open(filename)
            except OSError as err:
                print(f"Failed to open {filename}: {err}", file=sys.stderr)
                continue

        # Initialize a mutable counter variable to hold the line number
        last_num = 0

        # Iterate over each line from the file
        for line in f:
            # Check if the user wants line numbers
            if options["number_lines"]:
                # If so, print the current line number followed by a tab character and then the line of text
                print(f"{last_num:6}\t{line.rstrip()}")
                last_num += 1
            elif options["number_nonblank_lines"]:
                # Handle printing line numbers for non-blank lines
                if line.strip():
                    # If the line is not empty, increment last_num and print the output
                    last_num += 1
                    print(f"{last_num:6}\t{line.rstrip()}")
                else:
                    # If the line is empty, print a blank line
                    print()
            else:
                # If there are no numbering options, print the line
                print(line.rstrip())

        # Close the file
        f.close()


def run():
    # Parse command line arguments
    import argparse
    parser = argparse.ArgumentParser(
        description="Concatenate FILE(s), or standard input, to standard output.")
    parser.add_argument("files", metavar="FILE", nargs="*",
                        default=["-"], help="Input file(s)")
    parser.add_argument(
        "-n", "--number", action="store_true", help="number lines")
    parser.add_argument("-b", "--number-nonblank",
                        action="store_true", help="number non-blank lines")
    args = parser.parse_args()

    # Get the options from the command line arguments
    options = {
        "number_lines": args.number,
        "number_nonblank_lines": args.number_nonblank,
    }

    # Call cat function with the options and file names
    cat(args.files, options)


if __name__ == "__main__":
    try:
        run()
    except KeyboardInterrupt:
        sys.exit(1)
    except Exception as e:
        print(e)
        sys.exit(1)
