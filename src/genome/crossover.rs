
use crate::genome::sequence::BinarySequence;

#[derive(PartialEq)]
enum CrossoverError {
    InvalidSequenceLength,
}

trait Crossover<T> {
    fn crossover(&self, first: &mut BinarySequence<T>, second: &mut BinarySequence<T>) -> Result<(), CrossoverError>;
}

struct SinglePointCrossover {
    sequence_length: usize
}

impl Crossover<u16> for SinglePointCrossover {
    fn crossover(&self, first: &mut BinarySequence<u16>, second: &mut BinarySequence<u16>) -> Result<(), CrossoverError> {
        if self.sequence_length < 2 {
            return Err(CrossoverError::InvalidSequenceLength);
        }

        Ok(())
    }
}


macro_rules! impl_single_point_crossover_for_types {
    ($($type: ty), +) => {
        $(
        )+
    }
}
