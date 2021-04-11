use std::sync::atomic::{AtomicU16, AtomicU64, Ordering};

/// Sequencer generates sequential sequence numbers for building RTP packets
pub trait Sequencer {
    fn next_sequence_number(&self) -> u16;
    fn roll_over_count(&self) -> u64;
}

/// NewRandomSequencer returns a new sequencer starting from a random sequence
/// number
pub fn new_random_sequencer() -> impl Sequencer {
    SequencerImpl {
        sequence_number: AtomicU16::new(rand::random::<u16>()),
        roll_over_count: AtomicU64::new(0),
    }
}

/// NewFixedSequencer returns a new sequencer starting from a specific
/// sequence number
pub fn new_fixed_sequencer(s: u16) -> impl Sequencer {
    SequencerImpl {
        sequence_number: if s == 0 {
            AtomicU16::new(0xFFFF)
        } else {
            AtomicU16::new(s - 1)
        },
        roll_over_count: AtomicU64::new(0),
    }
}

struct SequencerImpl {
    sequence_number: AtomicU16,
    roll_over_count: AtomicU64,
}

impl Sequencer for SequencerImpl {
    /// NextSequenceNumber increment and returns a new sequence number for
    /// building RTP packets
    fn next_sequence_number(&self) -> u16 {
        let sequence_number = self.sequence_number.load(Ordering::SeqCst);
        if sequence_number == std::u16::MAX {
            self.roll_over_count.fetch_add(1, Ordering::SeqCst);
            self.sequence_number.store(0, Ordering::SeqCst);
            0
        } else {
            self.sequence_number
                .store(sequence_number + 1, Ordering::SeqCst);
            sequence_number + 1
        }
    }

    /// RollOverCount returns the amount of times the 16bit sequence number
    /// has wrapped
    fn roll_over_count(&self) -> u64 {
        self.roll_over_count.load(Ordering::SeqCst)
    }
}
