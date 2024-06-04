extern crate libc;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use clap::{Arg, ArgMatches, Command};
use std::io::{self, BufRead, Write};
use std::fmt;
use signal_hook::iterator::Signals;
use libc::{signal, SIGINT, SIGPIPE, SIG_IGN};
use std::env;
use std::process;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if any arguments are provided or if stdin is a tty (i.e., there's no piped data)
    if env::args().len() == 1 && unsafe { libc::isatty(libc::STDIN_FILENO) } != 0 {
        println!("Please provide arguments or piped data. Exiting.");
        process::exit(0);
    }

    // Set up the signal handler
    let mut signals = Signals::new([SIGINT])?;
    let signals_handle = signals.handle();
    let _ = thread::spawn(move || {
        #[allow(clippy::never_loop)]
        for _sig in signals.forever() {
            signals_handle.close();
            // Exit the program immediately when SIGINT is received
            std::process::exit(0);
        }
    });

    // Allow unsafe because we close the pipe
    unsafe {
        signal(SIGPIPE, SIG_IGN);
    }

    // Call the args handler
    let matches = args_handler();

    // Read input from stdin (piped input)
    let mut input_text = read_input_from_stdin();

    // Default to newline unless delimiter arg is passed
    let binding = "\n".to_string();
    let delimiter = matches.get_one::<String>("delimiter").unwrap_or(&binding);

    let column = matches.get_one::<String>("column");
    if let Some(col_str) = column {
        let col_num = col_str.parse::<usize>()?;
        let col_index = if col_num > 0 { col_num - 1 } else { 0 };
        let parsed_opts = column_parser(&input_text, col_index)?;
        input_text = parsed_opts.join("\n");
    }

    // Sanitize inputs
    let options = sanitize_input(&input_text, delimiter)?;

    // Prompt and get the selection from the user
    let selection = selection_handler(&options)?;

    // Call the output handler with choice
    output_handler(&options[selection].to_string());

    // Flush stdout
    io::stdout().flush().unwrap();

    // Goodbye!
    std::process::exit(0);
}

fn output_handler(line: &str) {
    print!("{}", line);
}

fn column_parser(input_text: &str, column_num: usize) -> Result<Vec<String>, fmt::Error> {
    let mut result = Vec::new();
    for line in input_text.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() > column_num {
            result.push(words[column_num].to_string());
        }
    }
    Ok(result)
}

fn sanitize_input(input_text: &str, delimiter: &str) -> Result<Vec<String>, fmt::Error> {
    // Parse the input into a list of options, excluding empty lines
    let options: Vec<String> = input_text
        .split(delimiter)
        .map(|line| line.trim().to_string()) // Trims whitespace from each line
        .filter(|line| !line.is_empty()) // Excludes empty lines
        .collect();

    Ok(options)
}

fn selection_handler(options: &[String]) -> Result<usize, Box<dyn std::error::Error>> {
    // Prompt the user to select an option using fuzzy search
    let selected_option = match FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(options)
        .default(0) // Set the default selection (optional)
        .max_length(10)
        .vim_mode(true)
        .interact() {
            Ok(selected) => selected,
            Err(_e) => {
                // fail silently if SIGINT received while making a selection
                return Err(Box::new(std::fmt::Error)); // Exit the program or handle the error as needed
            }
    };

    Ok(selected_option)
}

fn args_handler() -> ArgMatches {
    Command::new("Pick")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Steve Arnett - www.github.com/kg6zjl")
        .about("Pick allows you to pipe in any newline or delimiter separated data and waits
for you to make your selection before passing your decision to
the next tool in your piped command chain.")
        .arg(Arg::new("delimiter")
            .long("delimiter")
            .short('d')
            .help("Specify the delimiter (default is newline)")
            .value_name("DELIMITER")
        )
        .arg(Arg::new("column")
        .long("column")
        .short('c')
        .help("Specify the column")
        .value_name("COLUMN")
        )
        .get_matches()
}

fn read_input_from_stdin() -> String {
    // Check if stdin is a tty (i.e., there's no piped data)
    if unsafe { libc::isatty(libc::STDIN_FILENO) } != 0 {
        println!("Please provide piped data. Exiting.");
        process::exit(0);
    }

    let mut input_text = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock(); // Lock stdin to read line by line
    while let Ok(line) = handle.read_line(&mut input_text) {
        if line == 0 {
            break; // End of input
        }
    }
    input_text
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_input_default() {
        let input_text = "apple\nbanana\n\npear\n";
        let delimiter = "\n";
        let result = sanitize_input(input_text, delimiter).unwrap();
        assert_eq!(result, vec!["apple", "banana", "pear"]);
    }

    #[test]
    fn test_sanitize_input_custom() {
        let input_text = "apple,banana,pear\n";
        let delimiter = ",";
        let result = sanitize_input(input_text, delimiter).unwrap();
        assert_eq!(result, vec!["apple", "banana", "pear"]);
    }

}
