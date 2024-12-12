mod mutation {
    use std::usize;
    use crate::genome::sequence::BinarySequence;
    use rand::Rng;

    pub fn toss(prob: f32) -> bool {
        let mut rng = rand::thread_rng();
        let gn: f32 = rng.gen();
        if gn < prob {
            return true;
        }
        false
    }

    #[derive(Debug, PartialEq)]
    pub enum MutationError {
        UnmatchedTypeAndSequenceLength,
    }
    type MutationResult<T> = Result<T, MutationError>;

    pub trait Mutator<Item>{
        type Item;
        fn mutate(&self, sequence: &mut Self::Item) -> MutationResult<()>;
    }

    pub struct BitFlipMutator {
        probability: f32,
        sequence_length: usize
    }

    impl BitFlipMutator {
        pub fn new(probability: f32, sequence_length: usize) -> Self {
            assert!((probability < 1.) && (probability > 0.));
            Self {
                probability,
                sequence_length
            }
        }
        pub fn sequence_length(&self) -> usize {
            self.sequence_length
        }
    }

    // This Macro should only be used for unsigned integers //
    macro_rules! impl_mut_for_bitflip_mutator {
        ($($t: ty),*) => {
            $(impl Mutator<$t> for BitFlipMutator {
                type Item = BinarySequence<$t>;
                fn mutate(&self, sequence: &mut Self::Item) -> MutationResult<()> {
                    let sequence_size: usize = std::mem::size_of::<$t>()*8;
                    if (sequence_size < sequence.len()) {
                        return Err(MutationError::UnmatchedTypeAndSequenceLength);
                    }
                    for pos in 0..self.sequence_length {
                        if toss(self.probability) {
                            *sequence.as_mut() ^= (1<<pos);
                        }
                    }
                    Ok(())
                }
            })*
        };
    }

    impl_mut_for_bitflip_mutator!(u8, u16, u32, u64, u128, usize);

    pub struct SinglePointMutator;

    macro_rules! impl_mut_for_singlepoint_mutator {
        ($($type: ty),*) => {
            $(impl Mutator<$u> for SinglePointMutator {
                type Item = Sequence<$type>;
                fn mutate(&self, sequence: &mut $u) -> MutationResult<()> {
                    let sequence_size: usize = std::mem::size_of::<$u>()*8;
                    if (sequence_size < self.sequence_length) {
                        return Err(MutationError::UnmatchedTypeAndSequenceLength);
                    }
                    let mut rng = rand::thread_rng();
                    *sequence ^= (1<<rng.gen_range(0..self.sequence_length));
                    Ok(())
                }
            })*
        };
    }

    impl_mut_for_singlepoint_mutator!(u8, u16, u32, u64, usize);

    pub struct MultiPointMutator {
        sequence_length: usize,
        num_mutations: usize
    }

    impl MultiPointMutator {
        pub fn new(sequence_length: usize, num_mutations: usize) -> Self {
            assert!(num_mutations < sequence_length);
            Self {
                sequence_length,
                num_mutations,
            }
        }
        pub fn sequence_length(&self) -> usize {
            self.sequence_length
        }
    }

    macro_rules! impl_mut_for_multipoint_mutator {
        ($($t:ty),*) => {
            $(impl Mutator<$t> for MultiPointMutator {
                fn mutate(&self, sequence: &mut $t) -> MutationResult<()> {
                    let sequence_size = std::mem::size_of::<$t>()*8;
                    if self.sequence_length  > sequence_size {
                        return Err(MutationError::UnmatchedTypeAndSequenceLength)
                    }
                    let mut mutator: $t = 0;
                    let mut range: Vec<usize> = (0..self.sequence_length-1).collect();
                    for _ in 0..self.num_mutations{
                        let mut rng = rand::thread_rng();
                        let idx = rng.gen_range(0..range.len());
                        mutator += 1 << range[idx];
                        range.remove(idx);
                    }
                    *sequence ^= mutator;
                    Ok(())
                }
            })*
        };
    }
    impl_mut_for_multipoint_mutator!(u8, u16, u32, u64, u128, usize);
}

#[cfg(test)] 
mod tests {
    use super::mutation::*;

