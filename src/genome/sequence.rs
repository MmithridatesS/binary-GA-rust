pub struct BinarySequence<T> {
    sequence: T,
    len: usize,
}
pub enum BinarySequenceError {
    ChosenTypeUnnecessarilyBig,
}

impl<T> BinarySequence<T> {
    pub fn new(sequence: T, len: usize) -> Result<Self, BinarySequenceError> {
        if 1 << std::mem::size_of::<T>()*8 > 2*len {
            return Err(BinarySequenceError::ChosenTypeUnnecessarilyBig);
        };
        Ok(Self {
            sequence,
            len
        })
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn as_mut(&mut self) -> &mut T {
        &mut self.sequence
    }
}
