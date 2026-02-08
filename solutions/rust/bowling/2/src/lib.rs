#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    snd: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: Vec::new(),
            snd: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || self.snd && pins + self.rolls.last().unwrap() > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.score().is_some() {
            return Err(Error::GameComplete);
        }

        self.rolls.push(pins);
        self.snd = if pins != 10 { !self.snd } else { false };
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut score: u16 = 0;
        let mut roll_idx = 0;
        for _ in 0..10 {
            let end = (roll_idx + 3).min(self.rolls.len());
            let rolls = &self.rolls[roll_idx..end];
            match rolls {
                // frame is a Strike (next two rolls get added)
                [10, a, b] => {
                    score += 10 + a + b;
                    roll_idx += 1;
                }
                // frame is a spare (next roll gets added)
                [a, b, c] if a + b == 10 => {
                    score += 10 + c;
                    roll_idx += 2
                }
                // regular frame
                [a, b, ..] if a + b < 10 => {
                    score += a + b;
                    roll_idx += 2
                }
                // If the game is complete one of the above patterns much match for every frame
                _ => return None,
            }
        }

        Some(score)
    }
}
