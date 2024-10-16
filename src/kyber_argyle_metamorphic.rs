use pqc_kyber::{decapsulate, encapsulate, keypair, KYBER_CIPHERTEXTBYTES, KYBER_PUBLICKEYBYTES, KYBER_SECRETKEYBYTES};
use crate::{MetamorphicTest, Mutation, PrimitiveInput};

#[derive(Clone, Debug)]
pub struct KyberArgyleInput {
    sk: [u8; KYBER_SECRETKEYBYTES],
    pk: [u8; KYBER_PUBLICKEYBYTES],
    cipher: [u8; KYBER_CIPHERTEXTBYTES]
}

impl KyberArgyleInput {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let keys_bob = keypair(&mut rng).unwrap();

        // Alice encapsulates a shared secret using Bob's public key
        let (ciphertext, _) = encapsulate(&keys_bob.public, &mut rng).unwrap();
        //let shared_secret_bob = decapsulate(&ciphertext, &keys_bob.secret).unwrap();
       
        Self {
            sk: keys_bob.secret,
            pk: keys_bob.public,
            cipher: ciphertext,
        }
    }
}

impl PrimitiveInput for KyberArgyleInput {}

pub struct KyberArgyleCipherBitFlipMetamorphicTest {}

impl MetamorphicTest for KyberArgyleCipherBitFlipMetamorphicTest {
    type Input = KyberArgyleInput;
    type Output = [u8; 32];
    type InputMutation = KyberArgyleCipherSingleBitMutation;

    fn call(input: &Self::Input) -> Self::Output {
        decapsulate(&input.cipher, &input.sk).unwrap()
    }

    fn get_interesting_input_iterator() -> Box<dyn Iterator<Item=Self::Input>> {
        Box::new(InterestingKyberArgyleInputIterator::new())
    }
}

pub struct KyberArgyleCipherSingleBitMutation {
    bit_to_mutate_index: usize,
    original_input: KyberArgyleInput,
}

impl Mutation<KyberArgyleInput> for KyberArgyleCipherSingleBitMutation {
    const OUTPUT_SHOULD_BE_EQ: bool = false;

    fn clone_with_new_original_input(&self, new_original_input: &KyberArgyleInput) -> Self {
        Self {
            bit_to_mutate_index: 0,
            original_input: new_original_input.clone(),
        }
    }
}
impl KyberArgyleCipherSingleBitMutation {
    pub fn new(original_input: &KyberArgyleInput) -> Self {
        Self {
            bit_to_mutate_index: 0,
            original_input: original_input.clone(),
        }
    }

    fn mutate_input(&self, input: &KyberArgyleInput) -> Option<KyberArgyleInput> {
        if self.bit_to_mutate_index >= input.cipher.len() * 8 {
            return None;
        }
        let unsigned_pos = self.bit_to_mutate_index >> 3;
        let bit_pos = self.bit_to_mutate_index & 7;
        let mut output = input.clone();
        output.cipher[unsigned_pos] ^= 1 << bit_pos;
        Some(output)
    }
}

impl Iterator for KyberArgyleCipherSingleBitMutation {
    type Item = KyberArgyleInput;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.mutate_input(&self.original_input);
        self.bit_to_mutate_index += 1;
        res
    }
}

struct InterestingKyberArgyleInputIterator {
    count: usize,
}

impl InterestingKyberArgyleInputIterator {
    fn new() -> Self {
        Self { count: 10 }
    }
}

impl Iterator for InterestingKyberArgyleInputIterator {
    type Item = KyberArgyleInput;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        Some(KyberArgyleInput::new())
    }
}
