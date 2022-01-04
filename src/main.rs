use clap::{App, Arg, ArgMatches};
use types::CEResult;

use crate::{types::{Deck}, logic::database};

mod types;
mod import;
mod logic;


pub fn check_database(offline: bool, verbose: bool) {
     
    if !offline {
        println_verbose!(verbose, "Online - Mode active, checking on updates");
        database::update();
        // import::combo::update().expect("Can not find or download combo");
    } else {
        println_verbose!(verbose, "Offline - Mode active, checking on data correct and existing.");
        logic::database::load().expect("Can not load databases, fatal in offline modus");
        // import::combo::load().expect("Can not load combo data, fatal in offline modus");
    }
}

pub fn check_deck (offline: bool, verbose: bool, input: String) -> CEResult<Deck> {
    match Deck::load(&input, verbose) {
        Ok(t) => {
           return Deck::check(t, verbose, offline);
        },
        Err(e) => {
            println!("Deck not saved, build new deck from {}", &input);
            match Deck::make(input) {
                Ok(t) => {
                    return Deck::check(t, verbose, offline);
                }, 
                Err(e) => Err(e),
            }
        },
    }
}



fn main() {
    let args = get_app().get_matches();
    
    let input = args.value_of("input").unwrap_or("Error");
    let verbose = args.is_present("verbose");
    let register = args.is_present("register");
    let offline = args.is_present("offline");
    println_verbose!(verbose, "Verbose is active");

    // update routine to load or check neccessary data
    check_database(offline, verbose);
    
    match check_deck( offline, verbose, input.to_string()) {
        Ok(t) => {
            println!("Deck name: {}", t.name);
            for card in &t.library {
                println!("Card: {} CMC: {}", &card.name, &card.mana_cost );
                println!("Zones: {:?} , Type: {:?} ,\n  Keys: {:?} Keywords: {:?}\n Cartypes in Oracle: {:?} \n Backside: {:?}",
                 card.zones, card.cardtype, card.keys, card.keywords, card.oracle_types, card.backside);
            }
            for commander in &t.commander{
                println!("\n Commander: {:?}", commander);
            }

            Deck::save(&t);  
        },
        Err(e) => println!("Error: {}", e),
    }
    
}

fn get_app() -> App<'static, 'static>{
    App::new("mtg analyizer")
    .version("0.1")
    .author("Maximilian Wittich <maxi.wittich@outlook.com>")
    .about("Reads moxfield mtgo export lists to analyze the deck. Please add *CMDR* in the line your commander is")
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
