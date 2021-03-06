use std::{fs, io};
use clap::{App, Arg};
use crate::{types::{Deck, CEResult}, logic::{database}, statistic::tutor};
use crate::statistic::basic;
use crate::statistic::archetype;

mod types;
mod import;
mod logic;
mod statistic;

pub fn check_database(offline: bool, verbose: bool) {
     
    if !offline {
        println_verbose!(verbose, "Online - Mode active, checking on updates");
        database::update();
        match import::combo::update() {
            Ok(_) => println!("Combo-Database up to date"),
            Err(e) => println!("{}", e),  
        }
    } else {
        println_verbose!(verbose, "Offline - Mode active, checking on data correct and existing.");
        logic::database::load().expect("Can not load databases, fatal in offline modus");
        
    }
}
pub fn check_deck (offline: bool, verbose: bool, input: String) -> CEResult<Deck> {
    match Deck::load(&input, verbose) {
        Ok(t) => {
           return Deck::check(t, verbose, offline);
        },
        Err(_) => {
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
pub fn load_register(offline: bool, verbose: bool, input: String) -> CEResult<Vec<Deck>> {
    let entries = fs::read_dir(&input).expect("Can not open path")
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>().expect("Can not collect entries");


    let mut decklists: Vec<String> = Vec::new();
    let mut decks: Vec<Deck> = Vec::new();

    for path in entries {
        println_verbose!(verbose, "Queue: {:?}", &path);
        decklists.push(path.into_os_string().into_string().expect("Path not UFT8 formated").replace(&input, "")); 
    }

    for decklist in decklists {
        println_verbose!(verbose, "Make deck of {:?}", &decklist);
        match check_deck(offline, verbose,  decklist.to_string()) {
            Ok(t) => decks.push(t),
            Err(e) => println!("Can not build {} because {}", &decklist, e),
        }
    }
    
    Ok(decks)

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
    

    if register {
        match load_register(offline, verbose, input.to_string()) {
            Ok(t) => {
                for deck in &t {
                    Deck::save(deck);
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    } else {
         // passing check_deck, struct Deck is complete and correct
        match check_deck( offline, verbose, input.to_string()) {
            Ok(t) => {
                println!("Deck name: {}", t.name);
                let basics = basic::Basic::new(&t);
                let tutors = tutor::tutor(&t);
                basics.println();


        
                println!("\n Tutorlinking: \n"); 
                for (tutor,link) in &tutors.tutor {
                    println!("\n Targets for {} : \n", &tutor);
                    for card in link {
                        println!("{}",card.name);
                    }
                }
                println!("\n Fetches: \n");
                for (fetch, link) in &tutors.fetches {
                    println!("\n Targets for {}: \n", &fetch);
                    for card in link {
                        println!("{}", card.name);
                    }
                }
                println!("\n Landramp: \n");
                for (ramp, link) in &tutors.landramp {
                    println!("\n Targets for {}: \n", &ramp);
                    for card in link {
                        println!("{}", card.name);
                    }
                }
                

                let consistency = archetype::from(&t, &basics,  tutors); 
                
                consistency.println();

                Deck::save(&t);  
            },
            Err(e) => println!("Error: {}", e),
        }
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
