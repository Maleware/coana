use std::{fs, io};
use clap::{App, Arg};
use crate::{types::{Deck, CEResult}, logic::database, statistic::tutor};
use crate::statistic::basic;

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
                println!("Basic Statistics: ");

                println!("\n Detected combos({}):", &basics.combo.len());
                for combo in &basics.combo {
                    println!("{:?}", *combo);
                }

                println!("\nCreatures: {}", basics.cardtype.creatures.len());
                for item in basics.cardtype.creatures {
                    println!("{}", item.name);
                }
                println!("\nArtifacts: {}", basics.cardtype.artifacts.len());
                for item in basics.cardtype.artifacts {
                    println!("{}", item.name);
                } 
                println!("\nEnchantments: {}", basics.cardtype.enchantments.len());
                for item in basics.cardtype.enchantments {
                    println!("{}", item.name);
                } 
                println!("\nLands: {}", basics.cardtype.lands.len());
                for item in basics.cardtype.lands {
                    println!("{}", item.name);
                } 
                println!("\nInstants: {}", basics.cardtype.instants.len());
                for item in basics.cardtype.instants {
                    println!("{}", item.name);
                } 
                println!("\nSorcerys: {}", basics.cardtype.sorcerys.len());
                for item in basics.cardtype.sorcerys {
                    println!("{}", item.name);
                }  
                println!("\nPlaneswalkers: {} \n", basics.cardtype.planeswalkers.len());
                for item in basics.cardtype.planeswalkers {
                    println!("{}", item.name);
                }
                for (key, card) in basics.mana_cost {
                    println!("Mana Cost: {} Number:{:?} \n", key, card.len());
                }
                for (colour, pips) in basics.mana_dist.manacost {
                    println!("Colour: {:?}, Pips: {}",colour,pips )
                }
                println!("\n");
                for (colour, pips) in basics.mana_dist.manaprod {
                    println!("Colour: {:?}, Producing: {}", colour, pips);
                }

                println!("\nDorks:");
                for dork in basics.mana_dist.dorks {
                    println!("{}", dork.name);
                }
                println!("\nArtifacts:");
                for artifact in basics.mana_dist.artifacts {
                    println!("{}", artifact.name);
                }
                println!("\nEnchantment:");
                for enchantment in basics.mana_dist.enchantments {
                    println!("{}", enchantment.name);
                }
                println!("\nLands:");
                for land in basics.mana_dist.lands {
                    println!("{}", land.name);
                }

                println!("\nDraws:");
                for draw in basics.effect.draw {
                    println!("{}", draw.name);
                }
                println!("\nBounce:");
                for bounce in basics.effect.bounce {
                    println!("{}", bounce.name);
                }
                println!("\nRemoval:");
                for removal in basics.effect.removal {
                    println!("{}", removal.name);
                }  
                println!("\nBoardwipe:");
                for boardwipe in basics.effect.boardwipe {
                    println!("{}", boardwipe.name);
                } 
                println!("\nLord:");
                for lord in basics.effect.lord {
                    println!("{}", lord.name);
                } 
                println!("\nCounter:");
                for counter in basics.effect.counter {
                    println!("{}", counter.name);
                } 
                println!("\nPayoff:");
                for payoff in basics.effect.payoff {
                    println!("{}", payoff.name);
                } 
                println!("\nRecursion:");
                for recursion in basics.effect.recursion {
                    println!("{}", recursion.name);
                } 
                println!("\nReanimation:");
                for rean in basics.effect.reanimation {
                    println!("{}", rean.name);
                }
                println!("\nStax:");
                for stax in basics.effect.stax {
                    println!("{}", stax.name);
                }
                println!("\nFast Mana:");
                for fastmana in basics.effect.fastmana {
                    println!("{}", fastmana.name);
                }
                println!("Tutorlinking: \n"); 
                for (tutor,link) in tutors {
                    println!("\n Targets for {} : \n", &tutor);
                    for card in link {
                        println!("{}",card.name);
                    }
                }
                for card in &t.library {
                    if card.name == String::from("Ghostly Flicker") {println!("{:?}", card)}
                }
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
