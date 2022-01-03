
use std::{fmt, error, sync::Arc };
use strum_macros::EnumIter;
use std::{thread, sync::mpsc};
use serde::{Serialize, Deserialize};
use std::{fs::*, io::{prelude::*, BufReader}};

use crate::import::user_import;
use crate::logic::{thread_fn, self};


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
    Instant(Vec<Option<SpellSubtype>>),
    Sorcery(Vec<Option<SpellSubtype>>),
    Artifact(Vec<Option<ArtifactSubtype>>),
    Creature(Vec<Option<CreatureSubtype>>),
    Enchantment(Vec<Option<EnchantmentSubtype>>),
    Land(Vec<Option<LandSubtype>>),
    Planeswalker,
    InvalidCardType,
}
#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize)]
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
#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum SpellSubtype{
    Adventure, 
    Arcane, 
    Lesson, 
    Trap,
}
#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize)]
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
AssemblyWorker, // Name on card: Assembly-Worker
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
#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize)]
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
#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize)]
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
#[derive(Debug, Clone,Eq, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum Stats{
    Power(u8),
    Toughness(u8),
    Loyality(u8),
}

impl_fmt!(for CardType, ArtifactSubtype, SpellSubtype, CreatureSubtype, EnchantmentSubtype, LandSubtype, Stats);

/*************************************** Keywords, Zones, effects and Colours *************************************/

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum Keys{
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
    Target,
    Player,
    Opponent,
    Token,
    All,
    Each,
    Every,
}

// pub enum Effects{}
#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum Keywords{
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
 Daybound_and_Nightbound,
 Disturb,
 Decayed,
 Cleave,
 Training,
}

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum Zones{
    Battlefield,
    Hand,
    Exile,
    Graveyard,
    CommandZone,
    Library,
}
#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum Colours {
    White,
    Blue,
    Black,
    Red,
    Green,
}
impl_fmt!(for Keys, Zones, Keywords);

impl fmt::Display for Colours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Colours::Black => "{B}",
                Colours::Blue => "{U}",
                Colours::White => "{W}",
                Colours::Green => "{G}",
                Colours::Red => "{R}",
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
    pub backside: Box<Option<Card>>,
    pub oracle_text: String,
    pub keys: Option<Vec<Keys>>, 
    pub zones: Option<Vec<Zones>>,
    pub keywords: Option<Vec<Keywords>>,
    pub oracle_types: Option<Vec<CardType>>,
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
            backside: Box::new(None),
            oracle_text: String::from(""),
            keys: None,
            zones: None,
            keywords: None,
            oracle_types: None,
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
                || v["type_line"].to_string().contains("//"){
                    mdfc = Some(&v["card_faces"][1]);
                }

                println!("Card layout: {}", v["layout"].to_string());

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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck{
    pub name: String,
    pub commander: Vec<Card>,
    pub library: Vec<Card>,
}
impl Deck {
    pub fn make(input: String)-> CEResult<Deck>{
        use serde_json::Value;

        let mut deck = Deck{
            name: String::from(&input),
            commander: Vec::<Card>::new(),
            library: Vec::<Card>::new(),
        };

        match user_import::decklist(input) {

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

                let database_unwrap: Value = match logic::database::load() {
                    Ok(t) => t,
                    Err(e) => {
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
                        thread_fn::thread_card_make(&tasks_arc_clone1, &tx, &i, &database_arc_clone1);
                    }        
                });
                let handle2 = thread::spawn(move || { 
                    for i in quater_one..quater_two {
                        thread_fn::thread_card_make(&tasks_arc_clone2, &tx1, &i, &database_arc_clone2); 
                    } 
                });
                let handle3 = thread::spawn(move || {
                    for i in quater_two..quater_three {
                        thread_fn::thread_card_make(&tasks_arc_clone3, &tx2, &i, &database_arc_clone3);
                    } 
                });
                
                for i in quater_three..tasks.len() {
                    thread_fn::thread_card_make(&tasks, &tx3, &i, &database);
                } 
                

                drop(tx3);
                
                for card in rx {
                    if card.commander == false {
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
    pub fn load(identifier: &String) -> CEResult<Deck> {
        use serde_json::Value;

        let save = String::from("save/");
        let path = format!("{}{}", save, identifier);
      

        match File::open(path) {
            Ok(t) => {
               
                let deck = serde_json::from_reader(t).expect("Saved deck no proper json");

                println!("Deck successfully opened"); 
               
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
    fn new(name: String, commander: Vec<Card>, library: Vec<Card>) -> Deck {
        Deck{
            name: name,
            commander: commander,
            library: library,
        }
    }
}
/********************************************************************************************************/