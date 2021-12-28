mod types;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub cmc: f64,
    pub mana_cost: String,
    pub name: String,
    pub cardtype: Vec::new(CardType),
    pub legendary: bool,
    pub stats: Vec::new(Stats),
    pub commander: bool,
    pub backside: Box<Option(Card)>
}
