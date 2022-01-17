#![allow(non_camel_case_types)]


use std::{fmt::{self, Debug, Display}, error, fs::*};
use strum_macros::{EnumIter};
use serde::{Serialize, Deserialize};


use crate::logic;


/************************************** Macros ***********************************************************/

macro_rules! impl_fmt {
    (for $($t:ty), +) => {
        $(impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", self)

            }
        })*
    };
}

#[macro_export]
macro_rules! println_verbose {
    ($verbose:expr, $($arg:tt)*) => {
        if $verbose {
            println!($($arg)*);
        }
    };
}
/************************************** Errors **********************************************************/
#[derive(Debug)]
pub enum CEerror {
    FailImportDeck(String),
    APIError,
    DatabaseError,
    FetchValueError,
    CardNotFound,
    ComboError,
    HyperGeoFailed,
}
impl_fmt!(for CEerror);
impl error::Error for CEerror {} 

pub type CEResult<T> = Result<T, CEerror>;

/********************************** Magic Card Types *****************************************************/

#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum CardType{
    Instant(Option<Vec<SpellSubtype>>),
    Sorcery(Option<Vec<SpellSubtype>>),
    Artifact(Option<Vec<ArtifactSubtype>>),
    Creature(Option<Vec<CreatureSubtype>>),
    Enchantment(Option<Vec<EnchantmentSubtype>>),
    Land(Option<Vec<LandSubtype>>),
    Planeswalker,
    Token,
    Basic,
    InvalidCardType, 
    Card,
}
impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            &CardType::Creature(_t) => write!(f, "{}","Creature"),
            &CardType::Instant(_t) => write!(f, "{}","Instant"),
            &CardType::Sorcery(_t) => write!(f, "{}","Sorcery"),
            &CardType::Artifact(_t) => write!(f, "{}","Artifact"),
            &CardType::Enchantment(_t) => write!(f, "{}","Enchantment"),
            &CardType::Land(_t) => write!(f, "{}","Land"),
            _ => write!(f, "{:?}", self),
        }
    }
}
#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize, Copy)]
pub enum ArtifactSubtype{
    Blood, 
    Clue, 
    Contraption, 
    Equipment, 
    Food, 
    Fortification, 
    Gold, 
    Treasure, 
    Vehicle,
}
#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize, Copy)]
pub enum SpellSubtype{
    Adventure, 
    Arcane, 
    Lesson, 
    Trap,
}
#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize, Copy)]
pub enum CreatureSubtype{
Advisor,
Aetherborn,
Ally,
Angel,
Antelope,
Ape,
Archer,
Archon,
Army,
Artificer,
Assassin,
Assembly_Worker, // Name on card: Assembly-Worker
Atog,
Aurochs,
Avatar,
Azra,
Badger,
Barbarian,
Bard,
Basilisk,
Bat,
Bear,
Beast,
Beeble,
Beholder,
Berserker,
Bird,
Blinkmoth,
Boar,
Bringer,
Brushwagg,
Camarid,
Camel,
Caribou,
Carrier,
Cat,
Centaur,
Cephalid,
Chimera,
Citizen,
Cleric,
Cockatrice,
Construct,
Coward,
Crab,
Crocodile,
Cyclops,
Dauthi,
Demigod,
Demon,
Deserter,
Devil,
Dinosaur,
Djinn,
Dog,
Dragon,
Drake,
Dreadnought,
Drone,
Druid,
Dryad,
Dwarf,
Efreet,
Egg,
Elder,
Eldrazi,
Elemental,
Elephant,
Elf,
Elk,
Eye,
Faerie,
Ferret,
Fish,
Flagbearer,
Fox,
Fractal,
Frog,
Fungus,
Gargoyle,
Germ,
Giant,
Gnoll,
Gnome,
Goat,
Goblin,
God,
Golem,
Gorgon,
Graveborn,
Gremlin,
Griffin,
Hag,
Halfling,
Hamster,
Harpy,
Hellion,
Hippo,
Hippogriff,
Homarid,
Homunculus,
Horror,
Horse,
Human,
Hydra,
Hyena,
Illusion,
Imp,
Incarnation,
Inkling,
Insect,
Jackal,
Jellyfish,
Juggernaut,
Kavu,
Kirin,
Kithkin,
Knight,
Kobold,
Kor,
Kraken,
Lamia,
Lammasu,
Leech,
Leviathan,
Lhurgoyf,
Licid,
Lizard,
Manticore,
Masticore,
Mercenary,
Merfolk,
Metathran,
Minion,
Minotaur,
Mole,
Monger,
Mongoose,
Monk,
Monkey,
Moonfolk,
Mouse,
Mutant,
Myr,
Mystic,
Naga,
Nautilus,
Nephilim,
Nightmare,
Nightstalker,
Ninja,
Noble,
Noggle,
Nomad,
Nymph,
Octopus,
Ogre,
Ooze,
Orb,
Orc,
Orgg,
Otter,
Ouphe,
Ox,
Oyster,
Pangolin,
Peasant,
Pegasus,
Pentavite,
Pest,
Phelddagrif,
Phoenix,
Phyrexian,
Pilot,
Pincher,
Pirate,
Plant,
Praetor,
Prism,
Processor,
Rabbit,
Ranger,
Rat,
Rebel,
Reflection,
Rhino,
Rigger,
Rogue,
Sable,
Salamander,
Samurai,
Sand,
Saproling,
Satyr,
Scarecrow,
Scion,
Scorpion,
Scout,
Sculpture,
Serf,
Serpent,
Servo,
Shade,
Shaman,
Shapeshifter,
Shark,
Sheep,
Siren,
Skeleton,
Slith,
Sliver,
Slug,
Snake,
Soldier,
Soltari,
Spawn,
Specter,
Spellshaper,
Sphinx,
Spider,
Spike,
Spirit,
Splinter,
Sponge,
Squid,
Squirrel,
Starfish,
Surrakar,
Survivor,
Tentacle,
Tetravite,
Thalakos,
Thopter,
Tiefling,
Thrull,
Treefolk,
Trilobite,
Triskelavite,
Troll,
Turtle,
Unicorn,
Vampire,
Vedalken,
Viashino,
Volver,
Wall,
Warlock,
Warrior,
Weird,
Werewolf,
Whale,
Wizard,
Wolf,
Wolverine,
Wombat,
Worm,
Wraith,
Wurm,
Yeti,
Zombie,
Zubera,
}
#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize, Copy)]
pub enum EnchantmentSubtype{
    Aura, 
    Cartouche, 
    Class, 
    Curse, 
    Rune, 
    Saga, 
    Shrine, 
    Shard,
}
#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize, Copy)]
pub enum LandSubtype{
    Plains, 
    Island, 
    Swamp, 
    Mountain, 
    Forest,
    Desert, 
    Gate, 
    Lair, 
    Locus, 
    UrzasMine, 
    UrzasPowerPlant, 
    UrzasTower,
}
#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize, Copy)]
pub enum Stats{
    Power(u8),
    Toughness(u8),
    Loyality(u8),
}
impl Stats {
    pub fn to_key(&self) -> Keys {
        match self {
            Stats::Power(_) => Keys::Power,
            Stats::Toughness(_) => Keys::Toughness,
            Stats::Loyality(_) => Keys::Loyality,
        }
    }
}

