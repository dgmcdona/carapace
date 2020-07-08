extern crate termion;
extern crate dirs;
extern crate libc;
mod prompt;
mod builtin;
mod command;
mod jobs;

use std::ffi::OsStr;
use command::{ CommandType };
use std::process::{ Command };
use crate::jobs::{ Job, JobList, JobType };

fn main() -> Result<(), Box<dyn std::error::Error>> {

    loop {
        prompt::print_prompt()?;
        let mut job_list = JobList::new();
        let mut cmdline = String::new();
        std::io::stdin().read_line(&mut cmdline)
            .expect("Error reading command line.");
        eval(&cmdline, &mut job_list);
    }
}

fn eval(cmdline: &str, job_list: &mut JobList) {
    let command = command::parse_command(cmdline).unwrap();
    match &command {
        CommandType::Builtin(_) => {
            return
        },
        CommandType::Foreground(argv) => {
            let mut child = Command::new(OsStr::new(&argv[0]))
                .args(argv[1..].into_iter())
                .spawn()
                .expect("Error running foreground job.");
            child.wait().expect("Failed to wait for child.");
            job_list.add_fg(Job::new(command, JobType::Foreground, child));
        },
        CommandType::Background(argv) => {
            let child = Command::new(OsStr::new(&argv[0]))
                .args(argv[1..].into_iter())
                .spawn()
                .expect("Error running background job.");
            job_list.add_bg(Job::new(command, JobType::Background, child));
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
