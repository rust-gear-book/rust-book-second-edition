use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]
struct Cli {
    /// Sets the level of verbosity
    #[arg(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[arg(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    println!("Is verbosity specified?: {}", cli.verbose);
}
