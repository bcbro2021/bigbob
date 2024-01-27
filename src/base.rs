use std::{io::Write, thread, time::Duration};

pub fn input(text: &str) -> String {
    let mut buff: String = String::new();

    // Print a prompt to the console
    print!("{text}");
    // Flush the output to make sure the prompt is displayed
    std::io::stdout().flush().unwrap();

    // Read a line of input from the user and store it in the String
    std::io::stdin().read_line(&mut buff).expect("Failed to read line");

    // Trim the newline character from the end of the input
    let trimmedbuff: String = buff.trim().to_string();

    return trimmedbuff;
}

pub fn clear_terminal() {
    // Print the ANSI escape code to clear the terminal screen
    print!("\x1B[2J");

    // Move the cursor to the top-left corner (optional)
    print!("\x1B[H");

    // Flush the output to make sure it appears immediately
    std::io::stdout().flush().unwrap();
}

pub fn prettyprint(text: &str, time: u32) {
    print!("\n");
    for char in text.chars() {
        print!("{}",char);
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(time as u64));
    }
    print!("\n\n");
}