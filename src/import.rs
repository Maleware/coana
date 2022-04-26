/*********************************** User Input *******************************************/
pub mod user_import {
    use std::fs;
    use std::io;

    use crate::types::{CEerror, CEResult};

    pub fn decklist(filename: String) -> CEResult<Vec<String>>{
    println!("Load decklist {}", filename);

    let path = format!("decks/{}", filename);

  match fs::read_to_string(path){
        Ok(t) => return Ok( t.replace("\r", "")
            .split("\n")
            .flat_map(str::parse::<String>)
            .collect::<Vec<String>>() ),
        Err(_) => return Err(CEerror::FailImportDeck(String::from("Can not read from path"))),
    }; 
}
    pub fn quantity_card(decklist: &String) -> CEResult<Vec<String>> {
    let mut qty_card  = decklist.splitn(2, " ");
    let mut result = Vec::new();

    match qty_card.next() {
        Some(t) => {
            result.push(t.to_string());
            match qty_card.next() {
                Some(t) => {
                    result.push(t.to_string());
                    return Ok(result);
                },
                None => return Err(CEerror::FailImportDeck(String::from("Error getting Cardname. Empty string"))),
            }
        },
        None => return Err(CEerror::FailImportDeck(String::from("Error getting Quantity. Empty string"))),

    }
}


// Needed if user interaction is required
    pub fn user_input() -> Vec<String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    let input_vec = input.trim().split('-').flat_map(str::parse::<String>).collect::<Vec<String>>();
    println!("{:?}", &input_vec);
    input_vec
}
}
/********************************* Scryfall Import ****************************************/
pub mod scryfall {
    use reqwest::blocking;
    use serde_json::Value;
    use crate::types::{CEerror, CEResult};
    use crate::logic::database;

    pub fn get(cardname: &String) -> CEResult<String> {
        let request = match exact_request(cardname) {
            Ok(t) => {
                let v: Value = serde_json::from_str(&t).expect("No string found in request_card");
                if v["code"] == "not_found".to_string() {
                    println!("Card not found by name, try fuzzy request");
                    match fuzzy_request(cardname) {
                        Ok(t) => return Ok(t),
                        Err(e) => return Err(e),
                    };
                }
                Ok(t)
            },
            Err(_) => match fuzzy_request(cardname) {
                Ok(t) => Ok(t),
                Err(_) => Err(CEerror::CardNotFound),
            }
        };
        request
    }
    fn fuzzy_request(cardname: &String) -> CEResult<String> {
        let mut api = String::from("https://api.scryfall.com/cards/named?fuzzy=");
        api = format!("{}{}", api, make_fuzzy(cardname));
        let request = match
            blocking::get(api) {
                Ok(t) => match t.text() {
                    Ok(t) => Ok(t),
                    Err(_) => Err(CEerror::APIError),
                },
                Err(_) => Err(CEerror::APIError),
        };
        request
    }
    fn exact_request(cardname: &String) -> CEResult<String> {
        let mut api = String::from("https://api.scryfall.com/cards/named?exact=");
        api = format!("{}{}", api, *cardname);
    
        let request = match blocking::get(api) {
                Ok(t) => match t.text() {
                    Ok(t) => Ok(t),
    
                    Err(_) => Err(CEerror::APIError),
                },
                Err(_) => Err(CEerror::APIError),
            };
        request
    }
    fn make_fuzzy(cardname: &String ) -> String {

        let mut fuzzy_string = String::new();
        let mut i = 0;
    
        let vec_name = cardname
            .to_lowercase()
            .split(" ")
            .flat_map(str::parse::<String>)
            .collect::<Vec<String>>();
    
        for mut word in vec_name {
            let buffer: Vec<char> = word.chars().collect();
            let mut length = buffer.len();
    
            if length%2 != 0 {
                length = length + 1;
            }
    
            match length {
                1 => (),
                2 => (),
               // 3 => word = word[..2].to_string(),
                3 => (),
                4 => (),
                _ => word = word[..(length-( length/2 ) ) + 1 ].to_string(),
            };
    
            if i == 0 {
                fuzzy_string = word;
            } else {
                fuzzy_string = format!("{} {}", fuzzy_string, word);
            }
            i += 1;
        }
    
        fuzzy_string.replace(" ", "+")
    }
    pub fn get_bulk() -> CEResult<()> {
        let mut api = String::from("https://api.scryfall.com/bulk-data");

        println!("Downloading database from scryfall....");

        match blocking::get(api) {
            Ok(t) => match t.text() {
                Ok(t) => {
                    let v: Value = serde_json::from_str(&t).expect("Bulk-data frame was not retrieved");
                    if v["code"] == "not_found".to_string() {
                        println!("Bulk-Data temporally not available");
                    }
                    api = v["data"][0]["download_uri"].to_string().replace("\"", "");
                    
                    match blocking::get(api) {
                        Ok(t) => match t.text() {
                            Ok(t) => {
                                let v: Value = serde_json::from_str(&t).expect("Bulk-Data can not be formated to json");
                                if v["code"] == "not_found".to_string() {
                                    println!("Bulk-Data temporally not available");
                                }
                                return database::save(&t);
                            },
                            Err(_) => return Err(CEerror::APIError), 
                        }
                        Err(_) => return Err(CEerror::APIError), 
                    }
                },
                Err(_) => return Err(CEerror::APIError),
            }
            Err(_) => return Err(CEerror::APIError),
        }
    }
}
/********************************** Combo Import ******************************************/

