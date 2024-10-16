use fake_crypto_rng::FakeCryptoRng;
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
        //let mut rng = rand::thread_rng();
        let mut rng = FakeCryptoRng::new(0);
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

pub struct KyberArgylePkBitFlipMetamorphicTest {}

pub struct KyberArgyleSkBitFlipMetamorphicTest {}

pub struct AllArgyleKyberOutput {
    plain: [u8; 32],
    pk: [u8; KYBER_PUBLICKEYBYTES],
    sk: [u8; KYBER_SECRETKEYBYTES],
    cipher: [u8; KYBER_CIPHERTEXTBYTES]
}

pub struct PlainAndCipherArgyleKyberOutput {
    plain: [u8; 32],
    cipher: [u8; KYBER_CIPHERTEXTBYTES]
}

impl PartialEq for AllArgyleKyberOutput {
    fn eq(&self, other: &Self) -> bool { // actually we test that all fields are different, this is illogical but this is legacy code '^^
        self.plain == other.plain || self.pk == other.pk || self.sk == other.sk || self.cipher == other.cipher
    }
}

impl PartialEq for PlainAndCipherArgyleKyberOutput {
    fn eq(&self, other: &Self) -> bool { // actually we test that all fields are different, this is illogical but this is legacy code '^^
        self.plain == other.plain || self.cipher == other.cipher
    }
}

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

impl MetamorphicTest for KyberArgylePkBitFlipMetamorphicTest {
    type Input = KyberArgyleInput;
    type Output = PlainAndCipherArgyleKyberOutput;
    type InputMutation = KyberArgylePkSingleBitMutation;

    fn call(input: &Self::Input) -> Self::Output {
        PlainAndCipherArgyleKyberOutput {
            plain: decapsulate(&input.cipher, &input.sk).unwrap(),
            cipher: input.cipher,
        }
    }

    fn get_interesting_input_iterator() -> Box<dyn Iterator<Item=Self::Input>> {
        Box::new(InterestingKyberArgyleInputIterator::new())
    }
}

impl MetamorphicTest for KyberArgyleSkBitFlipMetamorphicTest {
    type Input = KyberArgyleInput;
    type Output = [u8; 32];
    type InputMutation = KyberArgyleSkSingleBitMutation;

    fn call(input: &Self::Input) -> Self::Output {
        //if 2130435834 != crc32fast::hash(&decapsulate(&input.cipher, &input.sk).unwrap()) {
            println!("{} -> {} ({})", crc32fast::hash(&input.sk), crc32fast::hash(&decapsulate(&input.cipher, &input.sk).unwrap()), crc32fast::hash(&input.cipher));
        //}
        //println!("{:?}", decapsulate(&input.cipher, &input.sk).unwrap());
        decapsulate(&input.cipher, &input.sk).unwrap()
    }

    fn get_interesting_input_iterator() -> Box<dyn Iterator<Item=Self::Input>> {
        Box::new(InterestingKyberArgyleInputIterator::new())
    }
}

pub struct KyberArgyleFakeRngMetamorphicTest {}

impl MetamorphicTest for KyberArgyleFakeRngMetamorphicTest {
    type Input = KyberArgyleInput;
    type Output = AllArgyleKyberOutput;
    type InputMutation = KyberArgyleFakeRngMutation;

    fn call(input: &Self::Input) -> Self::Output {
        AllArgyleKyberOutput {
            plain: decapsulate(&input.cipher, &input.sk).unwrap(),
            pk: input.pk,
            sk: input.sk,
            cipher: input.cipher,
        }
    }

    fn get_interesting_input_iterator() -> Box<dyn Iterator<Item=Self::Input>> {
        Box::new(InterestingKyberArgyleInputIterator::new())
    }
}

pub struct KyberArgyleCipherSingleBitMutation {
    bit_to_mutate_index: usize,
    original_input: KyberArgyleInput,
}

pub struct KyberArgylePkSingleBitMutation {
    bit_to_mutate_index: usize,
    original_input: KyberArgyleInput,
}

