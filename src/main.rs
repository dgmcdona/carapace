extern crate termion;
extern crate dirs;

mod prompt;
mod builtin;

fn main() -> std::io::Result<()> {
    loop {
        prompt::print_prompt()?;
        let mut cmdline = String::new();
        let cmdline_result = std::io::stdin().read_line(&mut cmdline);
        match cmdline_result {
            Ok(_n) => {
            }
            Err(e) => {
                println!("Error parsing command line: {}", e);
            }
        }
        let argv: Vec<&str> = cmdline.split_whitespace().collect();
        if !builtin::builtin_cmd(&argv){

        }
    }
}


/*

fn sigchld_handler(){

}

fn sigtstp_handler(){

}

fn sigint_handler(){

}

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

fn parseline(){

}

*/
