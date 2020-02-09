use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;

#[derive(Debug)]
pub enum DiceError {
    ParseError(DicepoolParseError),
}

#[derive(Debug)]
pub enum DicepoolParseError {
    InvalidFormat,
    InvalidCount,
    InvalidSides,
    InvalidModifier,
}

pub type Result<T> = std::result::Result<T, DiceError>;
pub type DieRoll = u32;
pub type RollModifier = i32;
pub type DicepoolRoll = i64;

lazy_static! {
    static ref DICEPOOL_DESCRIPTION: Regex =
        Regex::new(r"^(?P<count>\w+?)?d(?P<sides>\w+)(?P<mod>[+\-]\w+)?$").unwrap();
}

#[derive(Debug)]
pub struct Dicepool {
    count: u32,
    sides: DieRoll,
    modifier: RollModifier,
}

impl Dicepool {
    pub fn new(count: u32, sides: DieRoll, modifier: RollModifier) -> Self {
        Self {
            count,
            sides,
            modifier,
        }
    }

    pub fn from_description(description: &str) -> Result<Self> {
        let cap = DICEPOOL_DESCRIPTION
            .captures(description)
            .ok_or(DiceError::ParseError(DicepoolParseError::InvalidFormat))?;

        let count = match cap.name("count") {
            Some(m) => m
                .as_str()
                .parse::<u32>()
                .map_err(|_| DiceError::ParseError(DicepoolParseError::InvalidCount))?,
            None => 1,
        };

        let sides = cap["sides"]
            .parse::<DieRoll>()
            .map_err(|_| DiceError::ParseError(DicepoolParseError::InvalidSides))?;

        let modifier = match cap.name("mod") {
            Some(m) => m
                .as_str()
                .parse::<RollModifier>()
                .map_err(|_| DiceError::ParseError(DicepoolParseError::InvalidModifier))?,
            None => 0,
        };
        Ok(Dicepool::new(count, sides, modifier))
    }

    pub fn roll(&self) -> DicepoolRoll {
        let mut result = DicepoolRoll::from(self.modifier);
        for _ in 0..self.count {
            let die = 1 + thread_rng().gen::<DieRoll>() % self.sides;
            result += DicepoolRoll::from(die);
        }
        result
    }
}
