extern crate termion;
use std::error::Error;
use std::path::Path;
pub fn builtin_cmd(argv: &Vec<&str>) -> bool {
    match argv[0] {
        "exit" => {
            std::process::exit(0);
        }
        "cd" => {
            cd(argv).unwrap_or_else(|error| {
                println!("Error: {}", error);
            });
            true
        }
        "fg" | "bg" => {
            true
        }
        "jobs" => {
            true
        }
        "clear" => {
            print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1)); 
            true
        }
        _ => {false}
    }
}

pub fn cd(argv: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let path = Path::new(argv[1]);
    std::env::set_current_dir(path)?;
    Ok(())
}
