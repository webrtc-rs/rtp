/// Sequencer generates sequential sequence numbers for building RTP packets
pub trait Sequencer {
    fn next_sequence_number(&mut self) -> u16;
}

#[derive(Debug, Clone)]
pub struct WrappingSequencer {
    next_sequence_number: u16,
    roll_over_count: u64,
}

impl WrappingSequencer {
    /// Returns a new sequencer starting from a specific sequence number.
    pub fn new(init_sequence_number: u16) -> Self {
        Self {
            next_sequence_number: init_sequence_number,
            roll_over_count: 0,
        }
    }

    /// Returns a new sequencer starting from a random sequence number.
    pub fn new_random() -> Self {
        Self::new(rand::random::<u16>())
    }

    /// RollOverCount returns the amount of times the 16bit sequence number
    /// has wrapped
    pub fn roll_over_count(&self) -> u64 {
        self.roll_over_count
    }
}

impl Sequencer for WrappingSequencer {
    /// NextSequenceNumber increment and returns a new sequence number for
    /// building RTP packets
    fn next_sequence_number(&mut self) -> u16 {
        let next_sequence_number = self.next_sequence_number;
        self.next_sequence_number = self.next_sequence_number.wrapping_add(1);

        if self.next_sequence_number == 0 {
            self.roll_over_count += 1;
        }

        next_sequence_number
    }
}
