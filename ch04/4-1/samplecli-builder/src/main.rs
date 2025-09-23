use clap::{ArgAction, Command, arg};

fn main() {
    let matches = Command::new("My RPN program")
        .version("1.0.0")
        .author("Your name")
        .about("Super awesome sample RPN calculator")
        .arg(arg!([FILE] "Formulas written in RPN").required(false))
        .arg(arg!(-v --verbose ... "Sets the level of verbosity").action(ArgAction::SetTrue))
        .get_matches();

    match matches.get_one::<String>("FILE") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    let verbose = matches.get_flag("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
