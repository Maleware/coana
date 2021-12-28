pub enum CardType{
    Instant(Vec::new(SpellSubtype)),
    Sorcery(Vec::new(SpellSubtype)),
    Artifact(Vec::new(ArtifactSubtype)),
    Creature(Vec::new(CreatureSubtype)),
    Enchantment(Vec::new(EnchantmentSubtype)),
    Land(Vec::new(LandSubtype)),
    Planeswalker(Vec::new(PlaneswalkerSubtype)),
    InvalidCardType,
}

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
    None,
}
pub enum SpellSubtype{
    Adventure, 
    Arcane, 
    Lesson, 
    Trap,
    None,
}
pub enum CreatureSubtype{}
pub enum EnchantmentSubtype{
    Aura, 
    Cartouche, 
    Class, 
    Curse, 
    Rune, 
    Saga, 
    Shrine, 
    Shard,
    None,
}
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
    None,
}

pub enum Stats{
    Power(u8),
    Toughness(u8),
    Loyality(u8),
}