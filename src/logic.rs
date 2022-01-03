/******************************* Functions for Threads **************************************/
pub mod thread_fn {
    use std::{thread, sync::{mpsc, Arc}, time::Duration};
    use crate::import::{user_import::quantity_card, self};
    use crate::types::Card;

    use super::database;

    pub fn thread_card_make_api(decklist: &Arc<Vec<String>> , tx: &mpsc::Sender<Card> , i: &usize) {
        let mut commander: bool = false;
        let mut quantity_card = quantity_card(&decklist[*i]).expect("Incompatible decklist format");

        if quantity_card[1].contains("*CMDR*") {
            commander = true;
            quantity_card[1] = quantity_card[1].replace(" *CMDR*", "");
        }

        match import::scryfall::get(&quantity_card[1]) {
            Ok(t) => {
                match Card::make(&t, commander) {
                    Ok(t) => {
                        println!("Fetched Card:{} {}",&quantity_card[0], t.name);
                        for _j in 0..quantity_card[0].parse::<u8>().expect("List format error: No integer.") {   
                            tx.send(t.clone()).expect("Thread can not send.");
                        }    
                        thread::sleep(Duration::from_millis(10))
                    },
                    Err(e) => println!("Thread error detected: {}", e),
                }
            },
            Err(e) => println!("Thread error detected: {}", e),
        }
    }
    pub fn thread_card_make(decklist: &Arc<Vec<String>> , tx: &mpsc::Sender<Card> , i: &usize, database: &Arc<serde_json::Value>){ 
        let mut commander: bool = false;
        let mut quantity_card = quantity_card(&decklist[*i]).expect("Incompatible decklist format");

        if quantity_card[1].contains("*CMDR*") {
            commander = true;
            quantity_card[1] = quantity_card[1].replace(" *CMDR*", "");
        }

        match database::get(&quantity_card[1], &database) {
            Ok(t) => {
                match Card::make(&t.to_string(), commander) {
                    Ok(t) => {
                        println!("Database Card:{} {}",&quantity_card[0], t.name);
                        for _j in 0..quantity_card[0].parse::<u8>().expect("List format error: No integer.") {   
                            tx.send((t.clone())).expect("Thread can not send");
                        }
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
    use crate::types::*;


    pub fn build(v: serde_json::Value, commander: bool) -> Card {
         Card{
            cmc: cmc(v["cmc"].to_string()),
            mana_cost: mana_cost(v["mana_cost"].to_string()),
            name: name(v["name"].to_string()),
            cardtype: cardtype(v["type_line"].to_string()),
            legendary: legendary(v["type_line"].to_string()),
            stats:stats(&v),
            commander: commander,
            backside: Box::new(None),
            oracle_text: oracle_text(v["oracle_text"].to_string()),
            keys: keys(v["oracle_text"].to_string()),
            zones: zones(v["oracle_text"].to_string()),
         }
    }
    fn name(input: String) -> String {  
        input.replace("\"", "")
    }
    fn cmc(input: String) -> f32 {
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
    fn mana_cost(input: String) -> String {
        input.replace("\"", "")
    }
    fn cardtype(input: String) -> Vec<CardType> {
        let mut cardtype: Vec<CardType> = Vec::new();
        let mut result: Vec<CardType> = Vec::new();

        for types in CardType::iter() { 
            if input.contains(&types.to_string().replace("([])", "")) {
                cardtype.push(types);
            }
        }

        for item in cardtype { 
            result.push(get_type(&input, item));
        }

        return result;

    }
    fn legendary(input: String) -> bool {
        if input.contains("Legendary") {
            return true;
        } else {
            return false;
        }
    }
    fn stats(input: &serde_json::Value) -> Option<Vec<Stats>> {
        let mut result: Vec<Stats> = Vec::new();

        match input["power"].to_string().replace("\"", "").replace("*", "0").parse::<u8>() {
            Ok(t) => result.push(Stats::Power(t)),
            Err(_) => (),
        }
        match input["toughness"].to_string().replace("\"", "").replace("*", "0").parse::<u8>() {
            Ok(t) => result.push(Stats::Toughness(t)),
            Err(_) => (),
        }
        match input["loyality"].to_string().replace("\"", "").replace("*", "0").parse::<u8>() {
            Ok(t) => result.push(Stats::Loyality(t)),
            Err(_) => (),
        }
        if result.len() != 0 {
            return Some(result);
        } else {
            return None;
        }

    }
    fn backside(input: String) {}
    fn oracle_text(input: String) -> String { // not neccessary, but maybe need to build something here
        input
    }
    fn keys(input: String) -> Option<Vec<Keys>> {
        let mut result: Vec<Keys> = Vec::new();

        for key in Keys::iter() {
            if input.to_lowercase().contains(&key.to_string().to_lowercase()) {
                result.push(key);
            }
        }

        if result.len() != 0 {
            return Some(result);
        }else {
            return None;
        }
    }
    fn zones(input: String) -> Option<Vec<Zones>> {
        let mut result: Vec<Zones> = Vec::new();
        
        for zone in Zones::iter() {
            if input.to_lowercase().contains(&zone.to_string().to_lowercase()) {
                result.push(zone); 
            }
        }
        
        if result.len() != 0 {
            return Some(result);
        }else {
            return None;
        }
    }
    fn get_type(input: &String, cardtype: CardType) -> CardType {
        match cardtype {
            CardType::Creature(_) => {
                let mut buffer: Vec<Option<CreatureSubtype>> = Vec::new();
                for item in CreatureSubtype::iter() {
                    if input.to_lowercase().contains(&item.to_string().to_lowercase()) { 
                        buffer.push(Some(item));
                    }
                }
                if buffer.len() != 0 {
                    return CardType::Creature(buffer);
                } else {
                    buffer.push(None);
                    return CardType::Creature(buffer);
                }                
            },
            CardType::Artifact(_) => {
                let mut buffer: Vec<Option<ArtifactSubtype>> = Vec::new();
                for item in ArtifactSubtype::iter() {
                    if input.to_lowercase().contains(&item.to_string().to_lowercase()) { 
                        buffer.push(Some(item));
                    }
                }
                if buffer.len() != 0 {
                    return CardType::Artifact(buffer);
                } else {
                    buffer.push(None);
                    return CardType::Artifact(buffer);
                } 
            },
            CardType::Enchantment(_) => {
                let mut buffer: Vec<Option<EnchantmentSubtype>> = Vec::new();
                for item in EnchantmentSubtype::iter() {
                    if input.to_lowercase().contains(&item.to_string().to_lowercase()) { 
                        buffer.push(Some(item));
                    }
                }
                if buffer.len() != 0 {
                    return CardType::Enchantment(buffer);
                } else {
                    buffer.push(None);
                    return CardType::Enchantment(buffer);
                } 
            },
            CardType::Instant(_) => {
                let mut buffer: Vec<Option<SpellSubtype>> = Vec::new();
                for item in SpellSubtype::iter() {
                    if input.to_lowercase().contains(&item.to_string().to_lowercase()) { 
                        buffer.push(Some(item));
                    }
                }
                if buffer.len() != 0 {
                    return CardType::Instant(buffer);
                } else {
                    buffer.push(None);
                    return CardType::Instant(buffer);
                } 
            },
            CardType::Land(_) => {
                let mut buffer: Vec<Option<LandSubtype>> = Vec::new();
                for item in LandSubtype::iter() {
                    if input.to_lowercase().contains(&item.to_string().to_lowercase()) { 
                        buffer.push(Some(item));
                    }
                }
                if buffer.len() != 0 {
                    return CardType::Land(buffer);
                } else {
                    buffer.push(None);
                    return CardType::Land(buffer);
                } 
            },
            CardType::Sorcery(_) => {
                let mut buffer: Vec<Option<SpellSubtype>> = Vec::new();
                for item in SpellSubtype::iter() {
                    if input.to_lowercase().contains(&item.to_string().to_lowercase()) { 
                        buffer.push(Some(item));
                    }
                }
                if buffer.len() != 0 {
                    return CardType::Sorcery(buffer);
                } else {
                    buffer.push(None);
                    return CardType::Sorcery(buffer);
                } 
            
            },
            CardType::Planeswalker => { return CardType::Planeswalker; },
            _ => { return CardType::InvalidCardType; }, 
        }
    }
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