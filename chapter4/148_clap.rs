use clap::{Command, Arg};

fn main() {
    let matches = Command::new("My RPN Program")
        .version("1.0.0")
        .author("Kaito Watanabe")
        .about("Super awesome sample RPN calclator")
        .arg(
            Arg::new("formula file")
                .help("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .help("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches();

    match matches.value_of("formula file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified")
    }
    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
