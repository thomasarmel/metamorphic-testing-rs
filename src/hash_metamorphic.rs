use sha3::{Digest, Sha3_256};
use crate::{MetamorphicTest, Mutation, PrimitiveInput};
use rand::{distributions::Uniform, Rng};

#[derive(Clone, Debug)]
pub struct HashInput {
    input: Vec<u8>
}

impl HashInput {
    pub fn new(hash_input: &[u8]) -> Self {
        Self {
            input: Vec::from(hash_input)
        }
    }
}

impl PrimitiveInput for HashInput {}

pub struct Sha3BitContributionMetamorphicTest {

}

impl MetamorphicTest for Sha3BitContributionMetamorphicTest {
    type Input = HashInput;
    type Output = Vec<u8>;
    type InputMutation = SingleBitMutation;

    fn output_check(output: &Self::Output, reference_output: &Self::Output, should_be_equal: bool) -> Result<(), ()> {
        if (output == reference_output) == should_be_equal {
            return Ok(());
        }
        Err(())
    }

    fn call(input: &Self::Input) -> Self::Output {
        let mut hasher = Sha3_256::new();
        hasher.update(&input.input);
        let hash: Vec<u8> = hasher.finalize().to_vec();
        hash
    }

    fn get_interesting_input_iterator() -> Box<dyn Iterator<Item=Self::Input>> {
        Box::new(InterestingInputIterator::new())
    }
}

pub struct SingleBitMutation {
    bit_to_mutate_index: usize,
    original_input: HashInput
}

impl SingleBitMutation {
    pub fn new(original_input: &HashInput) -> Self {
        Self {
            bit_to_mutate_index: 0,
            original_input: original_input.clone()
        }
    }

    fn mutate_input(&self, input: &HashInput) -> Option<HashInput> {
        if self.bit_to_mutate_index >= input.input.len() * 8 {
            return None
        }
        let unsigned_pos = self.bit_to_mutate_index >> 3;
        let bit_pos = self.bit_to_mutate_index & 7;
        let mut output = input.clone();
        output.input[unsigned_pos] ^= 1 << bit_pos;
        Some(output)
    }

    pub fn incr(&mut self) {
        self.bit_to_mutate_index += 1;
    }
}

impl Mutation<HashInput> for SingleBitMutation {
    const OUTPUT_SHOULD_BE_EQ: bool = false;

    fn clone_with_new_original_input(&self, new_original_input: &HashInput) -> Self {
        Self {
            bit_to_mutate_index: 0,
            original_input: new_original_input.clone()
        }
    }
}

impl Iterator for SingleBitMutation {
    type Item = HashInput;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.mutate_input(&self.original_input);
        self.incr();
        res
    }
}

struct InterestingInputIterator{
    input_size: usize
}

impl InterestingInputIterator {
    fn new() -> Self {
        Self {
            input_size: 1
        }
    }
}

impl Iterator for InterestingInputIterator {
    type Item = HashInput;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input_size > 1024 {
            return None
        }
        let range = Uniform::from(0..=255);
        let values: Vec<u8> = rand::thread_rng().sample_iter(&range).take(self.input_size).collect();
        self.input_size += 1;
        Some(HashInput {
            input: values
        })
    }
}