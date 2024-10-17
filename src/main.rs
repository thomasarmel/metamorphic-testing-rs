use std::io::Write;
use std::ops::Add;
use fake_crypto_rng::FakeCryptoRng;
use num_bigint::BigUint;
use num_traits::{Num, One, Zero};
use pbkdf2::{pbkdf2_hmac, pbkdf2_hmac_array};
use pqc_kyber::{decapsulate, encapsulate, keypair};
use sha2::{compress256, compress512, Sha256, Sha512};
use rayon::iter::{ParallelBridge, ParallelIterator};
use sha1::Sha1;
use sha3::Digest;
use metamorphic_testing_rs::kyber_argyle_metamorphic::{KyberArgyleCipherBitFlipMetamorphicTest, KyberArgyleCipherSingleBitMutation, KyberArgyleFakeRngMutation, KyberArgyleFakeRngMetamorphicTest, KyberArgyleInput, KyberArgylePkBitFlipMetamorphicTest, KyberArgylePkSingleBitMutation, KyberArgyleSkSingleBitMutation, KyberArgyleSkBitFlipMetamorphicTest};
use metamorphic_testing_rs::kyber_metamorphic::{KyberCipherBitFlipMetamorphicTest, KyberInput, KyberCipherSingleBitMutation, PossibleKeySize, KyberSkSingleBitMutation, KyberSkBitFlipMetamorphicTest};
use metamorphic_testing_rs::MetamorphicTest;

fn main() {
    let mut state = [0u64; 8];
    let mut block = BigUint::zero();
    loop {
        let mut block_vec = block.to_bytes_le();
        block_vec.resize(128, 0);
        let block_vec: [u8; 128] = block_vec.try_into().unwrap();
        compress512(&mut state, &[block_vec.into()]);
        let state_u8: [u8; 64] = unsafe {std::mem::transmute(state) };
        std::io::stdout().write(&state_u8);
        std::io::stdout().flush();
        block += BigUint::one();
    }
}
