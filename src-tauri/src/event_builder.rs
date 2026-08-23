#[derive(Debug, Clone)]
pub struct TransientEvent {
    pub duration_ms: f32,
    pub peak_rms: f32,
    pub avg_centroid_hz: f32,
}

#[derive(PartialEq)]
enum BuilderState {
    Idle,
    Recording,
    GracePeriod,
}

pub struct EventBuilder {
    threshold: f32,
    grace_period_ms: f32,
    frame_duration_ms: f32,

    state: BuilderState,

    // Ongoing event features
    accumulated_rms: Vec<f32>,
    accumulated_centroid: Vec<f32>,

    // State machine timers
    grace_timer_ms: f32,
}

impl EventBuilder {
    pub fn new(threshold: f32, grace_period_ms: f32, frame_duration_ms: f32) -> Self {
        Self {
            threshold,
            grace_period_ms,
            frame_duration_ms,
            state: BuilderState::Idle,
            accumulated_rms: Vec::new(),
            accumulated_centroid: Vec::new(),
            grace_timer_ms: 0.0,
        }
    }

    pub fn process_frame(&mut self, rms: f32, centroid: f32) -> Option<TransientEvent> {
        match self.state {
            BuilderState::Idle => {
                if rms > self.threshold {
                    self.state = BuilderState::Recording;
                    self.accumulated_rms.clear();
                    self.accumulated_centroid.clear();
                    self.accumulated_rms.push(rms);
                    self.accumulated_centroid.push(centroid);
                }
                None
            }
            BuilderState::Recording => {
                self.accumulated_rms.push(rms);
                self.accumulated_centroid.push(centroid);

                if rms <= self.threshold {
                    self.state = BuilderState::GracePeriod;
                    self.grace_timer_ms = 0.0;
                }
                None
            }
            BuilderState::GracePeriod => {
                self.accumulated_rms.push(rms);
                self.accumulated_centroid.push(centroid);
                self.grace_timer_ms += self.frame_duration_ms;

                if rms > self.threshold {
                    // False alarm, back to recording
                    self.state = BuilderState::Recording;
                    None
                } else if self.grace_timer_ms >= self.grace_period_ms {
                    // Grace period expired, finalize event
                    self.state = BuilderState::Idle;
                    Some(self.finalize_event())
                } else {
                    None
                }
            }
        }
    }

    fn finalize_event(&self) -> TransientEvent {
        let duration_ms = self.accumulated_rms.len() as f32 * self.frame_duration_ms;

        // Find peak RMS
        let peak_rms = self
            .accumulated_rms
            .iter()
            .cloned()
            .fold(0.0 / 0.0, f32::max);

        // Average centroid across all frames
        let avg_centroid_hz = if !self.accumulated_centroid.is_empty() {
            self.accumulated_centroid.iter().sum::<f32>() / self.accumulated_centroid.len() as f32
        } else {
            0.0
        };

        TransientEvent {
            duration_ms,
            peak_rms,
            avg_centroid_hz,
        }
    }
}
