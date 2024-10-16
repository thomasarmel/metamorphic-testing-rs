use ml_kem::{KemCore, MlKem1024, MlKem1024Params, MlKem512, MlKem512Params, MlKem768, MlKem768Params};
use ml_kem::array::Array;
use ml_kem::kem::{Decapsulate, DecapsulationKey, Encapsulate};
use crate::{MetamorphicTest, Mutation, PrimitiveInput};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum PossibleKeySize {
    Key512,
    Key768,
    Key1024
}

#[derive(Clone, Debug)]
enum PossibleDecapsulate {
    K512(DecapsulationKey<MlKem512Params>),
    K768(DecapsulationKey<MlKem768Params>),
    K1024(DecapsulationKey<MlKem1024Params>)
}

#[derive(Clone, Debug)]
pub struct KyberInput {
    sk: PossibleDecapsulate,
    enc: Vec<u8>
}

impl KyberInput {
    pub fn new(possible_key_size: PossibleKeySize) -> Self {
        let mut rng = rand::thread_rng();
        let (dk, ct) = match possible_key_size {
            PossibleKeySize::Key512 => {
                let (dk, ek) = MlKem512::generate(&mut rng);
                let (ct, _) = ek.encapsulate(&mut rng).unwrap();
                (PossibleDecapsulate::K512(dk), ct.0.to_vec())
            }
            PossibleKeySize::Key768 => {
                let (dk, ek) = MlKem768::generate(&mut rng);
                let (ct, _) = ek.encapsulate(&mut rng).unwrap();
                (PossibleDecapsulate::K768(dk), ct.0.to_vec())
            }
            PossibleKeySize::Key1024 => {
                let (dk, ek) = MlKem1024::generate(&mut rng);
                let (ct, _) = ek.encapsulate(&mut rng).unwrap();
                (PossibleDecapsulate::K1024(dk), ct.0.to_vec())
            }
        };

        Self {
            sk: dk,
            enc: ct,
        }
    }
}

impl PrimitiveInput for KyberInput {}

pub struct KyberBitFlipMetamorphicTest {}

impl MetamorphicTest for KyberBitFlipMetamorphicTest {
    type Input = KyberInput;
    type Output = Vec<u8>;
    type InputMutation = KyberSingleBitMutation;

    fn call(input: &Self::Input) -> Self::Output {
        let k_recv = match &input.sk {
            PossibleDecapsulate::K512(sk) => sk.decapsulate(&Array(input.enc.clone().try_into().unwrap())).unwrap(),
            PossibleDecapsulate::K768(sk) => sk.decapsulate(&Array(input.enc.clone().try_into().unwrap())).unwrap(),
            PossibleDecapsulate::K1024(sk) => sk.decapsulate(&Array(input.enc.clone().try_into().unwrap())).unwrap()
        };
        k_recv.to_vec()
    }

    fn get_interesting_input_iterator() -> Box<dyn Iterator<Item=Self::Input>> {
        Box::new(InterestingKyberInputIterator::new())
    }
}

pub struct KyberSingleBitMutation {
    bit_to_mutate_index: usize,
    original_input: KyberInput,
}

impl Mutation<KyberInput> for KyberSingleBitMutation {
    const OUTPUT_SHOULD_BE_EQ: bool = false;

    fn clone_with_new_original_input(&self, new_original_input: &KyberInput) -> Self {
        Self {
            bit_to_mutate_index: 0,
            original_input: new_original_input.clone(),
        }
    }
}

impl KyberSingleBitMutation {
    pub fn new(original_input: &KyberInput) -> Self {
        Self {
            bit_to_mutate_index: 0,
            original_input: original_input.clone(),
        }
    }

    fn mutate_input(&self, input: &KyberInput) -> Option<KyberInput> {
        if self.bit_to_mutate_index >= input.enc.len() * 8 {
            return None;
        }
        let unsigned_pos = self.bit_to_mutate_index >> 3;
        let bit_pos = self.bit_to_mutate_index & 7;
        let mut output = input.clone();
        output.enc[unsigned_pos] ^= 1 << bit_pos;
        Some(output)
    }

}

impl Iterator for KyberSingleBitMutation {
    type Item = KyberInput;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.mutate_input(&self.original_input);
        self.bit_to_mutate_index += 1;
        res
    }
}

struct InterestingKyberInputIterator { // TODO different Kyber key sizes
    key_size_iter: PossibleKeySizeIter,
}

impl InterestingKyberInputIterator {
    fn new() -> Self {
        Self { key_size_iter: PossibleKeySize::iter() }
    }
}

impl Iterator for InterestingKyberInputIterator {
    type Item = KyberInput;

    fn next(&mut self) -> Option<Self::Item> {
        Some(KyberInput::new(self.key_size_iter.next()?))
    }
}
