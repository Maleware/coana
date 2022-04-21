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
        pub fastmana: Vec<&'deck Card>,
        
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
    // TODO: refracture 
    pub fn mana_distribution(deck: &Deck) -> ManaDist{
        
        let mut manacost: HashMap<Colors, u8> = HashMap::new();
        let mut manaprod: HashMap<Colors, u8> = HashMap::new();
        let mut dorks: Vec<&Card> = Vec::new();
        let mut artifacts: Vec<&Card> = Vec::new();
        let mut enchantments: Vec<&Card> = Vec::new();
        let mut lands: Vec<&Card> = Vec::new();

        let mut libcom: Vec<&Card> = Vec::new();
        
        for card in &deck.library {
            libcom.push(card)
        }
        for card in &deck.commander {
            libcom.push(card)
        }

        for card in libcom {
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
                                                *manaprod.entry(color).or_insert(0) += card.oracle_text.matches(&color.to_string()).count() as u8;
                                            
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
            && card.contains(CardType::Land(None), CardFields::OracleType) 
            && !card.contains(CardType::Planeswalker, CardFields::CardType)
            && !card.contains(CardType::Instant(None), CardFields::CardType){
                dorks.push(card);
            }
            // Needed to get the mana reduction artifacts and effects
            if card.contains(Keys::Cost, CardFields::Keys) 
            && card.contains(Restrictions::Less, CardFields::Restrictions) 
            && card.contains(Keys::Cast, CardFields::Keys) 
            && card.contains(Keys::Spell, CardFields::Keys)
            && !(card.contains(Zones::Battlefield, CardFields::Zones)  && !card.contains(Keys::ETB, CardFields::Keys) ) 
            && card.name != String::from("The Great Henge")
            {
                if card.contains(CardType::Artifact(None), CardFields::CardType){
                    artifacts.push(card);
                } else {
                    dorks.push(card);
                }
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
        let mut fastmana: Vec<&Card> = Vec::new();
      
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
                    if is_fastmana(&backside) {
                        fastmana.push(&backside)
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
            if is_fastmana(card) {
                fastmana.push(card);
            }
        }

        for card in &deck.commander {
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
                    if is_fastmana(&backside) {
                        fastmana.push(&backside)
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
            if is_fastmana(card) {
                fastmana.push(card);
            } 
        }

        return Effect{draw, bounce, removal, boardwipe, lord, counter, payoff, recursion, reanimation, stax, fastmana};
    }
    fn is_draw(card: &Card) -> bool {
        if (( (card.contains(Keys::Draw, CardFields::Keys) && !card.contains(Restrictions::CanT, CardFields::Restrictions) )
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
            || (card.contains(Keys::Exile, CardFields::Keys) && !card.contains(&*card.name, CardFields::OracleText ) && !card.contains(Keys::Return, CardFields::Keys) && !card.contains(Keys::Cast, CardFields::Keys)) 
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
            && (card.contains(Restrictions::PlusSymbol, CardFields::Restrictions) && !card.contains(CardType::Planeswalker, CardFields::CardType))
            && !card.contains(Keywords::Exalted, CardFields::Keywords)
            && !card.contains(Restrictions::MinusSymbol, CardFields::Restrictions)
            && !card.contains(Keys::Remove, CardFields::Keys){
                return true; 
        } else {
            return false;
        }
    } 
    fn is_counter(card: &Card) -> bool {
        if( (card.contains(Keys::Counter, CardFields::Keys) && !card.contains(Restrictions::CanT, CardFields::Restrictions) && !card.contains(Keys::Put, CardFields::Keys))
        && card.contains(Restrictions::Target, CardFields::Restrictions)
        && card.contains(Keys::Spell, CardFields::Keys) )
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
        && !card.contains(Keys::Counter, CardFields::Keys)
        && !card.contains(Keys::Surveil, CardFields::Keys){
            return true;
        } else {
            return false;
        }
    } 
    fn is_reanimation(card: &Card) -> bool {
        if( (card.contains(Keys::Return, CardFields::Keys) 
            && !(card.contains(card.name.to_string(), CardFields::OracleText) && !card.contains(Keys::Tapped, CardFields::Keys) )) 
            ||  (card.contains(Keys::Put, CardFields::Keys) ))
        && card.contains(Zones::Graveyard, CardFields::Zones) 
        && (card.contains(Zones::Battlefield, CardFields::Zones) && !card.contains(Zones::Hand, CardFields::Zones) )
        && !card.contains(Keys::OneMana, CardFields::Keys)
        && !card.contains(Restrictions::Instead, CardFields::Restrictions)
        && !card.contains(Keys::Search, CardFields::Keys)
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
        && !card.contains(Keys::ETB, CardFields::Keys)
        && !card.contains(&card.name, CardFields::OracleText){
                return true;
            }
            return false;
    }
    fn is_fastmana(card: &Card) -> bool {
        if (( card.contains(Keys::Sacrifice, CardFields::Keys) 
                && card.contains(&card.name, CardFields::OracleText) 
                && !card.contains(Keys::Search, CardFields::Keys)
                && !card.contains(CardType::Land(None), CardFields::CardType) )
            || ( (card.contains(CardType::Instant(None), CardFields::CardType) 
                    && !card.contains(Keys::Return, CardFields::Keys))
                || card.contains(CardType::Sorcery(None), CardFields::CardType) 
                || (card.contains(CardType::Creature(None), CardFields::CardType) 
                && !card.contains(Keys::Tap, CardFields::Keys) )))
        && (card.contains(Keys::Add, CardFields::Keys) || card.contains(ArtifactSubtype::Treasure, CardFields::OracleText) )
        && !card.contains(Keys::Additional, CardFields::Keys)
        && !card.contains(Keywords::Retrace, CardFields::Keywords) 
        && !card.contains(Keys::Draw, CardFields::Keys){  
            return true;
        } else {
            return false;
        }

    }
}
 
pub mod archetype {
    #[derive(Debug)]
    pub enum Archetype {
        Flicker,
        Storm,
        Enchantments,
        Pod, //Birthing Pod evolution decks
        LandsMatter,
        Wheel,
        Lifegain,
        Mill,
        Tokens,
        Voltron,
        Counters, //+1/+1 counter or any other sort of putting counter on stuff
        SuperFriends,
        Aristocrats,
        Artifacts,
        Tribal, // Human, Treefolk etc.
    }

    
}

pub mod tutor {
    use std::{collections::HashMap};
    use strum::IntoEnumIterator;

    use crate::types::{Card, Deck, *};
    use crate::statistic::basic;

    pub fn tutor<'deck>(deck: &'deck Deck) -> HashMap<&'deck String, Vec<&'deck Card>> {
        let mut tutor: HashMap<&'deck String, Vec<&'deck Card>> = HashMap::new();

        let mut sdeck = basic::Basic::new(&deck).cardtype;

        for card in &deck.library {           
            if card.contains(Keys::Search, CardFields::Keys) 
            && !card.contains(Keys::Opponent, CardFields::Keys)
            && !card.contains(Restrictions::CanT, CardFields::Restrictions)
            && !card.contains(String::from("Research"), CardFields::OracleText)
            // Tutor can force you to sacrifce a permanent of the chosen type. Diabolic intent and Arcum Dagson force you to sacrice a creature to find a another type 
            // && card.name != String::from("Diabolic Intent") 
            && card.name != String::from("Search for Azcanta"){
                let mut buffer: Vec<&Card> = Vec::new();
       
                match &card.oracle_types {
                    Some(types) => {
                        for typ in types {
                           buffer.append(&mut link_target(&card, &deck, &mut sdeck, typ)); 
                        }
                    },
                    None => (),
                }
                tutor.insert(&card.name, buffer);
            }
        }
        
        for card in &deck.commander {
            if card.contains(Keys::Search, CardFields::Keys) 
            && !card.contains(Keys::Opponent, CardFields::Keys)
            && !card.contains(Restrictions::CanT, CardFields::Restrictions)
            && !card.contains(String::from("Research"), CardFields::OracleText) {
                let mut buffer: Vec<&Card> = Vec::new();
       
                match &card.oracle_types {
                    Some(types) => {
                        for typ in types {
                           buffer.append(&mut link_target(&card, &deck, &mut sdeck, typ)); 
                        }
                    },
                    None => (),
                }
                tutor.insert(&card.name, buffer);
            } 
        }
        
        tutor
    }
    fn link_target<'deck>(tutor: &Card, deck: &'deck Deck, sdeck: &mut basic::Cardtype<'deck>, typ: &CardType) -> Vec<&'deck Card> {
        let mut targets: Vec<&'deck Card> = Vec::new();

        match typ {
            CardType::Artifact(_) => {
                if tutor.contains(Keys::With, CardFields::Keys) && !tutor.contains(Keys::Investigate, CardFields::Keys){
                    targets.append(&mut restrictions(deck, tutor, sdeck, CardType::Artifact(None)));
                } else {
                    match color_restrictions(sdeck, tutor, typ) {
                        Some(mut result) => {targets.append(&mut result)},
                        None => {
                            for card in &sdeck.artifacts {
                                if card.name != tutor.name && !tutor.contains(Keys::Investigate, CardFields::Keys){
                                    targets.push(*card)
                                }
                            }
                        },
                    }   
                }
            },
            CardType::Creature(_) => {
                if tutor.contains(Keys::With, CardFields::Keys)
                && !((tutor.contains(Keys::Exile, CardFields::Keys) && !tutor.contains(&tutor.name, CardFields::OracleText))
                    || tutor.contains(Keys::Token, CardFields::Keys)
                    || tutor.contains(Keys::Counter, CardFields::Keys)) { 
                    targets.append(&mut restrictions(deck, tutor, sdeck, CardType::Creature(None)));
                } else {
                    if !(tutor.contains(Keys::Exile, CardFields::Keys) 
                    || tutor.contains(Keys::Token, CardFields::Keys) 
                    || tutor.contains(Keywords::Convoke, CardFields::Keywords)
                    || tutor.contains(Keys::Counter, CardFields::Keys)
                    || (tutor.contains(Restrictions::All, CardFields::Restrictions) && tutor.contains(Zones::Graveyard, CardFields::Zones) )
                    || tutor.contains(Restrictions::Whenever, CardFields::Restrictions,) && tutor.contains(Keys::Cast, CardFields::Keys))
                    && tutor.name != String::from("Arcum Dagsson")
                    && tutor.name != String::from("Diabolic Intent"){

                        match color_restrictions(sdeck, tutor, typ) {
                            Some(mut result) => {targets.append(&mut result)},
                            None => {
                                for card in &sdeck.creatures {
                                    if card.name != tutor.name {
                                        targets.push(*card)
                                    }
                                }
                            },
                        }
                    }
                }
            },
            CardType::Enchantment(_) => {
                if tutor.contains(Keys::With, CardFields::Keys){
                    targets.append(&mut restrictions(deck, tutor, sdeck, CardType::Enchantment(None)));
                } else {
                    match color_restrictions(sdeck, tutor, typ) {
                        Some(mut result) => {targets.append(&mut result)},
                        None => {
                            for card in &sdeck.enchantments {
                                if card.name != tutor.name {
                                    targets.push(*card)
                                }
                            }
                        },
                    }
                }
            },
            CardType::Instant(_) => {
                if tutor.contains(Keys::With, CardFields::Keys){
                    targets.append(&mut restrictions(deck, tutor, sdeck, CardType::Instant(None)));
                } else {
                    match color_restrictions(sdeck, tutor, typ) {
                        Some(mut result) => {targets.append(&mut result)},
                        None => {
                            for card in &sdeck.instants {
                                if card.name != tutor.name {
                                    targets.push(*card)
                                }
                            }
                        },
                    }
                }
            },
            CardType::Sorcery(_)=> {
                if !tutor.contains(Restrictions::Only, CardFields::Restrictions) 
                && !tutor.contains(Keys::Activate, CardFields::Keys) {
                    if tutor.contains(Keys::With, CardFields::Keys){
                        targets.append(&mut restrictions(deck, tutor, sdeck, CardType::Sorcery(None)));
                    } else {
                        match color_restrictions(sdeck, tutor, typ) {
                            Some(mut result) => {targets.append(&mut result)},
                            None => {
                                for card in &sdeck.sorcerys {
                                    if card.name != tutor.name {
                                        targets.push(*card)
                                    }
                                }
                            },
                        }
                    }
                }
            },
            CardType::Planeswalker => {
                if tutor.contains(Keys::With, CardFields::Keys){
                    targets.append(&mut restrictions(deck, tutor, sdeck, CardType::Planeswalker));
                } else {
                    match color_restrictions(sdeck, tutor, typ) {
                        Some(mut result) => {targets.append(&mut result)},
                        None => {
                            for card in &sdeck.planeswalkers {
                                if card.name != tutor.name {
                                    targets.push(*card)
                                }
                            }
                        },
                    }
                }
            },
            CardType::Land(subtype) => {
                match subtype {
                    Some(subtypes) => {
                        for card in &deck.library {
                            for types in &card.cardtype {
                                match types {
                                    CardType::Land(subs) => {
                                        match subs {
                                            Some(subs) => {
                                                if subs == subtypes && !tutor.contains(Keys::Investigate, CardFields::Keys){
                                                    targets.push(card);
                                                } else {
                                                    for subtype in subtypes {
                                                        for sub in subs {
                                                            if sub == subtype && !tutor.contains(Keys::Investigate, CardFields::Keys) {
                                                                targets.push(&card);
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                            None => (),
                                        }
                                    },
                                    _ => (),
                                }
                            }
                        }    
                    },
                    None => {
                        for card in &sdeck.lands {
                            if card.name != tutor.name 
                            && !tutor.contains(CardType::Basic, CardFields::OracleType)
                            && !tutor.contains(Keys::Black, CardFields::Keys){
                                targets.push(*card);
                            } else if card.name != tutor.name && tutor.contains(CardType::Basic, CardFields::OracleType) { 
                                if card.contains(CardType::Basic, CardFields::CardType) 
                                && !tutor.contains(Keys::Black, CardFields::Keys){
                                    targets.push(*card);
                                }
                            }    
                        }
                    },
                }
            },
            CardType::Card => {
                if !( ( tutor.contains(CardType::Artifact(None), CardFields::OracleType) 
                    &&!tutor.contains(Keys::Investigate, CardFields::Keys) )
                || ( tutor.contains(CardType::Creature(None), CardFields::OracleType) 
                    && !(tutor.contains(Restrictions::All, CardFields::Restrictions) 
                    && tutor.contains(Zones::Graveyard, CardFields::Zones) ) )
                || tutor.contains(CardType::Enchantment(None), CardFields::OracleType)
                || tutor.contains(CardType::Instant(None), CardFields::OracleType)
                || tutor.contains(CardType::Sorcery(None), CardFields::OracleType)
                || ( tutor.contains(CardType::Land(None), CardFields::OracleType) 
                    && !tutor.contains(Restrictions::Control, CardFields::Restrictions) 
                    && !tutor.contains(Keys::Investigate, CardFields::Keys) )
                || tutor.contains(CardType::Planeswalker, CardFields::OracleType) )
                // Very special tutor. Got a lot of text and can't be passed through existing rules
                || tutor.name == String::from("Scrapyard Recombiner")
                // Falls somewhere through a rule, TODO: make it passing above rules 
                || tutor.name == String::from("Diabolic Intent"){
                    for card in &deck.library {    
                        if tutor.contains(Keys::NonLegendary, CardFields::Keys) {
                            if !card.legendary && card.name != tutor.name{
                                targets.push(card);
                            }
                        } else if tutor.contains(Keys::Legendary, CardFields::Keys) 
                        && !tutor.contains(Keys::NonLegendary, CardFields::Keys) {
                            if card.legendary && card.name != tutor.name{
                                targets.push(card);
                            }
                        }else { 
                            for sub in ArtifactSubtype::iter() {
                                if tutor.contains(&sub, CardFields::OracleText) {
                                   for typ in &card.cardtype {
                                    match &typ {
                                        &CardType::Artifact(artifact) => {
                                            match artifact{
                                                Some(subtypes) => {
                                                    for subtype in subtypes {
                                                        if subtype == &sub {
                                                            if card.name != tutor.name {
                                                                targets.push(card);
                                                            }
                                                        }
                                                    }
                                                },
                                                None => (),
                                            }
                                        },
                                        _ => (),
                                    }
                                   } 
                                }
                            }
                            for sub in CreatureSubtype::iter() {
                                if tutor.contains(&sub, CardFields::OracleText) && !tutor.contains(Keywords::Suspend, CardFields::Keywords){
                                    for typ in &card.cardtype {
                                        match &typ {
                                            &CardType::Creature(creature) => {
                                                match creature{
                                                    Some(subtypes) => {
                                                        for subtype in subtypes {
                                                            if subtype == &sub {
                                                                if card.name != tutor.name {
                                                                    targets.push(card);
                                                                }
                                                            }
                                                        }
                                                    },
                                                    None => (),
                                                }
                                            },
                                            _ => (),
                                        } 
                                    } 
                                 }
                            }
                            for sub in EnchantmentSubtype::iter() {
                                if tutor.contains(&sub, CardFields::OracleText) {
                                    for typ in &card.cardtype {
                                        match &typ {
                                            &CardType::Enchantment(enchantment) => {
                                                match enchantment {
                                                    Some(subtypes) => {
                                                        for subtype in subtypes {
                                                            if subtype == &sub {
                                                                if card.name != tutor.name {
                                                                    targets.push(card);
                                                                }
                                                            }
                                                        }
                                                    },
                                                    None => (),
                                                }
                                            },
                                            _ => (),
                                        }
                                    } 
                                 }
                            }
                            for sub in SpellSubtype::iter() {
                                if tutor.contains(&sub, CardFields::OracleText) {
                                    for typ in &card.cardtype {
                                         if( *typ == CardType::Instant(Some(vec![sub])) 
                                         || *typ == CardType::Sorcery(Some(vec![sub])))
                                         && card.name != tutor.name {
                                             targets.push(card);
                                         }
                                    } 
                                 }
                            }
                        }
                    }
                    if targets.len() == 0 {
                        for card in &deck.library {
                            if card.name != tutor.name{
                                targets.push(card);
                            }
                        }
                    } 
                }    
            },
            _ => (),
        }
        targets
    }
    fn restrictions<'deck>(deck: &'deck Deck, tutor: &Card, sdeck: &mut basic::Cardtype<'deck>, cardtype: CardType) -> Vec<&'deck Card> {
        let mut result: Vec<&Card> = Vec::new();
        // there are tutors who search keywords like flash but are not further restricted. So return restriction if found for keywords
        
        for keyword in Keywords::iter() {
            if tutor.contains(keyword, CardFields::OracleText) {
                for card in &deck.library {
                    match &card.keywords {
                        Some(cardkeywords) => {
                            for cardkeyword in cardkeywords {
                                if cardkeyword == &keyword && card.name != tutor.name {
                                    result.push(card);
                                }
                            }
                        },
                        None => (),
                    }
                }
            }
        }
        
        if tutor.contains(Keywords::Transmute, CardFields::Keywords){
            for card in &deck.library {
                if card.cmc == tutor.cmc && card.name != tutor.name{
                    result.push(&card);
                }
            }
        }else if tutor.contains(Restrictions::Cmc, CardFields::Restrictions)
        || tutor.contains(Restrictions::ManaValue, CardFields::Restrictions) 
        || tutor.contains(Restrictions::ManaCost, CardFields::Restrictions) {
            if !(tutor.contains(Restrictions::Plus, CardFields::Restrictions) 
            || tutor.contains(String::from("X"), CardFields::ManaCost)){
                // Special excuse for Urza's Saga, need to think about different representation of planeswalker and saga textboxes
                if tutor.name == String::from("Urza's Saga"){
                  for card in &sdeck.artifacts {
                      if card.cmc <= 1.0 {
                          result.push(*card);
                      }
                  } 
                } else if tutor.contains(Restrictions::Less, CardFields::Restrictions) 
                && !tutor.contains(Restrictions::Equal, CardFields::Restrictions)
                && !tutor.contains(Restrictions::OrLess, CardFields::Restrictions)  {
                   result.append(&mut less(deck, tutor, cardtype));

                }else if !tutor.contains(Restrictions::Less, CardFields::Restrictions) 
                && tutor.contains(Restrictions::Equal, CardFields::Restrictions) {
                   result.append(&mut equal(deck, tutor, cardtype)); 

                }else if( tutor.contains(Restrictions::Less, CardFields::Restrictions) 
                && tutor.contains(Restrictions::Equal, CardFields::Restrictions) )
                || tutor.contains(Restrictions::OrLess, CardFields::Restrictions) {
                    result.append(&mut less_or_equal(deck, tutor, cardtype));
                }  
            } else {
                match cardtype {        
                    CardType::Artifact(_) => {
                        for card in &sdeck.artifacts {
                            if card.name != tutor.name {
                                result.push(*card);
                            }
                        }
                    },
                    CardType::Creature(_) => {
                        match color_restrictions(sdeck, tutor, &cardtype) {
                            Some(mut targets) => {result.append(&mut targets)},
                            None => {
                                for card in &sdeck.creatures {
                                    if card.name != tutor.name {
                                        result.push(*card)
                                    }
                                }
                            },
                        }
                    },
                    CardType::Enchantment(_) => {
                        for card in &sdeck.enchantments {
                            if card.name != tutor.name {
                                result.push(*card);
                            }
                        }
                    },
                    CardType::Instant(_) => {
                        for card in &sdeck.instants {
                            if card.name != tutor.name {
                                result.push(*card);
                            }
                        }
                    },
                    CardType::Sorcery(_)=> {
                        for card in &sdeck.sorcerys {
                            if card.name != tutor.name {
                                result.push(*card);
                            }
                        }
                    },
                    CardType::Planeswalker => {
                        for card in &sdeck.planeswalkers {
                            if card.name != tutor.name {
                                result.push(*card);
                            }
                        }
                    }, 
                    _ => (),
                }   
            }  
        }else if tutor.contains(Keys::Power, CardFields::Keys) {
            if tutor.contains(Restrictions::Less, CardFields::Restrictions) 
            && !tutor.contains(Restrictions::Equal, CardFields::Restrictions) 
            && !tutor.contains(Restrictions::OrLess, CardFields::Restrictions) {
                result.append(&mut less(deck, tutor, cardtype)); 

            }else if !tutor.contains(Restrictions::Less, CardFields::Restrictions) 
            && tutor.contains(Restrictions::Equal, CardFields::Restrictions) {
                result.append(&mut equal(deck,tutor,cardtype));

            }else if (tutor.contains(Restrictions::Less, CardFields::Restrictions) 
            && tutor.contains(Restrictions::Equal, CardFields::Restrictions) )
            || tutor.contains(Restrictions::OrLess, CardFields::Restrictions){
                result.append(&mut less_or_equal(deck,tutor,cardtype)); 
            }
       
        } else if tutor.contains(Keys::Toughness, CardFields::Keys){
            if tutor.contains(Restrictions::Less, CardFields::Restrictions) 
            && !tutor.contains(Restrictions::Equal, CardFields::Restrictions) 
            && !tutor.contains(Restrictions::OrLess, CardFields::Restrictions) {
                result.append(&mut less(deck, tutor, cardtype)); 

            }else if !tutor.contains(Restrictions::Less, CardFields::Restrictions) 
            && tutor.contains(Restrictions::Equal, CardFields::Restrictions) {
                result.append(&mut equal(deck, tutor, cardtype)); 

            }else if( tutor.contains(Restrictions::Less, CardFields::Restrictions) 
            && tutor.contains(Restrictions::Equal, CardFields::Restrictions) )
            || tutor.contains(Restrictions::OrLess, CardFields::Restrictions)  {
                result.append(&mut less_or_equal(deck,tutor,cardtype)); 
            }
            // tutor can contain instant and card with flash, so through restriction it will end up here
            // flash is linked in the keyword for loop and then will fall through all if's and end up in else where all
            // instants will be linked to it
        } else {
            if tutor.name != String::from("Scrapyard Recombiner") {  
                match cardtype {
                    CardType::Artifact(_) => {
                        for card in &sdeck.artifacts {
                            result.push(*card);
                        }
                    },
                    CardType::Creature(_) => {
                        for card in &sdeck.creatures {
                            result.push(*card);
                        } 
                    },
                    CardType::Instant(_) => {
                        for card in &sdeck.instants {
                            result.push(*card);
                        }
                    },
                    CardType::Sorcery(_) => {
                        for card in &sdeck.sorcerys {
                            result.push(*card);
                        }
                    },
                    CardType::Planeswalker => {
                        for card in &sdeck.planeswalkers {
                            result.push(*card);
                        }
                    },
                    CardType::Enchantment(_) => {
                        for card in &sdeck.enchantments {
                            result.push(*card);
                        }
                    },
                    _ => (),
                }
            }
        }
        result
    }
    fn less<'deck>(deck: &'deck Deck, tutor: &Card, cardtype: CardType) -> Vec<&'deck Card> {
        let mut result: Vec<&Card> = Vec::new();
        match &tutor.restrictions {
            Some(restrictions) => {
                for restriciton in restrictions {
                    match restriciton {
                        /* 
                        Restrictions::OneStr => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 1.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::TwoStr => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 2.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::ThreeStr => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 3.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        */
                        Restrictions::Zero => (),
                        Restrictions::One => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 1.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Two => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 2.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Three => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 3.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Four => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 4.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Five => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 5.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Six => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 6.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Seven => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 7.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Eight => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 8.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Nine => {for card in &deck.library {
                            if card.name != tutor.name {
                                if card.cmc < 9.0 && card.contains(&cardtype, CardFields::CardType){
                                    result.push(&card);
                                }
                            }
                        }},
                        Restrictions::Ten => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 10.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Eleven => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 11.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Twelve => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc < 12.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        _ => (),
                    }
                }
            },
            None => (),
        }
        result
    }
    fn equal<'deck>(deck: &'deck Deck, tutor: &Card, cardtype: CardType) -> Vec<&'deck Card>{
        let mut result: Vec<&Card> = Vec::new();
        match &tutor.restrictions {
            Some(restrictions) => {
                for restriciton in restrictions {
                    match restriciton {
                        /*
                        Restrictions::OneStr => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 1.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::TwoStr => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 2.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::ThreeStr => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 3.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        */
                        Restrictions::Zero => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 0.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::One => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 1.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Two => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 2.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Three => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 3.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Four => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 4.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Five => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 5.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Six => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 6.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Seven => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 7.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Eight => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 8.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Nine => {for card in &deck.library {
                            if card.name != tutor.name {
                                if card.cmc == 9.0 && card.contains(&cardtype, CardFields::CardType){
                                    result.push(&card);
                                }
                            }
                        }},
                        Restrictions::Ten => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 10.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Eleven => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 11.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Twelve => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 12.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        _ => (),
                    }
                }
            },
            None => (),
        }
        result
    }
    fn less_or_equal<'deck>(deck: &'deck Deck, tutor: &Card, cardtype: CardType) -> Vec<&'deck Card> {
        let mut result: Vec<&Card> = Vec::new();
        match &tutor.restrictions {
            Some(restrictions) => {
                for restriciton in restrictions {
                    match restriciton {
                        /*
                        Restrictions::OneStr => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc <= 1.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::TwoStr => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc <= 2.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::ThreeStr => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc <= 3.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        */
                        Restrictions::Zero => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc == 0.0 {
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::One => {
                            for card in &deck.library {
                                if card.name != tutor.name && card.contains(&cardtype, CardFields::CardType){
                                    if card.cmc <= 1.0 {
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Two => {
                            for card in &deck.library {
                                if card.name != tutor.name && card.contains(&cardtype, CardFields::CardType){
                                    if card.cmc <= 2.0 {
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Three => {
                            for card in &deck.library {
                                if card.name != tutor.name && card.contains(&cardtype, CardFields::CardType){
                                    if card.cmc <= 3.0 {
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Four => {
                            for card in &deck.library {
                                if card.name != tutor.name && card.contains(&cardtype, CardFields::CardType){
                                    if card.cmc <= 4.0 {
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Five => {
                            for card in &deck.library {
                                if card.name != tutor.name && card.contains(&cardtype, CardFields::CardType){
                                    if card.cmc <= 5.0 {
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Six => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc <= 6.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Seven => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc <= 7.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Eight => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc <= 8.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Nine => {for card in &deck.library {
                            if card.name != tutor.name {
                                if card.cmc <= 9.0 && card.contains(&cardtype, CardFields::CardType){
                                    result.push(&card);
                                }
                            }
                        }},
                        Restrictions::Ten => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc <= 10.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Eleven => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc <= 11.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        Restrictions::Twelve => {
                            for card in &deck.library {
                                if card.name != tutor.name {
                                    if card.cmc <= 12.0 && card.contains(&cardtype, CardFields::CardType){
                                        result.push(&card);
                                    }
                                }
                            }
                        },
                        _ => (),
                    }
                }
            },
            None => (),
        }
        result
    }
    fn color_restrictions<'deck>(sdeck: &mut basic::Cardtype<'deck>, tutor: &Card ,cardtype: &CardType) -> Option<Vec<&'deck Card>> {
       
        let mut targets: Vec<&Card> = Vec::new();
        let mut sorted_deck: &Vec<&Card> = &Vec::new();

        match cardtype {
            CardType::Artifact(_) => {sorted_deck = &sdeck.artifacts},
            CardType::Creature(_) => {sorted_deck = &sdeck.creatures},
            CardType::Enchantment(_) =>{sorted_deck = &sdeck.enchantments},
            CardType::Sorcery(_) => {sorted_deck = &sdeck.sorcerys},
            CardType::Instant(_) => {sorted_deck = &sdeck.instants},
            CardType::Planeswalker => {sorted_deck = &sdeck.planeswalkers},
            _ => (),
        }

        match &tutor.keys {
            Some(keys) => {
                for key in keys {
                    match key {
                        Keys::SWhite => {
                            for card in sorted_deck {
                                if card.name != tutor.name && card.contains(Colors::White, CardFields::ManaCost) {
                                    targets.push(*card)
                                }
                            }
                        },
                        Keys::SBlue => {
                            for card in sorted_deck {
                                if card.name != tutor.name && card.contains(Colors::Blue, CardFields::ManaCost) {
                                    targets.push(*card)
                                }
                            }
                        },
                        Keys::SBlack => {
                            for card in sorted_deck  {
                                if card.name != tutor.name && card.contains(Colors::Black, CardFields::ManaCost) {
                                    targets.push(*card)
                                }
                            }
                        },
                        Keys::SRed => {
                            for card in sorted_deck  {
                                if card.name != tutor.name && card.contains(Colors::Red, CardFields::ManaCost) {
                                    targets.push(*card)
                                }
                            }
                        },
                        // Dryad arbor is a Creatur - Forest Dryad, therefore it's green but not mentioned on the card.
                        Keys::SGreen => {
                            for card in sorted_deck  {
                                if (card.name != tutor.name && card.contains(Colors::Green, CardFields::ManaCost) )
                                || card.name == String::from("Dryad Arbor"){
                                    targets.push(*card)
                                }
                            }
                        },
                        _ => (),
                    }
                }
            },
            None => (),
        }
        if targets.len() != 0 {
            return Some(targets);
        }else {
            return None;
        }
    }
}
/****************************************** Eval Powerlevel **************************************************/
pub mod powerlevel {}