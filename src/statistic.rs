/***************************** Statistic and Evaluation ***********************************/


pub mod basic {
    use crate::types::*;
    use std::collections::BTreeMap;
    pub struct Basic<'deck> {
        pub cardtype: Cardtype<'deck>,
        pub mana_cost: BTreeMap<u8, Vec<&'deck Card>> 
    }
    impl <'deck> Basic<'deck> {
        pub fn new(deck: &Deck) -> Basic {
            Basic {
                cardtype: cardtype(deck),
                mana_cost: mana_cost(deck),
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
    pub fn mana_distribution(deck: &Deck) {}
    pub fn effect(deck: &Deck) {} 
}

pub mod r#abstract {}

pub mod tutor {}
pub mod powerlevel {}