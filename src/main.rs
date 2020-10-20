use structopt::StructOpt;

/// Creates a new development task
#[derive(StructOpt)]
struct Cli {
    /// The task text
    task: String,    
}

fn main() {
    let args = Cli::from_args();
    println!("Task: {}", args.task)
}
