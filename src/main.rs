use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use clap::{Arg, ArgMatches, Command};
use std::io::{self, BufRead, Write};
use signal_hook::iterator::Signals;
use libc::signal;
use libc::SIGINT;
use libc::SIGPIPE;
use libc::SIG_IGN;
use std::env;
use std::thread;

fn main() -> Result<(), std::io::Error> {
    // Call the args handler
    let matches = args_handler();

    // Set up the signal handler
    let mut signals = Signals::new(&[SIGINT]).unwrap();
    thread::spawn(move || {
        for sig in signals.forever() {
            if sig == SIGINT {
                // Clean exit code
                std::process::exit(0);
            }
        }
    });

    // Allow unsafe because we close the pipe
    unsafe {
        signal(SIGPIPE, SIG_IGN);
    }

    // Read input from stdin (piped input)
    let input_text = read_input_from_stdin();

    // Default to newline unless delimiter arg is passed
    let binding = "\n".to_string();
    let delimiter = matches.get_one::<String>("delimiter").unwrap_or(&binding);

    // Parse the input into a list of options, excluding empty lines
    let options: Vec<String> = input_text
        .split(delimiter)
        .map(|line| line.trim().to_string()) // Trims whitespace from each line
        .filter(|line| !line.is_empty()) // Excludes empty lines
        .collect();
    
    // // Parse the input into a list of options, excluding empty lines
    // let options: Vec<String> = input_text
    //     .lines()
    //     .map(|line| line.trim().to_string()) // Trims whitespace from each line
    //     .filter(|line| !line.is_empty()) // Excludes empty lines
    //     .collect();

    // Prompt the user to select an option using fuzzy search
    let selected_option = match FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&options)
        .default(0) // Set the default selection (optional)
        .max_length(10)
        .vim_mode(true)
        .interact() {
            Ok(selected) => selected,
            Err(_e) => {
                // fail silently if SIGINT received while making a selection
                return Ok(()); // Exit the program or handle the error as needed
            }
    };

    // Print the selected option
    println!("{}", options[selected_option]);

    // Flush stdout
    io::stdout().flush().unwrap();

    // Goodbye!
    Ok(())
}

fn args_handler() -> ArgMatches {
    let matches = Command::new("Pick")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Steve Arnett steven.arnett@protonmail.com")
        .about("Pick allows you to pipe in any newline separated data and waits for you to make your selection before passing your decision to the next tool in your piped command chain.")
        .arg(Arg::new("delimiter")
            .long("delimiter")
            .short('d')
            .help("Specify the delimiter (default is newline)")
            .value_name("DELIMITER")
        )
        .get_matches();

    return matches
}

fn read_input_from_stdin() -> String {
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
