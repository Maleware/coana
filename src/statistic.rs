/***************************** Statistic and Evaluation ***********************************/


pub mod basic {
    use crate::types::*;
    use std::collections::{BTreeMap, HashMap};
    use crate::types::Colors;
    use strum::IntoEnumIterator;
    pub struct Basic<'deck> {
        pub cardtype: Cardtype<'deck>,
        pub mana_cost: BTreeMap<u8, Vec<&'deck Card>>,
        pub mana_dist: Mana_dist,
    }
    impl <'deck> Basic<'deck> {
        pub fn new(deck: &Deck) -> Basic {
            Basic {
                cardtype: cardtype(deck),
                mana_cost: mana_cost(deck),
                mana_dist: mana_distribution(deck),
            }
        }
    }
    #[derive(Debug)]
    pub struct Cardtype<'deck >{
        pub creatures: Vec<&'deck Card>,
        pub enchantments: Vec<&'deck Card>,
        pub artifacts: Vec<&'deck Card>,
        pub lands: Vec<&'deck Card>,
        pub planeswalkers: Vec<&'deck Card>,
        pub instants: Vec<&'deck Card>,
        pub sorcerys: Vec<&'deck Card>,
    }

    pub struct Mana_dist {
        pub manacost: HashMap<Colors, u8>,
        pub manaprod: HashMap<Colors, u8>,
    }
    pub fn cardtype<'deck> (deck: &'deck Deck) -> Cardtype<'deck> {
        let mut creatures = Vec::new();
        let mut enchantments = Vec::new();
        let mut artifacts = Vec::new();
        let mut lands =Vec::new();
        let mut planeswalkers = Vec::new();
        let mut instants = Vec::new();
        let mut sorcerys = Vec::new();

        for card in &deck.library {
            for cardtype in &card.cardtype { 
                match cardtype {
                    CardType::Creature(_) => ( creatures.push(card) ),
                    CardType::Enchantment(_) => ( enchantments.push(card)),
                    CardType::Artifact(_) => ( artifacts.push(card)),
                    CardType::Land(_) => ( lands.push(card) ),
                    CardType::Planeswalker => ( planeswalkers.push(card) ),
                    CardType::Instant(_) => ( instants.push(card) ),
                    CardType::Sorcery(_) => ( sorcerys.push(card) ),
                    _ => (),
                }
            }
        }
        Cardtype{
            creatures,
            enchantments,
            artifacts,
            lands, 
            planeswalkers,
            instants,
            sorcerys,
        }
    }
    pub fn mana_cost(deck: &Deck) -> BTreeMap<u8, Vec<&Card>> {
        let mut mana_cost = BTreeMap::new();

        for card in &deck.library {
            for types in &card.cardtype{
                match types {
                    CardType::Land(_) => (),
                    _ => mana_cost.entry(card.cmc as u8).or_insert(vec![card]).push(card),
                }
            }
        }

        mana_cost 
    }
    pub fn mana_distribution(deck: &Deck) -> Mana_dist{
        
        let mut manacost: HashMap<Colors, u8> = HashMap::new();
        let mut manaprod: HashMap<Colors, u8> = HashMap::new();
        let mut dorks: Vec<&Card> = Vec::new();
        let mut artifacts: Vec<&Card> = Vec::new();
        let mut enchantments: Vec<&Card> = Vec::new();

        for card in &deck.library {
            for color in Colors::iter() {
                if card.mana_cost.contains(&color.to_string()) {
                    *manacost.entry(color).or_insert(0) += card.mana_cost.matches(&color.to_string()).count() as u8;   
                }
            }
            for color in Colors::iter() {
                match &card.keys {
                    Some(keys) => {
                        for key in keys {
                            if *key == color.to_key() {
                                for key in keys {
                                    if key == &Keys::Tap || card.find(CardType::Enchantment(Vec::new()), CardFields::CardType){ // Creatures and Artifacts need to be tapped to add mana. Enchantments not
                                        for key in keys {
                                            if key == &Keys::Add {
                                                *manaprod.entry(color.clone()).or_insert(0) += card.oracle_text.matches(&color.to_string()).count() as u8;
                                                for cardtype in &card.cardtype {
                                                    match cardtype{ 
                                                        &CardType::Creature(_)=> {
                                                            let mut hit = false;
                                                            for dork in &dorks{
                                                            if card.name == *dork.name {
                                                                    hit = true;
                                                            }
                                                            }
                                                            if !hit {
                                                                dorks.push(&card)
                                                            }
                                                        },
                                                        &CardType::Artifact(_) => {
                                                            for key in keys {
                                                                match key {
                                                                    Keys::Sacrifice =>(),
                                                                    _ => {
                                                                        let mut hit = false;
                                                                        for ramp in &artifacts{
                                                                            if card.name == ramp.name {
                                                                                hit = true;
                                                                            }
                                                                        }
                                                                        if !hit {
                                                                            artifacts.push(card);
                                                                        }
                                                                    },
                                                                }
                                                            }
                                                        },
                                                        &CardType::Enchantment(_) => {
                                                            let mut hit = false;
                                                            for enchantment in &enchantments {
                                                                if card.name == enchantment.name {
                                                                    hit = true;
                                                                }
                                                            }
                                                            if !hit {
                                                                enchantments.push(card);
                                                            }
                                                        }
                                                        _ => (),
                                                    }
                                                } 
                                            }
                                        }
                                    } 
                                } 
                            }
                        }
                    },
                    None => (),
                }
            }
        }
        println!("Dorks: {:?}, Artifacts: {:?}, Enchantments: {:?}", dorks.len(), artifacts.len(), enchantments.len());
        return Mana_dist{ manacost, manaprod };        

    }
    pub fn effect(deck: &Deck) {} 
}

pub mod r#abstract {}

pub mod tutor {}
pub mod powerlevel {}