use crate::event_builder::TransientEvent;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GestureType {
    SingleKnock,
    Clap,
    Slam,
    Snap,
    // Note: Double/Triple Knocks are resolved by the PatternLayer, not the Classifier
}

impl std::fmt::Display for GestureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GestureType::SingleKnock => write!(f, "Single Knock"),
            GestureType::Clap => write!(f, "Clap"),
            GestureType::Slam => write!(f, "Slam"),
            GestureType::Snap => write!(f, "Snap"),
        }
    }
}

pub struct GestureClassifier;

impl GestureClassifier {
    pub fn new() -> Self {
        Self {}
    }

    /// Classify a single discrete acoustic event.
    ///
    /// Note: The thresholds below are "sensible defaults" meant only for scaffolding
    /// the Phase 4 engine. Real-world testing shows that these values (especially Centroid)
    /// vary wildly by device/microphone hardware.
    /// In Phase 5, these will be replaced by user-configured templates via the Calibration UI.
    pub fn classify(&self, event: &TransientEvent) -> Option<GestureType> {
        let dur = event.duration_ms;
        let cent = event.avg_centroid_hz;

        // Snap: Very short duration, extremely high frequency
        if (10.0..=45.0).contains(&dur) && cent > 4000.0 {
            return Some(GestureType::Snap);
        }

        // Clap: Short duration, high frequency
        if (20.0..=90.0).contains(&dur) && (2500.0..=4000.0).contains(&cent) {
            return Some(GestureType::Clap);
        }

        // Single Knock: Medium duration, low/mid frequency
        if (40.0..=150.0).contains(&dur) && (500.0..=2500.0).contains(&cent) {
            return Some(GestureType::SingleKnock);
        }

        // Slam: Long duration or very low frequency, high amplitude thud
        if dur >= 80.0 && cent <= 500.0 {
            return Some(GestureType::Slam);
        }
        None // Doesn't cleanly match a known gesture fingerprint
    }
}

impl Default for GestureClassifier {
    fn default() -> Self {
        Self::new()
    }
}
