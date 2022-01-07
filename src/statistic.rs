/***************************** Statistic and Evaluation ***********************************/


pub mod basic {
    use crate::types::*;
    use std::{collections::{BTreeMap, HashMap}, ops::RangeBounds};
    use crate::types::Colors;
    use strum::IntoEnumIterator;
    pub struct Basic<'deck> {
        pub cardtype: Cardtype<'deck>,
        pub mana_cost: BTreeMap<u8, Vec<&'deck Card>>,
        pub mana_dist: ManaDist<'deck>,
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
    #[derive(Debug)]
    pub struct ManaDist<'deck> {
        pub manacost: HashMap<Colors, u8>,
        pub manaprod: HashMap<Colors, u8>,
        pub dorks: Vec<&'deck Card>,
        pub artifacts: Vec<&'deck Card>,
        pub enchantments: Vec<&'deck Card>,
        pub lands: Vec<&'deck Card>,
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
    pub fn mana_distribution(deck: &Deck) -> ManaDist{
        
        let mut manacost: HashMap<Colors, u8> = HashMap::new();
        let mut manaprod: HashMap<Colors, u8> = HashMap::new();
        let mut dorks: Vec<&Card> = Vec::new();
        let mut artifacts: Vec<&Card> = Vec::new();
        let mut enchantments: Vec<&Card> = Vec::new();
        let mut lands: Vec<&Card> = Vec::new();

        for card in &deck.library {
            for color in Colors::iter() {
                if card.contains(&color, CardFields::ManaCost){
                    *manacost.entry(color).or_insert(0) += card.mana_cost.matches(&color.to_string()).count() as u8;   
                }
            }
            for color in Colors::iter() {
                match &card.keys {
                    Some(keys) => {
                        if card.contains(&color, CardFields::Keys) { 
                            if card.contains(Keys::Tap, CardFields::Keys)
                            || card.contains(CardType::Enchantment(None), CardFields::CardType){ 
                                for key in keys {
                                    if key == &Keys::Add {
                                        // if !card.find(Colors::OneMana, CardFields::Keys) && !card.find(Colors::AnyColor, CardFields::Keys) {
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
                                                    if !hit && !card.contains(Zones::Graveyard, CardFields::Zones){
                                                        dorks.push(&card)
                                                    }
                                                },
                                                &CardType::Artifact(_) => {
                                                    if !card.contains(Keys::Sacrifice, CardFields::Keys) { 
                                                        let mut hit = false;
                                                        for ramp in &artifacts{
                                                            if card.name == ramp.name {
                                                                hit = true;
                                                            }
                                                        }
                                                        if !hit && !card.contains(CardType::Land(None), CardFields::CardType) {
                                                            artifacts.push(card);
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
                    },
                    None => (),
                }
            }
            if card.contains(Keys::Search, CardFields::Keys) 
            && card.contains(CardType::Land(None), CardFields::OracleType)
            && card.contains(Zones::Library, CardFields::Zones) && card.contains(Zones::Battlefield, CardFields::Zones)
            && !card.contains(CardType::Land(None), CardFields::CardType)
            && !card.contains(CardType::Instant(None), CardFields::CardType) {
                lands.push(card);
            }
        }
        return ManaDist{ manacost, manaprod, dorks, artifacts, enchantments, lands };        

    }
    pub fn effect(deck: &Deck) {} 
}

pub mod r#abstract {}

pub mod tutor {}
pub mod powerlevel {}