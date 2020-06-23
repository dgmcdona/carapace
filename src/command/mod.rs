use std::io::Write;
use std::io::stdout;
use crate::builtin;

pub enum CommandType<'a> {
    Builtin(Vec<&'a str>),
    Foreground(Vec<&'a str>),
    Background(Vec<&'a str>),
}

pub fn parse_command(argv: Vec<&str>) -> Result<CommandType, Box<dyn std::error::Error>> {
    match argv[0] {
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
        _ if argv[argv.len() -1] == "&" => {
            Ok(CommandType::Background(argv))
        },
        _ => {
            Ok(CommandType::Foreground(argv))
        }
    }
     
}
