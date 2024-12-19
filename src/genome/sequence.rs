#[derive(Debug, PartialEq, Clone)]
pub struct BinarySequence<T> {
    sequence: T,
    len: usize,
}
#[derive(Debug)]
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
    pub fn sequence(&self) -> &T {
        &self.sequence
    }
    pub fn as_mut(&mut self) -> &mut T {
        &mut self.sequence
    }
}
impl<T> AsMut<T> for BinarySequence<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.sequence
    }
}

#[macro_export]
macro_rules! binary_sequence{
    ($len: literal) => {
        if $len <= 8 {return BinarySequence::new(0u8, $len).unwrap()}
        else if $len > 8 && $len <= 16 {BinarySequence::new(0u16, $len).unwrap()}
        else if $len > 16 && $len <= 32 {BinarySequence::new(0u32, $len).unwrap()}
        else if $len > 32 && $len <= 64 {BinarySequence::new(0u64, $len).unwrap()}
        else if $len > 64 && $len <= 128 {BinarySequence::new(0u128, $len).unwrap()}
        else {panic!()}
    };
}
