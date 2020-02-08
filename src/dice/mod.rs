use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;

#[derive(Debug)]
pub enum DiceError {
    InvalidDicepool,
    InvalidCount,
    InvalidSides,
    InvalidModifier,
}

pub type Result<T> = std::result::Result<T, DiceError>;

lazy_static! {
    static ref DICEPOOL_DESCRIPTION: Regex =
        Regex::new(r"^(?P<count>\w+)?d(?P<sides>\w+)(?P<mod>[+\-]\w+)?$").unwrap();
}

#[derive(Debug)]
pub struct Dicepool {
    count: u32,
    sides: u32,
    modifier: i32,
}

impl Dicepool {
    pub fn new(count: u32, sides: u32, modifier: i32) -> Self {
        Self {
            count,
            sides,
            modifier,
        }
    }

    pub fn from_description(description: &str) -> Result<Self> {
        let cap = DICEPOOL_DESCRIPTION
            .captures(description)
            .ok_or(DiceError::InvalidDicepool)?;

        let count = match cap.name("count") {
            Some(m) => m
                .as_str()
                .parse::<u32>()
                .map_err(|_| DiceError::InvalidCount)?,
            None => 1,
        };

        let sides = cap["sides"]
            .parse::<u32>()
            .map_err(|_| DiceError::InvalidSides)?;

        let modifier = match cap.name("mod") {
            Some(m) => m
                .as_str()
                .parse::<i32>()
                .map_err(|_| DiceError::InvalidModifier)?,
            None => 0,
        };
        Ok(Dicepool::new(count, sides, modifier))
    }

    pub fn dice(&self) -> Vec<i64> {
        let mut dice = vec![];
        for _ in 0..self.count {
            let die = 1 + thread_rng().gen::<u32>() % self.sides;
            dice.push(i64::from(die));
        }
        dice
    }

    pub fn roll(&self) -> i64 {
        let dice_total: i64 = self.dice().into_iter().sum();
        dice_total + i64::from(self.modifier)
    }
}