impl_fmt!(for  ArtifactSubtype, SpellSubtype, CreatureSubtype, EnchantmentSubtype, LandSubtype, Stats);

/*************************************** Keywords, Zones, Restrictions and Colours *************************************/

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Serialize, Deserialize, Copy)]
pub enum Keys{
    Remove,
    Surveil,
    Legendary,
    NonLegendary,
    With,
    Loyality,
    Power,
    Toughness,
    Activate,
    Cost, 
    Play,
    Redirect,
    Owner,
    Put,
    Create,
    Spell,
    Turns,
    Additional,
    Phase,
    Top,
    Look,
    Add,
    Cast,
    Exile,
    Destroy,
    Return,
    Draw,
    Counter,
    Damage,
    Attach,
    Fight,
    Mill,
    Sacrifice,
    Scry,
    Tap,
    Tapped,
    Untap,
    Discard,
    Search,
    Player,
    Opponent,
    Token,
    Copy,
    White,
    Blue,
    Black,
    Red,
    Green,
    SWhite,
    SBlue,
    SBlack,
    SRed,
    SGreen,
    Colourless,
//    AnyColor,
//    EachColor,
    ETB,
    OneMana,
    AnyMana,
}
impl fmt::Display for Keys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Keys::Tap => write!(f, "{}","{T}"),
            &Keys::Black => write!(f, "{}","{B}"),
            &Keys::Blue => write!(f, "{}","{U}"),
            &Keys::White => write!(f, "{}","{W}"),
            &Keys::Green => write!(f, "{}","{G}"),
            &Keys::Red => write!(f, "{}","{R}"),
            &Keys::Colourless => write!(f, "{}","{C}"),