/* This is a new combo import, Website: https://sheets.googleapis.com/v4/spreadsheets/1KqyDRZRCgy8YgMFnY0tHSw_3jC99Z0zFvJrPbfm66vA/values:batchGet?ranges=combos!A2:Q&key=AIzaSyBD_rcme5Ff37Evxa4eW5BFQZkmTbgpHew */

pub mod combo {
    use reqwest::blocking;
    use crate::types::{CEResult, CEerror, Deck};
    use serde_json::Value;
    use std::{fs::{self, *}, io::{prelude::*, BufReader}, time::{SystemTime, Duration}, ops::Add};
   
    pub fn search (deck: Deck) -> CEResult<()> { Ok(()) }
    
    pub fn load() -> CEResult<Vec<String>> {
        let mut contents = String::new();

        println!("Open combo data from system");

        match File::open("combo.txt") {
            Ok(t) => {
                let mut buf_reader = BufReader::new(t);
                buf_reader.read_to_string(&mut contents).expect("Can not open combo data");

                let result: Vec<String> = serde_json::from_str(&contents).expect("Json not properly build"); 

                println!("Combo data successfully opened"); 

                Ok(result) 
            },
            Err(_) => Err(CEerror::ComboError),
        }
    }

    pub fn update() -> CEResult<()>{

        match File::open("combo.txt") {
            Ok(_) => {
                let metadata = fs::metadata("combo.txt").expect("File found but can not open");
                let now = SystemTime::now();
                
                if let Ok(time) = metadata.modified() {
                    // Update every full day
                    if time.add(Duration::from_secs(86400)) <= now {
                        println!("File is older than a day: Update....");
                        match remove_file("combo.txt"){
                            Ok(_) => println!("Expired combo data removed..."),
                            Err(_) => println!("Can not remove old combo data..."),
                        }
                        match serde_json::to_writer(&File::create("combo.txt").expect("Can not save combos"), &get()?) {
                            Ok(_)=> return Ok(()),
                            Err(_) => return Err(CEerror::ComboError),
                        } 
                    }   
                }   
                Ok(())
            },
            Err(_) => {
                println!("combo.txt not found, create and download data");
                match serde_json::to_writer(&File::create("combo.txt").expect("Can not save combos"), &get()?) {
                    Ok(_)=> return Ok(()),
                    Err(_) => return Err(CEerror::ComboError),
                }
            },
        }
    }
    fn get() -> CEResult<Vec<Vec<String>>> { 
        let mut result = Vec::new();

        match request_combo() {
            Ok(combodata) => {
                let buffer = seperate_combos(combo_to_json(&combodata)); 
                
                for item in buffer {
                    let elements = item.split("\",\"")
                                        .flat_map(str::parse::<String>)
                                        .collect::<Vec<String>>();
                            // empty slots are length one
                            if elements.len() != 1 {
                                result.push(elements);
                            } 
                }

                return Ok(result);
            },
            Err(_) => return Err(CEerror::ComboError),
        }

    }
   
    fn request_combo() -> CEResult<String> {

        println!("Fetching available Combos...");

        let api = String::from("https://sheets.googleapis.com/v4/spreadsheets/1KqyDRZRCgy8YgMFnY0tHSw_3jC99Z0zFvJrPbfm66vA/values:batchGet?ranges=combos!A2:Q&key=AIzaSyBD_rcme5Ff37Evxa4eW5BFQZkmTbgpHew");

        let request = match
            blocking::get(api) {
                Ok(t) => match t.text() {
                    Ok(t) => Ok(t),
                    Err(_) => Err(CEerror::ComboError),
                },
                Err(_) => Err(CEerror::ComboError),
        };
        request
    }
    fn combo_to_json(request: &String) -> CEResult<Value> {

        match serde_json::from_str(request) {
            Ok(t) => Ok(t),
            Err(_) => Err(CEerror::ComboError),
        }
    }
    fn seperate_combos(json: CEResult<Value>) -> Vec<String> {

        let mut vec = Vec::new();

        match json {
            Ok(t) =>{
                vec = t["valueRanges"][0]["values"]
                    .to_string()
                    .trim()
                    .replace(",\"\"", "")
                    .split(",[")
                    .flat_map(str::parse::<String>)
                    .collect::<Vec<String>>();
            },
            Err(e) => println!("{}", e),
        }
        vec
    } 

}

