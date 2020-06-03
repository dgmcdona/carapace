use termion::{color};
use std::io::{Write};


pub fn print_prompt() -> std::io::Result<()>{
    let pwd = std::env::current_dir();
    let pwd = pwd.unwrap().into_os_string().into_string().unwrap();
    let home = dirs::home_dir();
    let home = home.unwrap().into_os_string().into_string().unwrap();
    let pwd = pwd.replace(&home, "~");
    print!("{}{} % {}", color::Fg(color::Blue), pwd, color::Fg(color::Reset));
    
    std::io::stdout().flush()?;
    Ok(())
}
