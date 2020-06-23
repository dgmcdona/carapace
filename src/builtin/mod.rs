extern crate termion;
extern crate dirs;
use std::error::Error;
use std::path::Path;

pub fn cd(argv: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let home = dirs::home_dir().unwrap();
    let mut path = home.as_path();
    if argv.len() > 1 {
        path = Path::new(argv[1]);
    }
    std::env::set_current_dir(path)?;
    Ok(())
}
