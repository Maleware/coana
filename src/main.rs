use clap::{App, Arg, ArgMatches};

use crate::{types::Deck, logic::database};

mod types;
mod import;
mod logic;

macro_rules! println_verbose {
    ($verbose:expr, $($arg:tt)*) => {
        if $verbose {
            println!($($arg)*);
        }
    };
}

fn main() {
    let args = get_app().get_matches();
    
    let input = args.value_of("input").unwrap_or("Error");
    let verbose = args.is_present("verbose");
    let register = args.is_present("register");
    let offline = args.is_present("offline");
    println_verbose!(verbose, "Verbose is active");

    if !offline {
        match import::scryfall::get_bulk() {
            Ok(_) => println!("Database downloaded"),
            Err(e) => println!("{}", e),    
        }
    }
    
    logic::database::update();

    match Deck::make(input.to_string()) {
        Ok(t) => {
            println!("Deck length: {}", t.library.len());
            for card in &t.library {
                println!("Deck: {:?}", (&card.0, &card.1.name));
            }
        },
        Err(e) => println!("Error: {}", e),
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
    .arg(
        Arg::with_name("offline")
        .short("o")
        .long("offline")
        .help("Only uses database for card import")
    )


}
