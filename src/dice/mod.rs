use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;

#[derive(Debug)]
pub enum DiceError {
    /// Could not parse expression
    ParseError,
}

pub type Result<T> = std::result::Result<T, DiceError>;

/// Type of a single die rolled value
pub type DieFace = u32;

/// Type of a modifier
pub type RollModifier = i32;

/// Type of a dicepool rolled total
pub type DicepoolRoll = i64;

lazy_static! {
    static ref DICEPOOL_DESCRIPTION: Regex =
        Regex::new(r"^(?P<count>\d+?)?d(?P<sides>\d+)(?P<mod>[+\-]\d+)?$").unwrap();
}

/// A dicepool description that can be rolled
#[derive(Debug)]
pub struct Dicepool {
    count: u32,
    sides: DieFace,
    modifier: RollModifier,
}

impl Dicepool {
    /// Create a pool from the given count, die type, and modifier
    pub fn new(count: u32, sides: DieFace, modifier: RollModifier) -> Self {
        Self {
            count,
            sides,
            modifier,
        }
    }

    /// Parse a string in the form "XdY+Z"
    pub fn from_description(description: &str) -> Result<Self> {
        let cap = DICEPOOL_DESCRIPTION
            .captures(description)
            .ok_or(DiceError::ParseError)?;

        let count = match cap.name("count") {
            Some(m) => m.as_str().parse::<u32>().unwrap(), // Guaranteed to be a number, as it matched (\d+)
            None => 1,
        };

        let sides = cap["sides"].parse::<DieFace>().unwrap(); // Guaranteed to be a number

        let modifier = match cap.name("mod") {
            Some(m) => m.as_str().parse::<RollModifier>().unwrap(), // Guaranteed to be a number
            None => 0,
        };
        Ok(Dicepool::new(count, sides, modifier))
    }

    /// Roll the dice in the pool and return the total
    pub fn roll(&self) -> DicepoolRoll {
        let mut result = DicepoolRoll::from(self.modifier);
        for _ in 0..self.count {
            let die = 1 + thread_rng().gen::<DieFace>() % self.sides;
            result += DicepoolRoll::from(die);
        }
        result
    }
}
