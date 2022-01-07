/******************************* Functions for Threads **************************************/
pub mod thread_fn {
    use std::{thread, sync::{mpsc, Arc}, time::Duration};
    use crate::{import::{user_import::{quantity_card, decklist}, self }, types::CEResult};
    use crate::types::{Card, Deck};
    use crate::logic::database;

    pub fn deck(input: String) -> CEResult<Deck> {
        use serde_json::Value;

        let mut deck = Deck::new(String::from(&input), Vec::<Card>::new(), Vec::<Card>::new(), );

        match decklist(input) {

            Ok(t) => {
                let (tx, rx) = mpsc::channel();
                let tx1 = tx.clone();
                let tx2 = tx.clone();
                let tx3 = tx.clone();

                let tasks = Arc::new(t);
                let tasks_arc_clone1 = Arc::clone(&tasks);
                let tasks_arc_clone2 = Arc::clone(&tasks);
                let tasks_arc_clone3 = Arc::clone(&tasks);

                let quater_one = tasks.len() / 4;
                let quater_two = tasks.len() / 2;
                let quater_three = 3 * tasks.len() / 4;          

                // Little hack to pass through a valid Value to get the API function when load failed
                let replace: Value = serde_json::from_str("{\"value\": \"Database not loaded\"}").expect("Fatal error: Can not build replacement json");

                let database_unwrap: Value = match database::load() {
                    Ok(t) => t,
                    Err(_) => {
                        println!("Can not open database, threads default to api request");
                        replace
                    },
                };

                let database = Arc::new(database_unwrap);
                let database_arc_clone1 = Arc::clone(&database);
                let database_arc_clone2 = Arc::clone(&database);
                let database_arc_clone3 = Arc::clone(&database);

                let handle1 = thread::spawn(move || {
                    for i in 0..quater_one{
                        thread_card_make(&tasks_arc_clone1, &tx, &i, &database_arc_clone1);
                    }        
                });
                let handle2 = thread::spawn(move || { 
                    for i in quater_one..quater_two {
                        thread_card_make(&tasks_arc_clone2, &tx1, &i, &database_arc_clone2); 
                    } 
                });
                let handle3 = thread::spawn(move || {
                    for i in quater_two..quater_three {
                        thread_card_make(&tasks_arc_clone3, &tx2, &i, &database_arc_clone3);
                    } 
                });
                
                for i in quater_three..tasks.len() {
                    thread_card_make(&tasks, &tx3, &i, &database);
                } 
                

                drop(tx3);
                
                for card in rx {
                    if !card.commander{
                        deck.library.push(card);
                    } else {
                        deck.commander.push(card);
                    } 
                }
                handle1.join().expect("Can not join thread");
                handle2.join().expect("Can not join thread");
                handle3.join().expect("Can not join thread");
                return Ok(deck);
            },
            Err(e) => return Err(e),
        }
    }
    fn thread_card_make_api(decklist: &Arc<Vec<String>> , tx: &mpsc::Sender<Card> , i: &usize) {
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
                        println!("Scryfall: {} {}",&quantity_card[0], t.name);
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
    fn thread_card_make(decklist: &Arc<Vec<String>> , tx: &mpsc::Sender<Card> , i: &usize, database: &Arc<serde_json::Value>){ 
        let mut commander: bool = false;
        let mut quantity_card = quantity_card(&decklist[*i]).expect("Incompatible decklist format");


        if quantity_card[1].contains("*CMDR*") {
            commander = true;
            quantity_card[1] = quantity_card[1].replace(" *CMDR*", "");
        }

        if quantity_card[1].contains("/") && !quantity_card[1].contains("//") {
            quantity_card[1] = quantity_card[1].replace("/", "//");
        }

 
        match database::get(&quantity_card[1], &database) {
            Ok(t) => {
                match Card::make(&t.to_string(), commander) {
                    Ok(t) => {
                        println!("Database: {} {}",&quantity_card[0], t.name);
                        for _j in 0..quantity_card[0].parse::<u8>().expect("List format error: No integer.") {   
                            tx.send(t.clone()).expect("Thread can not send");
                        }
                        thread::sleep(Duration::from_millis(10))
                    },
                    Err(e) => println!("Thread error detected: {}", e),
                } 
            },
            Err(_) => {
                
                thread_card_make_api(decklist, tx, i);
            },
        } 
    }
}
/******************************* Logic to build a Card **************************************/
pub mod card_build {
    use strum::IntoEnumIterator;
    use crate::types::*;


