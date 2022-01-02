/******************************* Functions for Threads **************************************/
pub mod thread_fn {
    use std::{thread, sync::{mpsc, Arc}, time::Duration};
    use crate::import::{user_import::quantity_card, self};
    use crate::types::Card;

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
    pub fn thread_card_make(decklist: &Arc<Vec<String>> , tx: &mpsc::Sender<(u8, Card)> , i: &usize, database: &Arc<serde_json::Value>){ 
        let quantity_card = quantity_card(&decklist[*i]).expect("Incompatible decklist format");
        match database::get(&quantity_card[1], &database) {
            Ok(t) => {
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
                
                thread_card_make_api(decklist, tx, i);
            },
        } 
    }
}
/******************************* Logic to build a Card **************************************/
pub mod card_build {
    use strum::IntoEnumIterator;

    pub fn name(input: String, dfc: bool, backside: bool) -> String { 
        if dfc {
            let split: Vec<String> = input.trim().split("\\").flat_map(str::parse::<String>).collect::<Vec<String>>();
            if backside {
                return split[1].replace("\"", "");
            } else {
                return split[0].replace("\"", "");
            }
        }
        input.replace("\"", "")
    }
    pub fn cmc(input: String) -> f32 {
        use crate::types::Colours;

        let mut i: f32 = 0.0;
        for colour in Colours::iter() {
            if input.contains(&colour.to_string()) {
                i += 1.0;
            }
        }

        // Prepare from {4}{R}{G} to 4 R G
        let mana = &input
        .replace("{", "")
        .replace("}", " ")
        .replace("\"", "")
        .trim()
        .split(" ")
        .flat_map(str::parse::<String>)
        .collect::<Vec<String>>();

        // Check if first char is number, if yes add colour
        match mana[0].parse::<f32>(){
            Ok(t) => {
                i += t; 
                return i;
            },
            Err(_) => return i,
        } 
    }
    pub fn mana_cost(input: String) -> String {
        input.replace("\"", "")
    }
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
pub mod database{
    use crate::types::{CEerror, CEResult};
    use std::{fs::{self, *}, io::{prelude::*, BufReader}, time::{SystemTime, Duration}, ops::Add};
    use serde_json::Value;
    use crate::import;
    
    pub fn save(input: String) -> CEResult<()> {
        
        let v: Value = serde_json::from_str(&input).expect("Can not create json");
        serde_json::to_writer(&File::create("database.txt").expect("Can not write database.txt"),&v).expect("Can not write database.txt"); 
       
        Ok(())
    }
    pub fn load() -> CEResult<serde_json::Value> {
        let mut contents = String::new();

        println!("Open database from system");

        match File::open("database.txt") {
            Ok(t) => {
                let mut buf_reader = BufReader::new(t);
                buf_reader.read_to_string(&mut contents).expect("Can not open database");

                let result: Value = serde_json::from_str(&contents).expect("Json not properly build"); 

                println!("Database successfully opened"); 

                Ok(result) 
            },
            Err(_) => Err(CEerror::DatabaseError),
        }       
    }
    pub fn update() {
        
        println!("Updating or creating local card library");

        let file = File::open("database.txt");
        
        match file {
            Ok(_) => {
                let metadata = fs::metadata("database.txt").expect("File found but can not open");
                let now = SystemTime::now();
                
                if let Ok(time) = metadata.modified() {
                    // Update every full day
                    if time.add(Duration::from_secs(86400)) <= now {
                        println!("File is older than a day: Update....");
                        match import::scryfall::get_bulk() {
                            Ok(_) => println!("Database successfully downloaded"),
                            Err(e) => println!("{}",e),
                        }
                    }   
                }                
            },
            Err(_) => {
                println!("Library was not found, download from scryfall");
                match import::scryfall::get_bulk() {
                    Ok(_) => println!("Database successfully downloaded"),
                    Err(e) => println!("{}",e),
                }
            },
        }


    }
    pub fn get<'a>(input: &String, database: &'a serde_json::Value) -> CEResult<&'a Value> {
        
        // Estimated length of library plus a few thousand. Faster than constructing length. 
        let data_len = 30000; 
        let art_card = format!("{} // {}", input, input);

        for i in 0..data_len {
            match database[i].get("name") {
                Some(t) => { 
                    if t.to_string().replace("\"", "").contains(input) 
                    && !t.to_string().contains(&art_card) {
                       return Ok(&database[i]); 
                    }
                },
                None => (),
            
            }
        }
        return Err(CEerror::CardNotFound);
    }
}