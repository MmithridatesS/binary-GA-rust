mod mutation {
    use std::usize;

    use rand::Rng;

    pub fn toss(prob: u8) -> bool {
        todo!()
    }

    enum MutationError {
        UnmatchedTypeAndSequenceLength,
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
        sequence_length: u8
    }
    macro_rules! impl_mut_for_multipoint_mutator {
        ($($t:ty), +) => {
            todo!()
        };
    }
}







