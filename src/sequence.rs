use std::fmt;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

/// Sequencer generates sequential sequence numbers for building RTP packets
pub trait Sequencer: fmt::Debug {
    fn next_sequence_number(&self) -> u16;
    fn roll_over_count(&self) -> u64;
}

/// NewRandomSequencer returns a new sequencer starting from a random sequence
/// number
pub fn new_random_sequencer() -> impl Sequencer {
    new_fixed_sequencer(rand::random::<u16>())
}

/// NewFixedSequencer returns a new sequencer starting from a specific
/// sequence number
pub fn new_fixed_sequencer(s: u16) -> impl Sequencer {
    SequencerImpl {
        count: Arc::new(AtomicU64::new(u64::from(s))),
    }
}

#[derive(Debug, Clone)]
struct SequencerImpl {
    // The most significant 48 bits store the number of roll overs, and the lower 16 bits store the
    // next sequence number.
    // If we gave out one sequence number per nanosecond, then we'd need to give out sequence
    // numbers for almost 600 years before we run out of roll overs.
    count: Arc<AtomicU64>,
}

impl Sequencer for SequencerImpl {
    /// NextSequenceNumber increment and returns a new sequence number for
    /// building RTP packets
    fn next_sequence_number(&self) -> u16 {
        (self.count.fetch_add(1, Ordering::Release) & 0xFFFF) as u16
    }

    /// RollOverCount returns the amount of times the 16bit sequence number
    /// has wrapped
    fn roll_over_count(&self) -> u64 {
        self.count.load(Ordering::Acquire).overflowing_shr(16).0
    }
}
