extern crate termion;
extern crate dirs;
extern crate libc;
extern crate tokio;
extern crate tokio_signal;

mod prompt;
mod builtin;
mod command;

use std::ffi::OsStr;
use command::{ CommandType };
use tokio::prelude::*;
use tokio::process::Command;
use tokio::signal::unix::{ signal, SignalKind };

const SIGTSTP: i32 = 20;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Set up signal handling streams;
    let mut sigtstp_stream = signal(SignalKind::from_raw(SIGTSTP))?;

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