//            &Keys::AnyColor => write!(f, "{}", "any color"),
//            &Keys::EachColor => write!(f, "{}", "each color"),
            &Keys::ETB => write!(f, "{}", "enters the battlefield"),
            &Keys::AnyMana => write!(f, "{}", "any one color"),
            &Keys::OneMana => write!(f, "{}", "one mana of any"),
            &Keys::Redirect => write!(f, "{}", "choose new target"),
            &Keys::SWhite => write!(f, "{}", "white"),
            &Keys::SBlue => write!(f, "{}", "blue"),
            &Keys::SBlack => write!(f, "{}", "black"),
            &Keys::SRed => write!(f, "{}", "red"),
            &Keys::SGreen => write!(f, "{}", "green"),

            _ => write!(f, "{:?}", self),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Serialize, Deserialize, Copy)]
pub enum Keywords{
    Imprint,
    Deathtouch,
    Defender,
    Double_Strike,
    Enchant,
    Equip,
    First_Strike,
    Flash,
    Flying,
    Haste,
    Hexproof,
    Indestructible,
    Intimidate,
    Landwalk,
    Lifelink,
    Protection,
    Reach,
    Shroud,
    Trample,
    Vigilance,
    Ward,
    Banding,
    Rampage,
    Cumulative_Upkeep,
    Flanking,
    Phasing,
    Buyback,
    Shadow,
    Cycling,
    Echo,
    Horsemanship,
    Fading,
    Kicker,
    Flashback,
    Madness,
    Fear,
    Morph,
    Amplify,
    Provoke,
    Storm,
    Affinity,
    Entwine,
    Modular,
    Sunburst,
    Bushido,
    Soulshift,
    Splice,
    Offering,
    Ninjutsu,
    Epic,
    Convoke,
    Dredge,
    Transmute,
    Bloodthirst,
    Haunt,
    Replicate,
    Forecast,
    Graft,
    Recover,
    Ripple,
    SplitSecond,
    Suspend,
    Vanishing,
    Absorb,
    Aura_Swap,
    Delve,
    Fortify,
    Frenzy,
    Gravestorm,
    Poisonous,
    Transfigure,
    Champion,
    Changeling,
    Evoke,
    Hideaway,
    Prowl,
    Reinforce,
    Conspire,
    Persist,
    Wither,
    Retrace,
    Devour,
    Exalted,
    Unearth,
    Cascade,
    Annihilator,
    Level_Up,
    Rebound,
    Totem_Armor,
    Infect,
    Battle_Cry,
    Living_Weapon,
    Undying,
    Miracle,
    Soulbond,
    Overload,
    Scavenge,
    Unleash,
    Cipher,
    Evolve,
    Extort,
    Tribute,
    Dethrone,
    Hidden_Agenda,
    Outlast,
    Prowess,
    Dash,
    Exploit,
    Menace,
    Renown,
    Awaken,
    Devoid,
    Ingest,
    Myriad,
    Surge,
    Skulk,
    Emerge,
    Escalate,
    Melee,
    Crew,
    Fabricate,
    Partner,
    Undaunted,
    Improvise,
    Aftermath,
    Embalm,
    Eternalize,
    Afflict,
    Ascend,
    Assist,
    Jump_Start,
    Mentor,
    Afterlife,
    Riot,
    Spectacle,
    Escape,
    Companion,
    Mutate,
    Encore,
    Boast,
    Foretell,
    Demonstrate,
    Daybound,
    Nightbound,
    Disturb,
    Decayed,
    Cleave,
    Training,
}
#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Serialize, Deserialize, Copy)]
pub enum Restrictions {
    OrLess,
    Dont,
    Exchange,
    Control,
    Get,
    You,
    Own,
    Instead,
    Every,
    Pay,
    Can,
    CanT,
    Whenever,
    Another,
    May,
    If,
    Target,
    Each,
    All,
    Cmc,
    ManaValue,
    ManaCost,
    OneStr,
    TwoStr,
    ThreeStr,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Xis, // to x is 
    Xor, // to x or
    PlusSymbol,
    MinusSymbol,
    Minus,
    Only,
    Non,
    Plus,
    Upkeep,
    Drawstep,
    Untapstep,
    MainPhase,
    Combat,
    Endstep,
    Untap,
    During,
    AtBeginnOf,
    MinusXX, // to -x/-x
    PlusXX,// to +x/+x
    Until,
    Reveal,
    CommanderControl,
    Double,
    That,
    Many,
    More,
    Than,
    Equal,
    Trigger,
    Without,
    GainLife,
    Power,
    Toughness,
    Less,
    EoT,
    Die,
    After, 
}
impl fmt::Display for Restrictions{
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Restrictions::OneStr => write!(f, "{}", "One"),
            &Restrictions::TwoStr => write!(f, "{}", "Two"),
            &Restrictions::ThreeStr => write!(f, "{}", "Three"),
            &Restrictions::EoT => write!(f, "{}", "end of turn"),
            &Restrictions::CanT => write!(f,"{}", "Can't"),
            &Restrictions::Zero => write!(f, "{}", "0"),
            &Restrictions::One => write!(f, "{}", "1"),
            &Restrictions::Two => write!(f, "{}", "2"),
            &Restrictions::Three => write!(f, "{}", "3"),
            &Restrictions::Four => write!(f, "{}", "4"),
            &Restrictions::Five => write!(f, "{}", "5"),
            &Restrictions::Six => write!(f, "{}", "6"),
            &Restrictions::Seven => write!(f, "{}", "7"),
            &Restrictions::Eight => write!(f, "{}", "8"),
            &Restrictions::Nine => write!(f, "{}", "9"),
            &Restrictions::Ten => write!(f, "{}", "10"),
            &Restrictions::Eleven => write!(f, "{}", "11"),
            &Restrictions::Twelve => write!(f, "{}", "12"),
            &Restrictions::AtBeginnOf => write!(f, "{}", "At the begin of"),
            &Restrictions::Cmc => write!(f, "{}", "cmc"),
            &Restrictions::ManaValue => write!(f, "{}", "Mana Value"),
            &Restrictions::ManaCost => write!(f, "{}", "Mana Cost"),
            &Restrictions::Xis => write!(f, "{}", "X is"),
            &Restrictions::Xor => write!(f, "{}", "X or"),
            &Restrictions::PlusSymbol => write!(f, "{}", "+"),
            &Restrictions::MinusSymbol => write!(f, "{}", "-"),
            &Restrictions::MainPhase => write!(f, "{}", "Main Phase"),
            &Restrictions::Drawstep => write!(f, "{}", "Draw Step"),
            &Restrictions::MinusXX => write!(f, "{}", "-X/-X"),
            &Restrictions::PlusXX => write!(f, "{}", "+X/+X"),
            &Restrictions::CommanderControl => write!(f, "{}", "control your commander"),
            &Restrictions::GainLife => write!(f, "{}", "gain life"),
            &Restrictions::Dont => write!(f, "{}", "Don't"), 
            &Restrictions::OrLess => write!(f, "{}", "or less"),
            _ => write!(f, "{:?}", self),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Serialize, Deserialize, Copy)]
