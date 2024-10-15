use sha3::{Digest, Sha3_256};
use crate::{MetamorphicTest, Mutation, PrimitiveInput};

#[derive(Clone)]
pub struct HashInput {
    input: Vec<u8>
}

impl PrimitiveInput for HashInput {}

struct Sha3BitContributionMetamorphicTest {

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
}

struct SingleBitMutation {
    bit_to_mutate_index: usize
}

impl Mutation<HashInput> for SingleBitMutation {
    const OUTPUT_SHOULD_BE_EQ: bool = false;

    fn mutate_input(&self, input: &HashInput) -> HashInput {
        let unsigned_pos = self.bit_to_mutate_index >> 3;
        let bit_pos = self.bit_to_mutate_index & 7;
        let mut output = input.clone();
        output.input[unsigned_pos] ^= 1 << bit_pos;
        output
    }
}