use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::io::{self, BufRead, Write};
use signal_hook::iterator::Signals;
use libc::signal;
use libc::SIGINT;
use libc::SIGPIPE;
use libc::SIG_IGN;
use std::env;
use std::thread;

fn main() -> Result<(), std::io::Error> {
    // Check for --version flag
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--version" {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

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

    // Parse the input into a list of options, excluding empty lines
    let options: Vec<String> = input_text
        .lines()
        .map(|line| line.trim().to_string()) // Trims whitespace from each line
        .filter(|line| !line.is_empty()) // Excludes empty lines
        .collect();

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