pub enum Zones{
    Battlefield,
    Hand,
    Exile,
    Graveyard,
    CommandZone,
    Library,
}
#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Serialize, Deserialize, Hash, Copy)]
pub enum Colors {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colourless,
//    AnyColor,
//    EachColor,
    AnyMana,
    OneMana,
}
impl_fmt!(for Zones, Keywords);
impl Colors {
    pub fn to_key(&self) -> Keys {
        match self {
            Colors::White => Keys::White,
            Colors::Blue => Keys::Blue,
            Colors::Black => Keys::Black,
            Colors::Red => Keys::Red,
            Colors::Green => Keys::Green,
            Colors::Colourless => Keys::Colourless,
//            Colors::AnyColor => Keys::AnyColor,
//            Colors::EachColor => Keys::EachColor,
            Colors::AnyMana => Keys::AnyMana,
            Colors::OneMana => Keys::OneMana
        }
    }
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Colors::Black => "{B}",
                Colors::Blue => "{U}",
                Colors::White => "{W}",
                Colors::Green => "{G}",
                Colors::Red => "{R}",
                Colors::Colourless => "{C}",
//               Colors::AnyColor => "any color",
//               Colors::EachColor => "each color",
                Colors::AnyMana => "any one color",
                Colors::OneMana => "one mana of any",
            }
        )
    }
}


