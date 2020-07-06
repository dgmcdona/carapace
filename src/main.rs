extern crate termion;
extern crate dirs;
extern crate libc;
mod prompt;
mod builtin;
mod command;

use std::ffi::OsStr;
use command::{ CommandType };
use std::process::{ Command };

fn main() -> Result<(), Box<dyn std::error::Error>> {

    loop {
        prompt::print_prompt()?;
        let mut cmdline = String::new();
        std::io::stdin().read_line(&mut cmdline)
            .expect("Error reading command line.");
        eval(&cmdline);
    }
}

fn eval(cmdline: &str) {
    let command = command::parse_command(cmdline).unwrap();
    match command {
        CommandType::Builtin(_) => {
            return
        },
        CommandType::Foreground(argv) => {
            println!("Foreground command detected");
            let mut child = Command::new(OsStr::new(argv[0]))
                .args(argv[1..].into_iter())
                .spawn()
                .expect("Error running foreground job.");
            child.wait().expect("Failed to wait for child.");
        },
        CommandType::Background(argv) => {
            let mut child = Command::new(OsStr::new(argv[0]))
                .args(argv[1..].into_iter())
                .spawn()
                .expect("Error running background job.");
        },
    }
}

/*
fn do_redirect(){

}

fn do_bgfg(){

}

fn addjob(){

}

fn deletejob(){

}

fn listjobs(){

}
*/
