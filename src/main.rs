use structopt::StructOpt;
use std::fs::OpenOptions;
use std::io::prelude::*;

/// Creates a new development task
#[derive(StructOpt)]
struct Cli {
    /// The task text
    task: String,    
}

fn main() {
    let args = Cli::from_args();
    println!("Task: {}", args.task);
    let todo: String = format!("{}\n",args.task);

    let exists = std::path::Path::new("test.txt").exists();

    if exists == false {
        std::fs::File::create("test.txt").expect("Could not create file");
    }
    
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("test.txt")
        .unwrap();
    
        if let Err(e) = writeln!(file, "{}", todo) {
            eprintln!("Couldn't write to file: {}", e);
        }

}