/*

Not Avialable anymore... 01 / 03 / 22

pub mod combo {
    use reqwest::blocking;
    use crate::types::{CEResult, CEerror};
    use serde_json::Value;
    use std::{fs::{self, *}, io::{prelude::*, BufReader}, time::{SystemTime, Duration}, ops::Add};

    pub fn update() -> CEResult<()>{

        match File::open("combo.txt") {
            Ok(_) => {
                let metadata = fs::metadata("combo.txt").expect("File found but can not open");
                let now = SystemTime::now();
                
                if let Ok(time) = metadata.modified() {
                    // Update every full day
                    if time.add(Duration::from_secs(86400)) <= now {
                        println!("File is older than a day: Update....");
                        match remove_file("combo.txt"){
                            Ok(_) => println!("Expired combo data removed..."),
                            Err(_) => println!("Can not remove old combo data..."),
                        }
                        match serde_json::to_writer(&File::create("combo.txt").expect("Can not save combos"), &get()?) {
                            Ok(_)=> return Ok(()),
                            Err(_) => return Err(CEerror::ComboError),
                        } 
                    }   
                }   
                Ok(())
            },
            Err(_) => {
                println!("combo.txt not found, create and download data");
                match serde_json::to_writer(&File::create("combo.txt").expect("Can not save combos"), &get()?) {
                    Ok(_)=> return Ok(()),
                    Err(_) => return Err(CEerror::ComboError),
                }
            },
        }
    }
    pub fn load() -> CEResult<Vec<String>> {
        let mut contents = String::new();

        println!("Open combo data from system");

        match File::open("combo.txt") {
            Ok(t) => {
                let mut buf_reader = BufReader::new(t);
                buf_reader.read_to_string(&mut contents).expect("Can not open combo data");

                let result: Vec<String> = serde_json::from_str(&contents).expect("Json not properly build"); 

                println!("Combo data successfully opened"); 

                Ok(result) 
            },
            Err(_) => Err(CEerror::ComboError),
        }
    }
    pub fn get() -> CEResult<Vec<Vec<String>>> {
        let mut result = Vec::new();
        match request_combo() {
            Ok(t) => {
                match combo_to_json(&t) {
                    Ok(t) => {
                        let buffer = seperate_combos(Ok(t));
                        for item in buffer {

                            let elements = item.split("\",\"")
                                        .flat_map(str::parse::<String>)
                                        .collect::<Vec<String>>();
                            // empty slots are length three
                            if elements.len() != 3 {

                                result.push(elements);
                            }
                        }
                    },
                    Err(e)=> return Err(e),
                }
            }
            Err(e) => return Err(e),
        }
        Ok(result)
    }
    fn request_combo() -> CEResult<String> {

        println!("Fetching available Combos...");

        let api = String::from("https://sheets.googleapis.com/v4/spreadsheets/1JJo8MzkpuhfvsaKVFVlOoNymscCt-Aw-1sob2IhpwXY/values:batchGet?ranges=combos!A2:Q&ranges=utilities!C2&key=AIzaSyDzQ0jCf3teHnUK17ubaLaV6rcWf9ZjG5E");

        let request = match
            blocking::get(api) {
                Ok(t) => match t.text() {
                    Ok(t) => Ok(t),
                    Err(_) => Err(CEerror::ComboError),
                },
                Err(_) => Err(CEerror::ComboError),
        };
        request
    } 
    fn combo_to_json(request: &String) -> CEResult<Value> {

        match serde_json::from_str(request) {
            Ok(t) => Ok(t),
            Err(_) => Err(CEerror::ComboError),
        }
    }
    fn seperate_combos(json: CEResult<Value>) -> Vec<String> {

        let mut vec = Vec::new();

        match json {
            Ok(t) =>{
                vec = t["valueRanges"][0]["values"]
                    .to_string()
                    .trim()
                    .replace(",\"\"", "")
                    .split(",[")
                    .flat_map(str::parse::<String>)
                    .collect::<Vec<String>>();
            },
            Err(e) => println!("{}", e),
        }
        vec
    }
}
*/