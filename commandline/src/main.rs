use ansi_term::{Colour, Style};
use clap::{arg, command, Command};

fn main() {
    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );

    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );

    println!(
        "{}, {} and {}",
        Colour::Yellow.paint("This is colored"),
        Style::new().bold().paint("this is bold"),
        Colour::Yellow.bold().paint("this is bold and colored")
    );

    let _matchs = command!()
        .arg_required_else_help(true)
        .arg(arg!([name] "Optional name to operate on"))
        .arg(arg!(-c --config <FILE> "Set a custom config file").required(false))
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(Command::new("test").about("Test sub command").arg(arg!(
            -l --list "List the test values"
        )))
        .get_matches();
}