pub struct KyberArgyleSkSingleBitMutation {
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

impl Mutation<KyberArgyleInput> for KyberArgylePkSingleBitMutation {
    const OUTPUT_SHOULD_BE_EQ: bool = false;

    fn clone_with_new_original_input(&self, new_original_input: &KyberArgyleInput) -> Self {
        Self {
            bit_to_mutate_index: 0,
            original_input: new_original_input.clone(),
        }
    }
}

impl Mutation<KyberArgyleInput> for KyberArgyleSkSingleBitMutation {
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

impl KyberArgylePkSingleBitMutation {
    pub fn new(original_input: &KyberArgyleInput) -> Self {
        Self {
            bit_to_mutate_index: 0,
            original_input: original_input.clone(),
        }
    }

    fn mutate_input(&self, input: &KyberArgyleInput) -> Option<KyberArgyleInput> {
        if self.bit_to_mutate_index >= input.pk.len() * 8 {
            return None;
        }
        let mut rng = FakeCryptoRng::new(0);
        let unsigned_pos = self.bit_to_mutate_index >> 3;
        let bit_pos = self.bit_to_mutate_index & 7;
        let mut output = input.clone();
        output.pk[unsigned_pos] ^= 1 << bit_pos;
        let (cipher, _) = encapsulate(&output.pk, &mut rng).unwrap();
        output.cipher = cipher; // we need to change the cipher too
        Some(output)
    }
}

impl KyberArgyleSkSingleBitMutation {
    pub fn new(original_input: &KyberArgyleInput) -> Self {
        Self {
            bit_to_mutate_index: 0,
            original_input: original_input.clone(),
        }
    }

    fn mutate_input(&self, input: &KyberArgyleInput) -> Option<KyberArgyleInput> {
        if self.bit_to_mutate_index >= input.sk.len() * 8 {
            return None;
        }
        let unsigned_pos = self.bit_to_mutate_index >> 3;
        let bit_pos = self.bit_to_mutate_index & 7;
        let mut output = input.clone();
        output.sk[unsigned_pos] ^= 1 << bit_pos;
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

impl Iterator for KyberArgylePkSingleBitMutation {
    type Item = KyberArgyleInput;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.mutate_input(&self.original_input);
        self.bit_to_mutate_index += 1;
        res
    }
}

impl Iterator for KyberArgyleSkSingleBitMutation {
    type Item = KyberArgyleInput;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.mutate_input(&self.original_input);
        self.bit_to_mutate_index += 1;
        res
    }
}

pub struct KyberArgyleFakeRngMutation {
    original_input: KyberArgyleInput,
    index: usize
}

impl Mutation<KyberArgyleInput> for KyberArgyleFakeRngMutation {
    const OUTPUT_SHOULD_BE_EQ: bool = false;

    fn clone_with_new_original_input(&self, new_original_input: &KyberArgyleInput) -> Self {
        Self {
            original_input: new_original_input.clone(),
            index: 1
        }
    }
}

impl KyberArgyleFakeRngMutation {
    pub fn new(original_input: &KyberArgyleInput) -> Self {
        Self {
            original_input: original_input.clone(),
            index: 1
        }
    }

    fn mutate_input(&self, _input: &KyberArgyleInput) -> Option<KyberArgyleInput> {
        if self.index == 0 {
            return None
        }
        let mut rng = FakeCryptoRng::new(1);
        let keys_bob = keypair(&mut rng).unwrap();

        // Alice encapsulates a shared secret using Bob's public key
        let (ciphertext, _) = encapsulate(&keys_bob.public, &mut rng).unwrap();
        //let shared_secret_bob = decapsulate(&ciphertext, &keys_bob.secret).unwrap();

        Some(KyberArgyleInput {
            sk: keys_bob.secret,
            pk: keys_bob.public,
            cipher: ciphertext,
        })
    }
}

impl Iterator for KyberArgyleFakeRngMutation {
    type Item = KyberArgyleInput;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.mutate_input(&self.original_input);
        self.index -= 1;
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
