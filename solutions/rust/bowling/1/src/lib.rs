#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }

        let (frame_cnt, frame_start) = self.frame_count();
        if frame_cnt < 9 {
            self.validate_regular_frame_roll(frame_start, pins)?;
        } else {
            self.validate_tenth_frame_roll(frame_start, pins)?;
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete() {
            return None;
        }

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
                [a, b, ..] => {
                    score += a + b;
                    roll_idx += 2
                }
                // there should be always 10 valid frames and the matching roll count
                _ => unreachable!("This should not be possible if game validation works."),
            }
        }

        Some(score)
    }

    /// Returns (frame_number, start_idx_of_current_frame)
    /// frame_number is 0-9 (0 = 1st frame, 9 = 10th frame)
    /// start_idx_of_current_frame is the index where the current frame starts
    fn frame_count(&self) -> (usize, usize) {
        let mut roll_idx = 0;
        let mut frame_num = 0;

        // Count complete frames in frames 0-8
        for frame_num_iter in 0..9 {
            if roll_idx >= self.rolls.len() {
                // We don't have any rolls yet for this frame
                return (frame_num_iter, roll_idx);
            }

            if self.rolls[roll_idx] == 10 {
                // Strike: frame complete with 1 roll
                roll_idx += 1;
            } else if roll_idx + 1 < self.rolls.len() {
                // Frame complete with 2 rolls
                roll_idx += 2;
            } else {
                // Incomplete frame: first roll exists but second doesn't
                return (frame_num_iter, roll_idx);
            }

            frame_num = frame_num_iter + 1;
        }

        // We've completed frames 0-8, so the next frame is 9 (the 10th frame)
        (frame_num, roll_idx)
    }

    fn validate_regular_frame_roll(&self, frame_start: usize, pins: u16) -> Result<(), Error> {
        let frame = &self.rolls[frame_start..];
        match frame {
            [] => Ok(()),
            [a] if a + pins <= 10 => Ok(()),
            [_, _] => {
                unreachable!("This shouldn't happen, regular frames can't have more then 2 rolls")
            }
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }

    fn validate_tenth_frame_roll(&self, frame_start: usize, pins: u16) -> Result<(), Error> {
        let tenth_frame_rolls = &self.rolls[frame_start..];
        match tenth_frame_rolls {
            [_, _, _] => unreachable!("This shouldn't happen if game validation works correctly"),
            [10, 10] => Ok(()),
            [10, snd] if snd + pins <= 10 => Ok(()),
            [fst, snd] if fst + snd == 10 => Ok(()),
            [10] => Ok(()),
            [fst] if fst + pins <= 10 => Ok(()),
            [] => Ok(()),
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }

    fn is_game_complete(&self) -> bool {
        let (frame_num, frame_start) = self.frame_count();
        // each game needs 10 frames index 0-9
        if frame_num < 9 {
            return false;
        }

        // Validate 10th frame rolls
        let tenth_frame_rolls = &self.rolls[frame_start..];
        match tenth_frame_rolls {
            [10, _, _] => true,               // Strike: need 3 rolls total
            [a, b, _] if a + b == 10 => true, // Spare: need 3 rolls total
            [a, b] if a + b < 10 => true,     // Normal: need 2 rolls, and they don't sum to 10
            _ => false,
        }
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
