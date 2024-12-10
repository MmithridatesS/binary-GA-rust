mod mutation {
    use std::usize;

    use rand::Rng;

    pub fn toss(prob: u8) -> bool {
        todo!()
    }

    enum MutationError {
        UnmatchedTypeAndSequenceLength,
        NumMutationMoreThanSequenceLength,
        MutationFailure,
    }
    type MutationResult<T> = Result<T, MutationError>;

    trait Mutator<Output>{
        fn mutate(&self, sequence: &mut Output) -> MutationResult<()>;
    }

    struct BitFlipMutator {
        probability: u8,
        sequence_length: u8
    }

    // This Macro should only be used for unsigned integers //
    macro_rules! impl_mut_for_bit_flip_mutator {
        ($($t: ty),*) => {
            $(impl Mutator<$t> for BitFlipMutator {
                fn mutate(&self, sequence: &mut $t) -> MutationResult<()> {
                    let sequence_size: u8 = (std::mem::size_of::<$t>()*8).try_into().unwrap();
                    if (sequence_size < self.sequence_length) {
                        return Err(MutationError::UnmatchedTypeAndSequenceLength);
                    }
                    for pos in 0..self.sequence_length {
                        if toss(self.probability) {
                            *sequence ^= (1<<pos);
                        }
                    }
                    Ok(())
                }
            })*
        };
    }

    impl_mut_for_bit_flip_mutator!(u8, u16, u32, u64, u128, usize);

    struct SinglePointMutator {
        probability: u8,
        sequence_length: u8
    }
    macro_rules! impl_mut_for_singlepoint_mutator {
        ($($u: ty),*) => {
            $(impl Mutator<$u> for SinglePointMutator {
                fn mutate(&self, sequence: &mut $u) -> MutationResult<()> {
                    let sequence_size: u8 = (std::mem::size_of::<$u>()*8).try_into().unwrap();
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

    struct MultiPointMutator {
        probability: u8,
        sequence_length: usize,
        num_mutations: usize
    }

    impl Mutator<u16> for MultiPointMutator {
        fn mutate(&self, sequence: &mut u16) -> MutationResult<()> {
            let sequence_size = std::mem::size_of::<u16>();
            if self.num_mutations > self.sequence_length {
                return Err(MutationError::NumMutationMoreThanSequenceLength);
            }
            if self.sequence_length as usize > sequence_size*8 {
                return Err(MutationError::UnmatchedTypeAndSequenceLength)
            }
            let mut mutator: u16 = 0;
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
    }

    macro_rules! impl_mut_for_multipoint_mutator {
        ($($t:ty),*) => {
            $(impl Mutator<$t> for MultiPointMutator {
                fn mutate(&self, sequence: &mut $t) -> MutationResult<()> {
                    let sequence_size = std::mem::size_of::<$t>();
                    if self.num_mutations > self.sequence_length {
                        return Err(MutationError::NumMutationMoreThanSequenceLength);
                    }
                    if self.sequence_length as usize > sequence_size*8 {
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
}







