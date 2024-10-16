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

pub struct KyberCipherBitFlipMetamorphicTest {}

impl MetamorphicTest for KyberCipherBitFlipMetamorphicTest {
    type Input = KyberInput;
    type Output = Vec<u8>;
    type InputMutation = KyberCipherSingleBitMutation;

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

pub struct KyberSkBitFlipMetamorphicTest {}

impl MetamorphicTest for KyberSkBitFlipMetamorphicTest {
    type Input = KyberInput;
    type Output = Vec<u8>;
    type InputMutation = KyberSkSingleBitMutation;

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

pub struct KyberCipherSingleBitMutation {
    bit_to_mutate_index: usize,
    original_input: KyberInput,
}

impl Mutation<KyberInput> for KyberCipherSingleBitMutation {
    const OUTPUT_SHOULD_BE_EQ: bool = false;

    fn clone_with_new_original_input(&self, new_original_input: &KyberInput) -> Self {
        Self {
            bit_to_mutate_index: 0,
            original_input: new_original_input.clone(),
        }
    }
}

impl KyberCipherSingleBitMutation {
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

impl Iterator for KyberCipherSingleBitMutation {
    type Item = KyberInput;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.mutate_input(&self.original_input);
        self.bit_to_mutate_index += 1;
        res
    }
}

pub struct KyberSkSingleBitMutation {
    bit_to_mutate_index: usize,
    original_input: KyberInput,
}

impl Mutation<KyberInput> for KyberSkSingleBitMutation {
    const OUTPUT_SHOULD_BE_EQ: bool = false;

    fn clone_with_new_original_input(&self, new_original_input: &KyberInput) -> Self {
        Self {
            bit_to_mutate_index: 0,
            original_input: new_original_input.clone(),
        }
    }
}

impl KyberSkSingleBitMutation {
    pub fn new(original_input: &KyberInput) -> Self {
        Self {
            bit_to_mutate_index: 0,
            original_input: original_input.clone(),
        }
    }

    fn mutate_input(&self, input: &KyberInput) -> Option<KyberInput> {
        let bit_count: usize = match &self.original_input.sk {
            PossibleDecapsulate::K512(sk) => {
                sk.dk_pke.s_hat.0.iter().map(|poly| {
                    poly.0.len()
                }).sum::<usize>()
            }
            PossibleDecapsulate::K768(sk) => {
                sk.dk_pke.s_hat.0.iter().map(|poly| {
                    poly.0.len()
                }).sum()
            }
            PossibleDecapsulate::K1024(sk) => {
                sk.dk_pke.s_hat.0.iter().map(|poly| {
                    poly.0.len()
                }).sum()
            }
        } * 16;
        if self.bit_to_mutate_index >= bit_count {
            return None;
        }
        match &self.original_input.sk {
            PossibleDecapsulate::K512(sk) => {
                let mut new_sk = sk.clone();
                let in_len = new_sk.dk_pke.s_hat.0[0].0.len();
                let out_pos = (self.bit_to_mutate_index / 16) / in_len;
                let in_pos = (self.bit_to_mutate_index / 16) % in_len;
                new_sk.dk_pke.s_hat.0[out_pos].0[in_pos].0 ^= 1 << (self.bit_to_mutate_index % 16);
                Some(KyberInput {
                    sk: PossibleDecapsulate::K512(new_sk),
                    enc: input.enc.clone(),
                })
            }
            PossibleDecapsulate::K768(sk) => {
                let mut new_sk = sk.clone();
                let in_len = new_sk.dk_pke.s_hat.0[0].0.len();
                let out_pos = (self.bit_to_mutate_index / 16) / in_len;
                let in_pos = (self.bit_to_mutate_index / 16) % in_len;
                new_sk.dk_pke.s_hat.0[out_pos].0[in_pos].0 ^= 1 << (self.bit_to_mutate_index % 16);
                Some(KyberInput {
                    sk: PossibleDecapsulate::K768(new_sk),
                    enc: input.enc.clone(),
                })
            }
            PossibleDecapsulate::K1024(sk) => {
                let mut new_sk = sk.clone();
                let in_len = new_sk.dk_pke.s_hat.0[0].0.len();
                let out_pos = (self.bit_to_mutate_index / 16) / in_len;
                let in_pos = (self.bit_to_mutate_index / 16) % in_len;
                new_sk.dk_pke.s_hat.0[out_pos].0[in_pos].0 ^= 1 << (self.bit_to_mutate_index % 16);
                Some(KyberInput {
                    sk: PossibleDecapsulate::K1024(new_sk),
                    enc: input.enc.clone(),
                })
            }
        }
    }
}
impl Iterator for KyberSkSingleBitMutation {
    type Item = KyberInput;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.mutate_input(&self.original_input);
        self.bit_to_mutate_index += 1;
        res
    }
}


struct InterestingKyberInputIterator {
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
