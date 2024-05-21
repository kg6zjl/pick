use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::io::{self, BufRead, Write};
use libc::signal;
use libc::SIGPIPE;
use libc::SIG_IGN;
use std::env;

fn main() {
    // Check for --version flag
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--version" {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // Allow unsafe because we close the pipe
    unsafe {
        signal(SIGPIPE, SIG_IGN);
    }

    // Read input from stdin (piped input)
    let input_text = read_input_from_stdin();

    // Parse the input into a list of options
    let options: Vec<&str> = input_text.split('\n').collect();

    // Prompt the user to select an option using fuzzy search
    let selected_option = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&options)
        .default(0) // Set the default selection (optional)
        .interact()
        .unwrap();

    // Print the selected option
    println!("{}", options[selected_option]);
    // Flush stdout
    io::stdout().flush().unwrap();

    // You can now pass the selected option to the next part of your pipeline
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