    pub fn build(v: &serde_json::Value, commander: bool, mdfc: Option<&serde_json::Value>) -> Card {
        
        Card{
            cmc: cmc(v["cmc"].to_string()),
            mana_cost: mana_cost(v["mana_cost"].to_string()),
            name: name(v["name"].to_string()),
            cardtype: cardtype(v["type_line"].to_string()),
            legendary: legendary(v["type_line"].to_string()),
            stats:stats(&v),
            commander: commander,
            backside: backside(mdfc),
            oracle_text: oracle_text(v["oracle_text"].to_string()),
            keys: keys(v["oracle_text"].to_string()),
            zones: zones(v["oracle_text"].to_string()),
            keywords: keywords(v["oracle_text"].to_string()),
            oracle_types: oracle_types(v["oracle_text"].to_string()),
            restrictions: restrictions(v["oracle_text"].to_string()),
        }
    }
    fn name(input: String) -> String {  
        input.replace("\"", "")
    }
    fn cmc(input: String) -> f32 {

        let mut i: f32 = 0.0;
        for colour in Colors::iter() {
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
            if input.contains(&types.to_string()) {
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
    fn backside(input: Option<&serde_json::Value>) -> Option<Box<Card>> {
        match input {
            Some(t) => {return Some(Box::new(build(t, false , None))); },
            None => None,
        }
    }
    fn oracle_text(input: String) -> String { // not neccessary, but maybe need to build something here
        input
    }
    fn keys(input: String) -> Option<Vec<Keys>> {
        let mut result: Vec<Keys> = Vec::new();

        for key in Keys::iter() {
            if input.to_lowercase().contains(&key.to_string().to_lowercase().replace("_", " ")) {
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
    fn keywords(input: String) -> Option<Vec<Keywords>> {
        let mut result: Vec<Keywords> = Vec::new();

        for keyword in Keywords::iter() {
            if input.to_lowercase().contains(&keyword.to_string().to_lowercase().replace("_", " ")) {
                result.push(keyword); 
            }
        }

        if result.len() != 0 {
            return Some(result);
        }else {
            return None;
        }
    }
    fn oracle_types(input: String) -> Option<Vec<CardType>> {
        let mut result: Vec<CardType> = Vec::new();
        let mut buffer: Vec<CardType> = Vec::new();

        for types in CardType::iter() {
            if input.to_string().to_lowercase().contains(&types.to_string().to_lowercase()) {
                buffer.push(types);
            }
        }

        for types in buffer {
            result.push(get_type(&input, types));
        }

        if result.len() != 0 {
            return Some(result);
        }else {
            return None;
        }
    }
    fn restrictions(input: String ) -> Option<Vec<Restrictions>> {
        let mut result: Vec<Restrictions> = Vec::new();

        for restriction in Restrictions::iter() {
            if input.to_lowercase().contains(&restriction.to_string().to_lowercase().replace("_", " ")) {
                result.push(restriction); 
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
                    if input.to_lowercase().contains(&item.to_string().to_lowercase().replace("_", " ")) { 
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
                    if input.to_lowercase().contains(&item.to_string().to_lowercase().replace("_", " ")) { 
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
                    if input.to_lowercase().contains(&item.to_string().to_lowercase().replace("_", " ")) { 
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
                    if input.to_lowercase().contains(&item.to_string().to_lowercase().replace("_", " ")) { 
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
                    if input.to_lowercase().contains(&item.to_string().to_lowercase().replace("_", " ")) { 
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
                    if input.to_lowercase().contains(&item.to_string().to_lowercase().replace("_", " ")) { 
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
            CardType::Planeswalker => { return CardType::Planeswalker },
            CardType::Token => { return CardType::Token },
            CardType::Basic => { return CardType::Basic },
            CardType::Card => { return CardType::Card },
            CardType::InvalidCardType => { return CardType::InvalidCardType }, 
        }
    }
}
/******************************** Database functions ****************************************/
pub mod database{
    use crate::types::{CEerror, CEResult};
    use std::{fs::{self, *}, io::{prelude::*, BufReader}, time::{SystemTime, Duration}, ops::Add};
    use serde_json::Value;
    use crate::import;
    
    pub fn save(input: &String) -> CEResult<()> {
        
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
                        match remove_file("database.txt"){
                            Ok(_) => println!("Expired database removed..."),
                            Err(_) => println!("Can not remove old database..."),
                        }
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
                    if t.to_string().replace("\"", "") == *input 
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