    #[test]
    fn mutated_should_always_be_less_than_sequence_length() {
        let mutator = BitFlipMutator::new(0.50, 7);
        let mut sequence_2: u16 = 0u16;
        let _ = mutator.mutate(&mut sequence_2);
        assert!(sequence_2 < 1 << mutator.sequence_length());
    }

    #[test]
    fn test_bitflip_mutator() {
        let mutator = BitFlipMutator::new(0.5, 7);
        let sequence_1: u16 = 0u16;
        let mut sequence_2: u16 = 0u16;
        let _ = mutator.mutate(&mut sequence_2);
        assert_ne!(sequence_2, sequence_1);
    }

    #[test]
    fn test_singlepoint_mutator() {
        let mutator = SinglePointMutator::new(0.5, 7);
        let sequence_1: u16 = 0u16;
        let mut sequence_2: u16 = sequence_1.clone();
        let _ = mutator.mutate(&mut sequence_2);
        assert_eq!(sequence_2 & (sequence_2 - 1), 0);
    }

    #[test]
    fn visualize_singlepoint_mutator() {
        let mutator = SinglePointMutator::new(0.5, 7);
        let sequence_1: u16 = 0u16;
        let mut sequence_2: u16 = sequence_1.clone();
        let _ = mutator.mutate(&mut sequence_2);
        assert_eq!(sequence_2 & (sequence_2 - 1), 0u16);
    }

    #[test]
    fn test_multipoint_mutator() {
        let mutator = MultiPointMutator::new(7, 0);
        let sequence_1: u16 = 0u16;
        let mut sequence_2: u16 = sequence_1.clone();
        let _ = mutator.mutate(&mut sequence_2);
        assert_eq!(sequence_2, sequence_1);
    }

    #[test]
    fn test_multipoint_mutator_uneq() {
        let mutator = MultiPointMutator::new(7, 1);
        let sequence_1: u16 = 0u16;
        let mut sequence_2: u16 = sequence_1.clone();
        let _ = mutator.mutate(&mut sequence_2);
        assert_ne!(sequence_2, sequence_1);
    }

    #[test]
    fn test_multipoint_is_singlepoint_for_1_mutation() {
        let mutator = MultiPointMutator::new(7, 1);
        let mut sequence_2: u16 = 0;
        let _ = mutator.mutate(&mut sequence_2);
        assert_eq!(sequence_2 & (sequence_2 - 1), 0);
    }

    #[test]
    fn test_multipoint_is_singlepoint_for_2_mutation() {
        let mutator = MultiPointMutator::new(7, 2);
        let mut sequence_2: u16 = 0;
        let _ = mutator.mutate(&mut sequence_2);
        assert_eq!(sequence_2.count_ones(), 2);
    }

    #[test]
    fn test_multipoint_is_singlepoint_for_3_mutation() {
        let mutator = MultiPointMutator::new(7, 3);
        let mut sequence_2: u16 = 0;
        let _ = mutator.mutate(&mut sequence_2);
        assert_eq!(sequence_2.count_ones(), 3);
    }

    #[test]
    fn test_multipoint_is_singlepoint_for_1_mutation_random() {
        let mutator = MultiPointMutator::new(7, 1);
        let sequence_1: u16 = 17u16;
        let mut sequence_2: u16 = sequence_1.clone();
        let _ = mutator.mutate(&mut sequence_2);
        assert_eq!((sequence_2 ^ sequence_1).count_ones(), 1);
    }

    #[test]
    fn test_multipoint_length_more_than_type_size() {
        let mutator = MultiPointMutator::new(9, 1);
        let mut sequence_1 = 255u8;
        let result = mutator.mutate(&mut sequence_1);
        assert_eq!(result, Err(MutationError::UnmatchedTypeAndSequenceLength));
    }

    #[test]
    fn test_mutlipoint_sequence_length_more_than_type_size_2() {
        let mutator = MultiPointMutator::new(8, 3);
        let mut sequence_1 = 255u8;
        let sequence_2 = sequence_1;
        let _ = mutator.mutate(&mut sequence_1);
        assert_eq!((sequence_2 ^ sequence_1).count_ones(), 3);
    }
}






