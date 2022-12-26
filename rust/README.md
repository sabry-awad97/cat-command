# Cat Command

<style>code { font-family: monospace; background-color: #eee; padding: 2px 4px; border-radius: 3px; }</style>

The `cat` command is a command-line utility that is used to concatenate and display files. It is one of the most basic and frequently used commands in Unix-like operating systems.

Here are some examples of how to use the `cat` command:

1. Display the contents of a file:  
    `cat file.txt`
2. Concatenate the contents of multiple files and display the result:  
    `cat file1.txt file2.txt file3.txt`
3. Create a new file by concatenating the contents of multiple files:  
    `cat file1.txt file2.txt > newfile.txt`
4. Append the contents of one file to another:  
    `cat file1.txt >> file2.txt`
5. Display the contents of a file, one page at a time:  
    `cat -n file.txt | less`
6. Display the contents of a file in reverse order:  
    `cat file.txt | tac`
7. Concatenate the contents of multiple files and send the output to the printer:  
    `cat file1.txt file2.txt | lpr`

The `cat` command has many options and can be used in a variety of ways. For more information, you can type `man cat` at the command prompt to view the manual page for the `cat` command.
