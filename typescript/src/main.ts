import * as fs from 'fs';
import * as readline from 'readline';
import { program } from 'commander';

// Define the options for the program
interface Options {
  numberLines: boolean;
  numberNonblankLines: boolean;
}

export async function run() {
  // Define command line options
  program
    .option('-n, --number', 'number lines')
    .option('-b, --number-nonblank', 'number non-blank lines')
    .parse(process.argv);

  // Get list of file names from command line arguments
  let files = program.args;
  if (files.length === 0) {
    files = ['-'];
  }

  // Get the options from the command object
  const opts = program.opts<{
    number: boolean;
    'number-nonblank': boolean;
  }>();

  const options: Options = {
    numberLines: opts.number,
    numberNonblankLines: opts['number-nonblank'],
  };

  // Iterate over each file
  for (const file of files) {
    // Open file or use stdin
    let reader: readline.Interface;
    if (file === '-') {
      reader = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
        terminal: false,
      });
    } else {
      try {
        reader = readline.createInterface({
          input: fs.createReadStream(file),
          output: process.stdout,
          terminal: false,
        });
      } catch (err) {
        console.error(`Failed to open ${file}: ${err}`);
        continue;
      }
    }

    // Initialize a mutable counter variable to hold the line number
    let lastNum = 0;

    // Iterate over each line from the reader
    for await (const line of reader) {
      // Check if the user wants line numbers
      if (options.numberLines) {
        // If so, print the current line number followed by a tab character and then the line of text
        console.log(`${lastNum.toString().padStart(6, ' ')}\t${line}`);
        lastNum++;
      } else if (options.numberNonblankLines) {
        // Handle printing line numbers for non-blank lines
        if (line.trim() !== '') {
          // If the line is not empty, increment lastNum and print the output
          lastNum++;
          console.log(`${lastNum.toString().padStart(6, ' ')}\t${line}`);
        } else {
          // If the line is empty, print a blank line
          console.log();
        }
      } else {
        // If there are no numbering options, print the line
        console.log(line);
      }
    }
  }
}

const main = async () => {
  try {
    await run();
  } catch (error: any) {
    console.log(error.message);
    process.exit(1);
  }
};

main();

// tsc
// node . --number file.txt
// node . --number file1.txt file2.txt file3.txt
// echo "Hello, world!" | node . --number -
// node . --number-nonblank file.txt
