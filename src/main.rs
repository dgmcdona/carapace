extern crate termion;
extern crate dirs;
extern crate signal_hook;
extern crate libc;

mod prompt;
mod builtin;
mod command;

use command::{ CommandType };

fn main() -> std::io::Result<()> {
    loop {
        prompt::print_prompt()?;
        let mut cmdline = String::new();
        std::io::stdin().read_line(&mut cmdline)
            .expect("Error reading command line.");
        eval(&cmdline[..]);
    }
}

fn eval(cmdline: &str) {
    let command = command::parse_command(cmdline).unwrap();
    match command {
        CommandType::Builtin(_) => {

        },
        CommandType::Foreground(_) => {

        },
        CommandType::Background(_) => {

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
