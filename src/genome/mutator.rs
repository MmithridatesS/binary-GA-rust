mod mutation {
    use std::marker::PhantomData;
    use std::usize;
    use crate::genome::sequence::BinarySequence;
    use crate::binary_sequence;
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


    pub trait Mutator<Target> {
        fn mutate(&self, sequence: &mut BinarySequence<Target>) -> MutationResult<()>;
    }

    pub struct BitFlipMutator{
        probability: f32,
    }

    impl BitFlipMutator {
        pub fn new(probability: f32) -> Self {
            assert!((probability < 1.) && (probability > 0.));
            Self{
                probability,

            }
        }
    }

    // This Macro should only be used for unsigned integers //
    macro_rules! impl_mut_for_bitflip_mutator {
        ($($t: ty),*) => {
            $(impl Mutator<$t> for BitFlipMutator {
                fn mutate(&self, sequence: &mut BinarySequence<$t>) -> MutationResult<()> {
                    let sequence_size: usize = std::mem::size_of::<$t>()*8;
                    if (sequence_size < sequence.len()) {
                        return Err(MutationError::UnmatchedTypeAndSequenceLength);
                    }
                    for pos in 0..(sequence.len()) {
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
            $(impl Mutator<$type> for SinglePointMutator {
                fn mutate(&self, sequence: &mut BinarySequence<$type>) -> MutationResult<()> {
                    let sequence_size: usize = std::mem::size_of::<$type>()*8;
                    if (sequence_size < sequence.len()) {
                        return Err(MutationError::UnmatchedTypeAndSequenceLength);
                    }
                    let mut rng = rand::thread_rng();
                    *(sequence.as_mut()) ^= (1<<rng.gen_range(0..sequence.len()));
                    Ok(())
                }
            })*
        };
    }

    impl_mut_for_singlepoint_mutator!(u8, u16, u32, u64, usize);

    pub struct MultiPointMutator{
        num_mutations: usize
    }

    impl MultiPointMutator {
        pub fn new(num_mutations: usize) -> Self {
            Self {
                num_mutations
            }
        }
        pub fn num_mutations(&self) -> usize {
            self.num_mutations
        }
    }

    macro_rules! impl_mut_for_multipoint_mutator {
        ($($t:ty),*) => {
            $(impl Mutator<$t> for MultiPointMutator {
                fn mutate(&self, sequence: &mut BinarySequence<$t>) -> MutationResult<()> {
                    let mut mutator: $t = 0;
                    let mut range: Vec<usize> = (0..sequence.len()-1).collect();
                    for _ in 0..self.num_mutations(){
                        let mut rng = rand::thread_rng();
                        let idx = rng.gen_range(0..range.len());
                        mutator += 1 << range[idx];
                        range.remove(idx);
                    }
                    *(sequence.as_mut()) ^= mutator;
                    Ok(())
                }
            })*
        };
    }
    impl_mut_for_multipoint_mutator!(u8, u16, u32, u64, u128, usize);
}

#[cfg(test)] 
mod tests {
    use crate::genome::sequence::BinarySequence;
    use crate::binary_sequence;

    use super::mutation::*;

    #[test]
    fn mutated_should_always_be_less_than_sequence_length() {
        let mutator = BitFlipMutator::new(0.50);
        let mut sequence: BinarySequence<u16> = BinarySequence::new(40, 8).unwrap();
        let _ = mutator.mutate(&mut sequence);
        assert!(*sequence.sequence() <= 1 << sequence.len());
    }

    #[test]
    fn test_bitflip_mutator() {
        let mutator = BitFlipMutator::new(0.5);

        let sequence_1: BinarySequence<u16> = BinarySequence::new(0, 256).unwrap();
        let mut sequence_2 = sequence_1.clone();
        let _ = mutator.mutate(&mut sequence_2);
        assert_ne!(sequence_2, sequence_1);
    }

    // #[test]
    // fn test_singlepoint_mutator() {
    //     let mutator = SinglePointMutator::new(0.5, 7);
    //     let sequence_1: u16 = 0u16;
    //     let mut sequence_2: u16 = sequence_1.clone();
    //     let _ = mutator.mutate(&mut sequence_2);
    //     assert_eq!(sequence_2 & (sequence_2 - 1), 0);
    // }
    //
    // #[test]
    // fn visualize_singlepoint_mutator() {
    //     let mutator = SinglePointMutator::new(0.5, 7);
    //     let sequence_1: u16 = 0u16;
    //     let mut sequence_2: u16 = sequence_1.clone();
    //     let _ = mutator.mutate(&mut sequence_2);
    //     assert_eq!(sequence_2 & (sequence_2 - 1), 0u16);
    // }
    //
    // #[test]
    // fn test_multipoint_mutator() {
    //     let mutator = MultiPointMutator::new(7, 0);
    //     let sequence_1: u16 = 0u16;
    //     let mut sequence_2: u16 = sequence_1.clone();
    //     let _ = mutator.mutate(&mut sequence_2);
    //     assert_eq!(sequence_2, sequence_1);
    // }
    //
    // #[test]
    // fn test_multipoint_mutator_uneq() {
    //     let mutator = MultiPointMutator::new(7, 1);
    //     let sequence_1: u16 = 0u16;
    //     let mut sequence_2: u16 = sequence_1.clone();
    //     let _ = mutator.mutate(&mut sequence_2);
    //     assert_ne!(sequence_2, sequence_1);
    // }
    //
    // #[test]
    // fn test_multipoint_is_singlepoint_for_1_mutation() {
    //     let mutator = MultiPointMutator::new(7, 1);
    //     let mut sequence_2: u16 = 0;
    //     let _ = mutator.mutate(&mut sequence_2);
    //     assert_eq!(sequence_2 & (sequence_2 - 1), 0);
    // }
    //
    // #[test]
    // fn test_multipoint_is_singlepoint_for_2_mutation() {
    //     let mutator = MultiPointMutator::new(7, 2);
    //     let mut sequence_2: u16 = 0;
    //     let _ = mutator.mutate(&mut sequence_2);
    //     assert_eq!(sequence_2.count_ones(), 2);
    // }
    //
    // #[test]
    // fn test_multipoint_is_singlepoint_for_3_mutation() {
    //     let mutator = MultiPointMutator::new(7, 3);
    //     let mut sequence_2: u16 = 0;
    //     let _ = mutator.mutate(&mut sequence_2);
    //     assert_eq!(sequence_2.count_ones(), 3);
    // }
    //
    // #[test]
    // fn test_multipoint_is_singlepoint_for_1_mutation_random() {
    //     let mutator = MultiPointMutator::new(7, 1);
    //     let sequence_1: u16 = 17u16;
    //     let mut sequence_2: u16 = sequence_1.clone();
    //     let _ = mutator.mutate(&mut sequence_2);
    //     assert_eq!((sequence_2 ^ sequence_1).count_ones(), 1);
    // }
    //
    // #[test]
    // fn test_multipoint_length_more_than_type_size() {
    //     let mutator = MultiPointMutator::new(9, 1);
    //     let mut sequence_1 = 255u8;
    //     let result = mutator.mutate(&mut sequence_1);
    //     assert_eq!(result, Err(MutationError::UnmatchedTypeAndSequenceLength));
    // }
    //
    // #[test]
    // fn test_mutlipoint_sequence_length_more_than_type_size_2() {
    //     let mutator = MultiPointMutator::new(8, 3);
    //     let mut sequence_1 = 255u8;
    //     let sequence_2 = sequence_1;
    //     let _ = mutator.mutate(&mut sequence_1);
    //     assert_eq!((sequence_2 ^ sequence_1).count_ones(), 3);
    // }
}






