use std::env;
use std::fs;
use std::process::Command;

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];

    let contents = fs::read_to_string(filepath).expect("Filepath should be valid");
    let html_version = markdown::to_html(&contents);
    fs::write("temp.html", &html_version)?;

    Command::new("lynx").arg("temp.html").spawn()?.wait()?;
    Ok(())
}
