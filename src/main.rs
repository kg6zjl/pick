use dialoguer::Select;

fn main() {
    // Read input from stdin (piped input)
    let input_text = read_input_from_stdin();

    // Parse the input into a list of options
    let options: Vec<&str> = input_text.split('\n').collect();

    // Prompt the user to select an option
    let selected_option = Select::new()
        .items(&options)
        .default(0) // Set the default selection (optional)
        .interact()
        .unwrap();

    // Print the selected option
    println!("{}", options[selected_option]);

    // You can now pass the selected option to the next part of your pipeline
}

use std::io::{self, BufRead};

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
