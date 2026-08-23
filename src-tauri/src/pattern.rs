use crate::classifier::GestureType;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FinalGesture {
    SingleKnock,
    DoubleKnock,
    TripleKnock,
    Clap,
    Slam,
    Snap,
}

pub struct PatternTracker {
    last_knock_time: Option<Instant>,
    knock_count: u32,
    window_ms: u64,
}

impl PatternTracker {
    pub fn new() -> Self {
        Self {
            last_knock_time: None,
            knock_count: 0,
            window_ms: 600, // wait up to 600ms for consecutive knocks
        }
    }

    /// Process a new gesture from the classifier.
    /// Returns a FinalGesture immediately if it doesn't need to wait for a pattern,
    /// or None if it's buffering it (like a knock).
    pub fn process_gesture(&mut self, gesture: GestureType) -> Option<FinalGesture> {
        match gesture {
            GestureType::SingleKnock => {
                let now = Instant::now();
                if let Some(last) = self.last_knock_time {
                    if now.duration_since(last).as_millis() as u64 <= self.window_ms {
                        self.knock_count += 1;
                        self.last_knock_time = Some(now);

                        if self.knock_count == 3 {
                            // Max pattern reached, emit immediately
                            self.knock_count = 0;
                            self.last_knock_time = None;
                            return Some(FinalGesture::TripleKnock);
                        }
                        return None;
                    }
                }

                // First knock or previous window expired
                self.knock_count = 1;
                self.last_knock_time = Some(now);
                None
            }
            GestureType::Clap => {
                self.reset();
                Some(FinalGesture::Clap)
            }
            GestureType::Slam => {
                self.reset();
                Some(FinalGesture::Slam)
            }
            GestureType::Snap => {
                self.reset();
                Some(FinalGesture::Snap)
            }
        }
    }

    /// Tick should be called periodically (e.g., every frame) to flush
    /// buffered patterns if the window has expired.
    pub fn tick(&mut self) -> Option<FinalGesture> {
        if let Some(last) = self.last_knock_time {
            if last.elapsed().as_millis() as u64 > self.window_ms {
                let result = match self.knock_count {
                    1 => Some(FinalGesture::SingleKnock),
                    2 => Some(FinalGesture::DoubleKnock),
                    _ => None, // Should not happen, 3 emits immediately
                };
                self.reset();
                return result;
            }
        }
        None
    }

    fn reset(&mut self) {
        self.knock_count = 0;
        self.last_knock_time = None;
    }
}
