/******************************* Functions for Threads **************************************/
pub mod thread_fn {
    use std::{thread, sync::{mpsc, Arc}, time::Duration};
    use crate::import::{user_import::quantity_card, self};
    use crate::types::{Card, CEerror, CEResult};

    use super::database;

    pub fn thread_card_make_api(decklist: &Arc<Vec<String>> , tx: &mpsc::Sender<(u8, Card)> , i: &usize) {
        let quantity_card = quantity_card(&decklist[*i]).expect("Incompatible decklist format");
        match import::scryfall::get(&quantity_card[1]) {
            Ok(t) => {
                match Card::make(&t) {
                    Ok(t) => {
                        println!("Fetched Card: {}", t.name);
                        tx.send((*i as u8,t)).unwrap();
                        thread::sleep(Duration::from_millis(10))
                    },
                    Err(e) => println!("Thread error detected: {}", e),
                }
            },
            Err(e) => println!("Thread error detected: {}", e),
        }
    }
    pub fn thread_card_make_database(decklist: &Arc<Vec<String>> , tx: &mpsc::Sender<(u8, Card)> , i: &usize, database: &Arc<serde_json::Value>){

        let quantity_card = quantity_card(&decklist[*i]).expect("Incompatible decklist format");
        match database::get(&quantity_card[1], &database) {
            Ok(t) => {
                println!("{}", &t);
                match Card::make(&t.to_string()) {
                    Ok(t) => {
                        println!("Database Card: {}", t.name);
                        tx.send((*i as u8,t)).unwrap();
                        thread::sleep(Duration::from_millis(10))
                    },
                    Err(e) => println!("Thread error detected: {}", e),
                } 
            },
            Err(e) => {
                println!("Failed to load from database, try scryfall request");
                match import::scryfall::get(&quantity_card[1]) {
                    Ok(t) => {
                        match Card::make(&t) {
                            Ok(t) => {
                                println!("Fetched Card: {}", t.name);
                                tx.send((*i as u8,t)).unwrap();
                                thread::sleep(Duration::from_millis(10))
                            },
                            Err(e) => println!("Thread error detected: {}", e),
                        }
                    },
                    Err(e) => panic!("Thread error detected: {}", e),
                } 
            },
        } 
    }
    }
/******************************* Logic to build a Card **************************************/
pub mod card_build {
    pub fn name(input: String) {}
    pub fn cmc(input: String) {}
    pub fn mana_cost(input: String) {}
    pub fn cardtype(input: String) {}
    pub fn legendary(input: String) {}
    pub fn stats(input: String) {}
    pub fn commander(input: String) {}
    pub fn backside(input: String) {}
    pub fn oracle_text(input: String) {}
    pub fn keys(input: String) {}
    pub fn zones(input: String) {}
}
/******************************** Database functions ****************************************/
// Rebuild for full json and managment with serde
pub mod database{
    use crate::types::{CEerror, CEResult};
    use std::{fs::File, io::{prelude::*, BufReader}};
    use serde_json::Value;

    pub fn save(input: String) -> CEResult<()> {
        let mut database = input
            .trim()
            .split("}},")
            .flat_map(str::parse::<String>)
            .collect::<Vec<String>>();
        let v: Value = serde_json::from_str(&input).expect("Can not create json");
        serde_json::to_writer(&File::create("database.txt").expect("Can not write database.txt"),&v).expect("Can not write database.txt"); 
       
        Ok(())
    }
    // There is a difference of around 900 cards between downloaded and loaded from database.txt
    pub fn load() -> CEResult<serde_json::Value> {
        let mut contents = String::new();


        match File::open("database.txt") {
            Ok(mut t) => {
                let mut buf_reader = BufReader::new(t);
                buf_reader.read_to_string(&mut contents).expect("Can not open database");

                let result: Value = serde_json::from_str(&contents).expect("Json not properly build"); 

                Ok(result) 
            },
            Err(_) => Err(CEerror::DatabaseError),
        }       
    }
    pub fn update() {}
    pub fn get<'a>(input: &String, database: &'a serde_json::Value) -> CEResult<&'a Value> {
        
        match database.get(input) {
            Some(t) => return Ok(t),
            None => return Err(CEerror::CardNotFound),
        }

    }
}
