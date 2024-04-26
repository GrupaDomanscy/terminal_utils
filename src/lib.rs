use std::{env::{self}, io::{stdout, Write}};

pub fn get_terminal_size() -> Result<(usize, usize), String> {
    if let Some((w, h)) = term_size::dimensions() {
        return Ok((w, h));
    } else {
        Err("unable to get terminal size".to_string())
    }
}

pub fn clear_terminal_line() -> Result<(), String> {
    let terminal_size = get_terminal_size()?;

    print!("\r");

    for _ in 0..terminal_size.0 {
        print!(" ");
    }

    print!("\r");

    stdout().flush().unwrap();

    return Ok(());
}

pub fn get_executable_name() -> String {
    env::args()
        .collect::<Vec<_>>()[0]
        .clone()
}

pub fn get_cmd_args() -> Vec<String> {
    return env::args()
        .collect::<Vec<String>>()
        .into_iter()
        .skip(1)
        .collect::<Vec<_>>();
}