/************************************** Card and Deck ***************************************************/



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub cmc: f32,
    pub mana_cost: String,
    pub name: String,
    pub cardtype: Vec<CardType>,
    pub legendary: bool,
    pub stats: Option<Vec<Stats>>,
    pub commander: bool,
    pub backside: Option<Box<Card>>,
    pub oracle_text: String,
    pub keys: Option<Vec<Keys>>, 
    pub zones: Option<Vec<Zones>>,
    pub keywords: Option<Vec<Keywords>>,
    pub oracle_types: Option<Vec<CardType>>,
    pub restrictions: Option<Vec<Restrictions>>,
}
#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Hash)]
pub enum CardFields {
    Cmc,
    ManaCost,
    Name,
    CardType,
    Legendary,
    Stats,
    Commander,
    Backside,
    OracleText,
    Keys,
    Zones,
    Keywords,
    OracleType,
    Restrictions
}

impl Card {
    pub fn new() -> Self {
        Card {
            cmc: 0.0,
            mana_cost: String::from(""),
            name: String::from(""),
            cardtype: vec![CardType::InvalidCardType],
            legendary: false,
            stats: None,
            commander: false,
            backside: None,
            oracle_text: String::from(""),
            keys: None,
            zones: None,
            keywords: None,
            oracle_types: None,
            restrictions: None,
         }
     }
    pub fn make(card: &String, commander: bool) -> CEResult<Self> {
        use serde_json::Value;
        use crate::logic::card_build;
        // card will contain backside From here we can just pass through to build and backside: Box<card_build::build(v["backside"], commander)>
        match serde_json::from_str(&card) {
            Ok(t) => {
                let v: Value = t;
                let mut mdfc: Option<&serde_json::Value> = None;

                if v["layout"] == "modal_dfc".to_string()
                || v["layout"] == "transform".to_string()
                || v["layout"] == "split".to_string()
                || v["type_line"].to_string().contains("//"){
                    mdfc = Some(&v["card_faces"][1]);
                }
 
                //Check if card was found
                if v["code"] == String::from("not_found") { 
                    return Err(CEerror::CardNotFound);
                } else {
                    return Ok(card_build::build(&v, commander, mdfc));
                }
            },
            Err(_) => Err(CEerror::FetchValueError),
        }
     }
    pub fn contains<T: Display + Eq + PartialEq> (&self, search: T , field: CardFields) -> bool {
        match field {
            CardFields::Cmc => {
                if self.cmc == search.to_string().parse::<f32>().expect("Card.find() in CardFields::Cmc not a f32") {
                    return true;
                } else {
                    return false;
                }
            },
            CardFields::ManaCost => { 
                if self.mana_cost.contains(&search.to_string().replace("\"", "")) {
                    return true;
                } else {
                    return false;
                }
            },
            CardFields::Name => (
                if self.name.contains(&search.to_string().replace("\"", "")) {
                    return true;
                } else {
                    return false;
                }
            ),
            CardFields::CardType => {
                for types in &self.cardtype {
                    if types.to_string() == search.to_string() {
                        return true;
                    } 
                }
                return false;
            },
            CardFields::Legendary => (return self.legendary),
    
            CardFields::Stats=> {
                match &self.stats{
                    Some(stats) => {
                        for stat in stats {
                            if stat.to_string() == search.to_string() {
                                return true;
                            }
                        }
                        return false;
                    },
                    None => return false,
                }                
            },
            CardFields::Commander=> return self.commander,
            CardFields::Backside=> (
                match self.backside {
                    Some(_) => return true,
                    None => return false,
                }

            ),
            CardFields::OracleText=> {
                if self.oracle_text.contains(&search.to_string()) {
                    return true;
                }
                return false;
            },
            CardFields::Keys=> {
                match &self.keys {
                    Some(keys)=> {
                        for key in keys {
                            if key.to_string() == search.to_string() {
                                return true;
                            }
                        }
                        return false;
                    },
                    None => return false,
                }
                
            },
            CardFields::Zones=> {
                match &self.zones {
                    Some(zones) => {
                        for zone in zones {
                            if zone.to_string() == search.to_string() {
                                return true;
                            }
                        }
                        return false;
                    },
                    None => return false,
                }
               
            },
            CardFields::Keywords=> {
                match &self.keywords {
                    Some(keywords) => {
                        for keyword in keywords {
                            if keyword.to_string() == search.to_string() {
                                return true;
                            }
                        }
                        return false;
                    },
                    None => return false,
                } 
            },
            CardFields::OracleType=> {
                match &self.oracle_types {
                    Some(cardtype) => {
                        for types in cardtype {
                            if types.to_string() == search.to_string() {
                                return true;
                            }
                        }
                        return false;
                    },
                    None => return false,
                }
                
            },
            CardFields::Restrictions=> {
                match &self.restrictions {
                    Some(restrictions) => {
                        for restriction in restrictions {
                            if restriction.to_string() == search.to_string() {
                                return true;
                            }
                        }
                        return false;
                    }, 
                    None => return false,
                }
                
            },
         }
     }

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck{
    pub name: String,
    pub commander: Vec<Card>,
    pub library: Vec<Card>,
}
impl Deck {
    pub fn check(mut deck: Deck, verbose: bool, offline: bool) -> CEResult<Deck>{
        use crate::import::{scryfall, user_import};
        use crate::database;
        
        if deck.commander.len() == 0 {
            let mut commander_new: Vec<Card> = Vec::new();

            println!("Found missing commander for {}! Please type your commander(Commander1-Commander2): ", &deck.name);
            
            let input = user_import::user_input();
            let mut index = 0;
            let mut index_buffer: Vec<usize> = Vec::new();      
            // most cases, card is in provided decklist just not *CMDR* setted
            for card in &deck.library {
                for commmander in &input {
                    if card.name.to_lowercase().contains(&commmander.to_lowercase()) {
                        deck.commander.push(card.clone());
                        index_buffer.push(index);
                        println_verbose!(verbose, "Found commander in deck");
                    }
                }
                index += 1;
            }
            // only removes if commander found in deck
            for index in &index_buffer {
                deck.library.remove(*index);
            }

            if index_buffer.len() == 0 {
                if offline {
                    match database::load() {
                        Ok(t) => {
                            for commander in input {
                            commander_new.push(Card::make(&database::get(&commander, &t)?.to_string(), true)?);
                            }
                        },
                        Err(_) => {
                            panic!("Database can not be loaded, unrecoverable error for offline-modus");
                        },
                    }
                } else {
                    for commander in input {
                        commander_new.push(Card::make(&scryfall::get(&commander)?, true)?);
                        println_verbose!(verbose, "{:?}", commander_new);
                    }
                }

                deck.commander = commander_new;

                println_verbose!(verbose, "Library: {}, Commander: {:?}", deck.library.len(), deck.commander);

                if deck.commander.len() + deck.library.len() == 100 {
                    println_verbose!(verbose, "Deck complete, save deck");
                    Deck::save(&deck);
                    return Ok(deck);
                } else {
                    panic!("Deck size not 100, deck corrupted");
                }

            } else {
                println_verbose!(verbose, "Deckcheck completed: Ok");
                Deck::save(&deck);
                Ok(deck)
            }
        } else {
            println_verbose!(verbose, "Deckcheck completed: Ok");
            Deck::save(&deck);
            Ok(deck)
        }
    }
    pub fn make(input: String)-> CEResult<Deck>{
       use logic::thread_fn::deck; 
       match deck(input) {
           Ok(t) => Ok(t),
           Err(e) => Err(e),
       }
    }  
    pub fn load(identifier: &String, verbose: bool) -> CEResult<Deck> {

        let save = String::from("save/");
        let path = format!("{}{}", save, identifier);
      
        match File::open(path) {
            Ok(t) => {
                let deck = serde_json::from_reader(t).expect("Saved deck no proper json");
                println_verbose!(verbose, "Deck successfully opened"); 
               
                Ok(deck) 
            },
            Err(_) => Err(CEerror::DatabaseError),
        }
    }
    pub fn save(deck: &Deck){
        let save = String::from("save/");
        serde_json::to_writer(&File::create(format!("{}{}",save, deck.name)).expect("Can not folder save/ not found"),
        &deck).expect("Can not write Deck"); 
    }
    pub fn new(name: String, commander: Vec<Card>, library: Vec<Card>) -> Deck {
        Deck{
            name: name,
            commander: commander,
            library: library,
        }
    }
}
