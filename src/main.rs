extern crate termion;
extern crate dirs;
extern crate signal_hook;
extern crate libc;
extern crate tokio;

mod prompt;
mod builtin;
mod command;

use std::ffi::OsStr;
use command::{ CommandType };
use tokio::process::Command;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    loop {
        prompt::print_prompt()?;
        let mut cmdline = String::new();
        std::io::stdin().read_line(&mut cmdline)
            .expect("Error reading command line.");
        eval(&cmdline[..]).await;
    }
}

async fn eval(cmdline: &str) {
    let command = command::parse_command(cmdline).unwrap();
    match command {
        CommandType::Builtin(_) => {
            return
        },
        CommandType::Foreground(_) => {
            println!("Foreground command detected");
            let mut cmd = Command::new(OsStr::new(cmdline));
            let exit_code = cmd.status().await;
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
