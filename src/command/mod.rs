extern crate tokio;

use std::io::Write;
use std::io::stdout;
use crate::builtin;

pub enum CommandType {
    Builtin(Vec<String>),
    Foreground(Vec<String>),
    Background(Vec<String>),
}

pub fn parse_command(cmdline: &str) -> Result<CommandType, Box<dyn std::error::Error>> {
    let mut argv: Vec<String> = cmdline.split_whitespace()
        .into_iter()
        .map(|arg| { String::from(arg) })
        .collect();
    match argv[0].as_ref() {
        "exit" => {
            std::process::exit(0);
        },
        "cd" => {
            builtin::cd(&argv).unwrap_or_else(|error| {
                println!("Error: {}", error);
                stdout().flush().ok();
            });
            Ok(CommandType::Builtin(argv))
        },
        "fg" | "bg" => {
            Ok(CommandType::Builtin(argv))
        },
        "jobs" => {
            Ok(CommandType::Builtin(argv))
        },
        "clear" => {
            print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1)); 
            Ok(CommandType::Builtin(argv))
        },
        _ if argv.contains(&String::from("&")) => {
            argv.retain(|arg| arg != "&");
            println!("{:?}", argv);
            Ok(CommandType::Background(argv))
        },
        _ => {
            Ok(CommandType::Foreground(argv))
        }
    }
     
}
