pub mod thread_fn {
    use std::{thread, sync::{mpsc, Arc}, time::Duration};
    use crate::import::{user_import::quantity_card, self};
    use crate::types::Card;
    
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
                        Err(e) => (),
                    }
                },
                Err(e) => (),
            }
        }
        pub fn thread_card_make_database(){}
    }