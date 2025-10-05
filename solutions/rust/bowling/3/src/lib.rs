// My idea for this exercise is creating a event-driven state machine
// where each roll is an event that triggers a state transition.
// The state machine will keep track of the current frame, the number of pins left,
// and the rolls made so far. The score will be calculated based on the state of the
// game after each roll.
use std::collections::VecDeque;

const NUMBER_OF_FRAMES: u8 = 10;
const PINS_PER_FRAME: u16 = 10;
const NO_FILL_BALL_THROW: u8 = 2;
const FILL_BALL_THROW: u8 = 3;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Frame(u8);

impl Frame {
    fn is_last(&self) -> bool {
        self.0 == NUMBER_OF_FRAMES
    }

    fn next(&mut self) {
        if self.0 < NUMBER_OF_FRAMES {
            self.0 += 1;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct EventsQueue(VecDeque<Vec<RollType>>);

impl EventsQueue {
    fn new() -> Self {
        Self(VecDeque::from([Vec::new(), Vec::new()]))
    }

    // Add an event to the queue at the given index
    // Index in the queue represents the future events at will be processed at current_frame + index + 1
    // For example, index 0 represents events to be processed at the next frame
    // index 1 represents events to be processed at the frame after next, and so on.
    fn add(&mut self, index: usize, event: RollType) {
        // If the index already exists, push the event to the existing vector
        if let Some(events) = self.0.get_mut(index) {
            events.push(event);
        } else {
            // If the index does not exist, create new vectors until the index is reached
            for i in 0..index {
                if self.0.get(i).is_none() {
                    self.0.push_back(Vec::new());
                }
            }
            self.0.push_back(vec![event]);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq)]
enum RollType {
    Strike(Frame), // Strike at the given frame
    Spare(Frame),  // Spare at the given frame
}

pub struct BowlingGame {
    frame: Frame,                             // Current frame (1-10)
    throw: u8,                                // Current throws in the current frame
    last_throw: u8,                           // The last throw number (2, or 3 in the last frame)
    available_pins: u16,                      // Pins left in the current frame
    rolls: Vec<u16>,                          // all rolls made in the game
    scores: [u16; NUMBER_OF_FRAMES as usize], // scores for each frame
    events_queue: EventsQueue,                // a queue of events to process
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frame: Frame(1),
            throw: 1,
            last_throw: NO_FILL_BALL_THROW,
            available_pins: PINS_PER_FRAME,
            rolls: Vec::new(),
            scores: [0; NUMBER_OF_FRAMES as usize],
            events_queue: EventsQueue::new(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            return Err(Error::GameComplete);
        }
        if self.available_pins < pins {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.rolls.push(pins);
        self.available_pins -= pins;
        // Score updating
        self.add_score(self.frame, pins);
        // Use the current roll to process previous events in the queue like Strikes and Spares
        self.process_event(pins);

        // Three conditions to start a new frame:
        // 1. Strike: all 10 pins knocked down on the first throw of the frame.
        // 2. Spare: all 10 pins knocked down after the second throw of the frame.
        // 3. Open Frame: neither a strike nor a spare, and both throws have been made.
        let is_strike = self.throw == 1 && self.available_pins == 0;
        let is_spare = self.throw == 2 && self.available_pins == 0;
        let is_last_frame = self.frame.is_last();

        // In the last frame, if a strike or spare is scored, the player is awarded an additional throw (fill ball).
        if is_last_frame && self.available_pins == 0 {
            self.last_throw = FILL_BALL_THROW;
        } else if !is_last_frame && is_strike {
            self.events_queue.add(0, RollType::Strike(self.frame));
            self.events_queue.add(1, RollType::Strike(self.frame));
        } else if !is_last_frame && is_spare {
            self.events_queue.add(0, RollType::Spare(self.frame));
        }

        // This logic handles moving to the next frame or throw
        let is_last_throw = self.throw == self.last_throw;
        // Check if last throw or if all pins are knocked down before reached last_throw
        // if we don't have !is_last_frame here, the game will end prematurely in the last frame
        if is_last_throw || (self.available_pins == 0 && !is_last_frame) {
            self.reset_frame();
        } else {
            self.continue_throw();
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.rolls.is_empty() || !self.is_complete() {
            return None;
        }
        self.scores.iter().sum::<u16>().into()
    }

    fn reset_frame(&mut self) {
        // In the last frame, the game is complete
        if self.frame.is_last() {
            self.available_pins = 0;
            self.throw = 0; // No more throws allowed
        } else {
            // Move to the next frame
            self.frame.next();
            self.available_pins = PINS_PER_FRAME;
            self.throw = 1; // Reset to first throw of the next frame
        }
    }

    fn continue_throw(&mut self) {
        self.throw += 1; // Move to the next throw in the current frame
        // Reset pins for fill balls in the last frame
        if self.available_pins == 0 {
            self.available_pins = PINS_PER_FRAME; // Reset pins if all are knocked down
        }
    }

    fn is_complete(&self) -> bool {
        self.frame.is_last() && self.throw == 0 && self.events_queue.0.is_empty()
    }

    fn add_score(&mut self, frame: Frame, pins: u16) {
        let frame_index = (frame.0 - 1) as usize;
        self.scores[frame_index] += pins;
    }

    // Process the events at the front of the queue
    fn process_event(&mut self, pins: u16) {
        if let Some(events) = self.events_queue.0.pop_front() {
            for event in events {
                match event {
                    RollType::Strike(f) | RollType::Spare(f) => {
                        self.add_score(f, pins);
                    }
                }
            }
        }
    }

    #[cfg(feature = "debug")]
    pub fn print_debug(&self) {
        println!("Frame: {:?}", self.frame);
        println!("Throw: {:?}", self.throw);
        println!("Available Pins: {:?}", self.available_pins);
        println!("Rolls: {:?}", self.rolls);
        println!("Scores: {:?}", self.scores);
        println!("Events Queue: {:?}", self.events_queue);
    }
}
