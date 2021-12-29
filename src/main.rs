use clap::{App, Arg, ArgMatches};
use strum::IntoEnumIterator;

mod types;
mod import;

macro_rules! println_verbose {
    ($verbose:expr, $($arg:tt)*) => {
        if $verbose {
            println!($($arg)*);
        }
    };
}

fn main() {
    let args = get_app().get_matches();
    let verbose = args.is_present("verbose");
    let register = args.is_present("register");
    
    println_verbose!(verbose, "Verbose is active");

    for enums in types::CreatureSubtype::iter() {
        println!("Enum: {}", enums.to_string());
    }

    for zones in types::Zones::iter() {
        println!("Zone: {}", zones.to_string());
    }
}

fn get_app() -> App<'static, 'static>{
    App::new("mtg analyizer")
    .version("0.1")
    .author("Maximilian Wittich <maxi.wittich@outlook.com>")
    .about("Reads moxfield mtgo export lists to analyze the deck.")
    .arg(
        Arg::with_name("input")
        .required(true)
        .help("Path to decklist or folder")
        .index(1)
    )
    .arg(
        Arg::with_name("register")
        .short("r")
        .long("register")
        .help("Takes a path to a folder containing decklists")
    )
    .arg(
        Arg::with_name("verbose")
        .short("v")
        .long("verbose")
        .help("If set, verbose mode is activated")
    )


}
