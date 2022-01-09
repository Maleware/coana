/***************************** Statistic and Evaluation ***********************************/


pub mod basic {
    use crate::types::*;
    use std::{collections::{BTreeMap, HashMap}};
    use crate::types::Colors;
    use strum::IntoEnumIterator;
    
    #[derive(Debug)]
    pub struct Basic<'deck> {
        pub cardtype: Cardtype<'deck>,
        pub mana_cost: BTreeMap<u8, Vec<&'deck Card>>,
        pub mana_dist: ManaDist<'deck>,
        pub effect: Effect<'deck>,
    }
    impl <'deck> Basic<'deck> {
        pub fn new(deck: &Deck) -> Basic {
            Basic {
                cardtype: cardtype(deck),
                mana_cost: mana_cost(deck),
                mana_dist: mana_distribution(deck),
                effect: effect(deck),
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
    #[derive(Debug)]
    pub struct Effect<'deck> {
        pub draw: Vec<&'deck Card>,
        pub bounce: Vec<&'deck Card>,
        pub removal: Vec<&'deck Card>,
        pub boardwipe: Vec<&'deck Card>,
        pub lord: Vec<&'deck Card>,
        pub counter: Vec<&'deck Card>,
        pub payoff: Vec<&'deck Card>,
        pub recursion: Vec<&'deck Card>,
        pub reanimation: Vec<&'deck Card>,
        pub stax: Vec<&'deck Card>,
        
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
                    // Pips per card, colored mana requirement
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
                                        // Mana productions to the basis of colors
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
                                                    if !card.contains(Keys::Sacrifice, CardFields::Keys){ 
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
            // Landramp including searching your library
            if (card.contains(Keys::Search, CardFields::Keys) 
            && card.contains(CardType::Land(None), CardFields::OracleType)
            && card.contains(Zones::Library, CardFields::Zones) && card.contains(Zones::Battlefield, CardFields::Zones)
            && !card.contains(CardType::Land(None), CardFields::CardType)
            && !card.contains(CardType::Instant(None), CardFields::CardType) )
            // Play additional Lands effects
            || (card.contains(Keys::Additional, CardFields::Keys,) 
                && card.contains(CardType::Land(None), CardFields::OracleType) 
                && card.contains(Keys::Turns, CardFields::Keys) ) {
                lands.push(card);
            }
            if card.contains(Keys::Untap, CardFields::Keys) 
            && card.contains(Restrictions::Target, CardFields::Restrictions)
            && card.contains(CardType::Land(None), CardFields::OracleType) {
                dorks.push(card);
            }
            // Needed to get the mana reduction artifacts and effects
            if card.contains(Keys::Cost, CardFields::Keys) 
            && card.contains(Restrictions::Less, CardFields::Restrictions) 
            && card.contains(Keys::Cast, CardFields::Keys) 
            && card.contains(Keys::Spell, CardFields::Keys)
            && !card.contains(Zones::Battlefield, CardFields::Zones)  
            {
                artifacts.push(card);
            }
        }
        return ManaDist{ manacost, manaprod, dorks, artifacts, enchantments, lands };        

    }
    pub fn effect(deck: &Deck) -> Effect {
        let mut draw: Vec<&Card> = Vec::new();
        let mut bounce: Vec<&Card> = Vec::new();
        let mut removal: Vec<&Card> = Vec::new();
        let mut boardwipe: Vec<&Card> = Vec::new();
        let mut lord: Vec<&Card> = Vec::new();
        let mut counter: Vec<&Card> = Vec::new();
        let mut payoff: Vec<&Card> = Vec::new();
        let mut recursion: Vec<&Card> = Vec::new();
        let mut reanimation: Vec<&Card> = Vec::new();
        let mut stax: Vec<&Card> = Vec::new();

        for card in &deck.library {
            match &card.backside {
                Some(backside) => {
                    if is_draw(&backside){
                        draw.push(&backside);
                    }
                    if is_removal(&backside){ 
                        removal.push(&backside);
                    }
                    if is_counter(&backside){
                        counter.push(&backside);
                    }
                    if  is_bounce(&backside){
                        bounce.push(&backside);
                    }
                    if is_recursion(&backside) {
                        recursion.push(&backside);
                    } 
                    if is_reanimation(&backside) {
                        reanimation.push(&backside); 
                    }
                    if is_boardwipe(&backside){
                        boardwipe.push(&backside);
                    }
                    if is_payoff(&backside) { 
                        payoff.push(&backside);
                    }
                    if is_lord(&backside) { 
                        lord.push(&backside);
                    }
                    if is_stax(&backside) {
                        stax.push(&backside);
                    } 
                },
                None => (),
            }
            if is_draw(card){
                draw.push(card);
            }
            if is_removal(card){ 
                removal.push(card);
            }
            if is_counter(card){
                counter.push(card);
            }
            if  is_bounce(card){
                bounce.push(card);
            }
            if is_recursion(card) {
                recursion.push(card);
            } 
            if is_reanimation(card) {
                reanimation.push(card); 
            }
            if is_boardwipe(card){
                boardwipe.push(card);
            }
            if is_payoff(card) { 
                payoff.push(card);
            }
            if is_lord(card) { 
                lord.push(card);
            }
            if is_stax(card) {
                stax.push(card);
            }
        }
        return Effect{draw, bounce, removal, boardwipe, lord, counter, payoff, recursion, reanimation, stax};
    }
    fn is_draw(card: &Card) -> bool {
        if (( card.contains(Keys::Draw, CardFields::Keys) 
        && (!card.contains(Restrictions::Drawstep, CardFields::Restrictions ) && !card.contains(Restrictions::After, CardFields::Restrictions) ) 
        && (!card.contains(Keys::Exile, CardFields::Keys) && !card.contains(Restrictions::Instead, CardFields::Restrictions))   )
        // Impulsive draw: Exile top card of your library 
        || ( card.contains(Keys::Exile, CardFields::Keys) 
            && card.contains(Keys::Top, CardFields::Keys) 
            && card.contains(CardType::Card, CardFields::OracleType) 
            && card.contains(Zones::Library, CardFields::Zones)
            && !card.contains(Restrictions::Reveal, CardFields::Restrictions)
            && !card.contains(Restrictions::Until, CardFields::Restrictions)
            && !card.contains(Restrictions::Instead, CardFields::Restrictions)) )
        && !card.contains(Keys::OneMana, CardFields::Keys)
        && !card.contains(Keys::Tapped, CardFields::Keys)
        && !card.contains(Keywords::Imprint, CardFields::Keywords) 
        && !card.contains(Keys::Search, CardFields::Keys){
            return true;
        } else {
            return false;
        }
    } 
    fn is_bounce(card: &Card) -> bool {
        if (card.contains(Keys::Return, CardFields::Keys) ||  card.contains(Keys::Put, CardFields::Keys) )
        && (card.contains(Keys::Owner, CardFields::Keys) && card.contains(Zones::Hand, CardFields::Zones) )
        && (!card.contains(Keys::Put, CardFields::Keys) && !card.contains(Keywords::Dash, CardFields::Keywords) && !card.contains(Zones::Graveyard, CardFields::Zones)){
            return true;
        } else {
            return false;
        }
    } 
    fn is_removal(card: &Card) -> bool {
        if( card.contains(Keys::Destroy, CardFields::Keys) || (card.contains(Keys::Exile, CardFields::Keys) && !card.contains(&*card.name, CardFields::OracleText) && !card.contains(Keys::Return, CardFields::Keys))) 
        && card.contains(Restrictions::Target, CardFields::Restrictions) 
        && ( !card.contains(Zones::Hand, CardFields::Zones) || card.contains(Keywords::Evoke, CardFields::Keywords) )
            // Overload boardwipes are removal too || Ugly hack to exlcude Sevinnes Reclamation
        &&!card.contains(Keywords::Flashback, CardFields::Keywords) && !card.contains(Restrictions::Own, CardFields::Restrictions) {
            return true; 
        } else {
            return false;
        } 
    } 
    fn is_boardwipe(card: &Card) -> bool {
        if( card.contains(Keys::Destroy, CardFields::Keys) || (card.contains(Keys::Exile, CardFields::Keys) && !card.contains(&*card.name, CardFields::OracleText) && !card.contains(Keys::Return, CardFields::Keys))) 
        && card.contains(Restrictions::Target, CardFields::Restrictions) 
        && ( !card.contains(Zones::Hand, CardFields::Zones) || card.contains(Keywords::Evoke, CardFields::Keywords) )
        && card.contains(Keywords::Overload, CardFields::Keywords) {
                return true;
        } else if ( card.contains(Restrictions::Each, CardFields::Restrictions) 
        || card.contains(Restrictions::All, CardFields::Restrictions) 
        || card.contains(Restrictions::Every, CardFields::Restrictions) ) 
        && (card.contains(Keys::Destroy, CardFields::Keys)
            || (card.contains(Keys::Exile, CardFields::Keys) && !card.contains(&*card.name, CardFields::OracleText ) && !card.contains(Keys::Return, CardFields::Keys) ) 
            || ( card.contains(Keys::Return, CardFields::Keys) && !card.contains(Keys::Exile, CardFields::Keys) )
            || card.contains(Restrictions::MinusXX, CardFields::Restrictions) ) 
        && !card.contains(Keywords::Overload, CardFields::Keywords) 
        && !card.contains(Keywords::Phasing, CardFields::Keywords)
        && !card.contains(Keywords::Flashback, CardFields::Keywords) 
        && !card.contains(CardType::Planeswalker, CardFields::CardType)
        && (!card.contains(Zones::Hand, CardFields::Zones) && !card.contains(Keys::Opponent, CardFields::Keys)){
            return true;
        } else if (card.contains(Keys::Return, CardFields::Keys) ||  card.contains(Keys::Put, CardFields::Keys))
        && (card.contains(Keys::Owner, CardFields::Keys) && card.contains(Zones::Hand, CardFields::Zones) )
        && card.contains(Keywords::Overload, CardFields::Keywords) && card.contains(Keys::Owner, CardFields::Keys){ 
            return true;
        } else {
            return false
        }
    } 
    fn is_lord(card: &Card) -> bool {
        if ( card.contains(Restrictions::Each, CardFields::Restrictions) 
                || card.contains(Restrictions::All, CardFields::Restrictions) 
                || card.contains(Restrictions::Every, CardFields::Restrictions)
                || (card.contains(CardType::Creature(None), CardFields::OracleType) 
                    && card.contains(Restrictions::You, CardFields::Restrictions) 
                    && card.contains(Restrictions::Control, CardFields::Restrictions) )) 
            && card.contains(Restrictions::Get, CardFields::Restrictions)
            && card.contains(Restrictions::PlusSymbol, CardFields::Restrictions) 
            && !card.contains(Keywords::Exalted, CardFields::Keywords)
            && !card.contains(Restrictions::MinusSymbol, CardFields::Restrictions){
                return true; 
        } else {
            return false;
        }
    } 
    fn is_counter(card: &Card) -> bool {
        if( (card.contains(Keys::Counter, CardFields::Keys) && !card.contains(Restrictions::CanT, CardFields::Restrictions))
        && card.contains(Restrictions::Target, CardFields::Restrictions)
        && card.contains(Keys::Spell, CardFields::Keys))
        || (card.contains(Keys::Redirect, CardFields::Keys) && !card.contains(CardType::Artifact(None), CardFields::CardType) && !card.contains(Keywords::Storm, CardFields::Keywords)){
            return true;
        } else {
            return false;
        }
 
    } 
    fn is_payoff(card: &Card) -> bool {
        if ( card.contains(Restrictions::Whenever, CardFields::Restrictions) && !card.contains(Keys::Tapped, CardFields::Keys) )
        && (card.contains(Keys::ETB, CardFields::Keys) 
            || (card.contains(Keys::Cast, CardFields::Keys) && card.contains(Restrictions::You, CardFields::Restrictions))
            || (card.contains(Keys::Copy, CardFields::Keys) && card.contains(Restrictions::You, CardFields::Restrictions))
            || (card.contains(Keys::Play, CardFields::Keys) && card.contains(Restrictions::You, CardFields::Restrictions))
            || (card.contains(Keys::Damage, CardFields::Keys) && !card.contains(Restrictions::You, CardFields::Restrictions)) 
            || card.contains(Restrictions::Die, CardFields::Restrictions)
            || (card.contains(Keys::Discard, CardFields::Keys) && !card.contains(Restrictions::Drawstep, CardFields::Restrictions) )
            || (card.contains(Restrictions::GainLife, CardFields::Restrictions) && card.contains(Restrictions::You, CardFields::Restrictions))
            || ( (card.contains(Keys::Draw, CardFields::Keys) &&!card.contains(Restrictions::Drawstep, CardFields::Restrictions)) && card.contains(Restrictions::You, CardFields::Restrictions) ))
        || (card.contains(Keys::Sacrifice, CardFields::Keys)   
            && card.contains(CardType::Creature(None), CardFields::OracleType) 
            && !card.contains(Keys::Search, CardFields::Keys)
            && !card.contains(&card.name.to_string(), CardFields::OracleText))
        || (card.contains(Keys::Sacrifice, CardFields::Keys) 
            && card.contains(CardType::Artifact(None), CardFields::OracleType)
            && !card.contains(Keys::Search, CardFields::Keys)
            && !card.contains(&card.name.to_string(), CardFields::OracleText )){ 
                return true;
        } else {
            return false;
        }        
    } 
    fn is_recursion(card: &Card) -> bool {
        if (card.contains(Keys::Return, CardFields::Keys) ||  card.contains(Keys::Put, CardFields::Keys) )
        && ( card.contains(Zones::Graveyard, CardFields::Zones) && !card.contains(Restrictions::Drawstep, CardFields::Restrictions)) 
        && ( ( card.contains(Zones::Hand, CardFields::Zones) && !card.contains(Restrictions::Reveal, CardFields::Restrictions)) 
            || (card.contains(Zones::Library, CardFields::Zones) && card.contains(Keys::Top, CardFields::Keys)))
        && !card.contains(Keys::Counter, CardFields::Keys){
            return true;
        } else {
            return false;
        }
    } 
    fn is_reanimation(card: &Card) -> bool {
        if( (card.contains(Keys::Return, CardFields::Keys) 
            && !(card.contains(card.name.to_string(), CardFields::OracleText) && !card.contains(Keys::Tapped, CardFields::Keys) )) 
            ||  card.contains(Keys::Put, CardFields::Keys) )
        && card.contains(Zones::Graveyard, CardFields::Zones) 
        && (card.contains(Zones::Battlefield, CardFields::Zones) && !card.contains(Zones::Hand, CardFields::Zones) )
        && !card.contains(Keys::OneMana, CardFields::Keys)
        && !card.contains(Restrictions::Instead, CardFields::Restrictions)
        { 
            return true;
        } else {
            return false;
        }
    } 
    fn is_stax(card: &Card) -> bool {
        if (card.contains(Restrictions::CanT, CardFields::Restrictions) 
            || (card.contains(Keys::Player, CardFields::Restrictions)&& card.contains(Restrictions::Each, CardFields::Restrictions))
            || (card.contains(Keys::Opponent, CardFields::Keys) && card.contains(Restrictions::Each, CardFields::Restrictions))
            || (card.contains(Keys::Cost, CardFields::Keys) 
                && card.contains(Restrictions::More, CardFields::Restrictions) ) 
            || (card.contains(Restrictions::CanT, CardFields::Restrictions) && card.contains(Keys::Activate, CardFields::Keys) ) 
            || (card.contains(Restrictions::Non, CardFields::Restrictions) && card.contains(CardType::Basic, CardFields::OracleType) )
            || (card.contains(CardType::Land(None), CardFields::OracleType) && card.contains(Restrictions::Dont, CardFields::Restrictions) && card.contains(Restrictions::Untap, CardFields::Restrictions)))
        && (card.contains(CardType::Creature(None), CardFields::CardType)
            || card.contains(CardType::Artifact(None), CardFields::CardType) 
            || card.contains(CardType::Enchantment(None), CardFields::CardType)
            || card.contains(CardType::Planeswalker, CardFields::CardType))
        && !card.contains(Keys::Add, CardFields::Keys)
        && !card.contains(Keys::ETB, CardFields::Keys){
                return true;
            }
            return false;
    }
}

pub mod r#abstract {}

pub mod tutor {}
pub mod powerlevel {}