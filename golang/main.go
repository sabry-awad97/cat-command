package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"os"
)

func main() {
	// Define command line flags
	numberLines := flag.Bool("n", false, "number lines")
	numberNonblankLines := flag.Bool("b", false, "number non-blank lines")

	// Parse command line flags
	flag.Parse()

	// Get list of file names from command line arguments
	files := flag.Args()
	if len(files) == 0 {
		files = []string{"-"}
	}

	// Iterate over each file
	for _, file := range files {
		// Open file or use stdin
		var reader io.Reader
		if file == "-" {
			reader = os.Stdin
		} else {
			f, err := os.Open(file)
			if err != nil {
				fmt.Fprintf(os.Stderr, "Failed to open %s: %v\n", file, err)
				continue
			}
			defer f.Close()
			reader = f
		}

		// Create a scanner to read the file line by line
		scanner := bufio.NewScanner(reader)

		// Initialize the line number.
		lineNumber := 0

		// Initialize a mutable counter variable to hold the line number
		lastNum := 0

		// Iterate over each line from the scanner
		for scanner.Scan() {
			// Get the current line of text
			line := scanner.Text()

			// Increment the line number.
			lineNumber++

			// Check if the user wants line numbers
			if *numberLines {
				// If so, print the current line number followed by a tab character and then the line of text
				fmt.Printf("%6d\t%s\n", lineNumber, line)
			} else if *numberNonblankLines {
				// Handle printing line numbers for non-blank lines
				if line != "" {
					// If the line is not empty, increment lastNum and print the output
					lastNum++
					fmt.Printf("%6d\t%s\n", lastNum, line)
				} else {
					// If the line is empty, print a blank line
					fmt.Println()
				}
			} else {
				// If there are no numbering options, print the line
				fmt.Println(line)
			}
		}

		// Check for scanner errors
		if err := scanner.Err(); err != nil {
			fmt.Fprintln(os.Stderr, err)
		}
	}
}
