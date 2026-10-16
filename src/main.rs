use std::env;
use std::io::Write;
use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];

    let contents = fs::read_to_string(filepath).expect("Filepath should be valid");
    let html_version = markdown::to_html(&contents);

    let mut tempfile = NamedTempFile::new()?;

    write!(tempfile,"{}",&html_version)?;

    Command::new("lynx").arg("-force_html").arg(tempfile.path()).spawn()?.wait()?;

    Ok(())
}
