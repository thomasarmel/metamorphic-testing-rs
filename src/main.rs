use std::io::Write;
use std::ops::Add;
use fake_crypto_rng::FakeCryptoRng;
use num_bigint::BigUint;
use num_traits::{Num, One, Zero};
use pbkdf2::{pbkdf2_hmac, pbkdf2_hmac_array};
use pqc_kyber::{decapsulate, encapsulate, keypair};
use sha2::{Sha256, Sha512};
use rayon::iter::{ParallelBridge, ParallelIterator};
use sha1::Sha1;
use sha3::Digest;
use metamorphic_testing_rs::kyber_argyle_metamorphic::{KyberArgyleCipherBitFlipMetamorphicTest, KyberArgyleCipherSingleBitMutation, KyberArgyleFakeRngMutation, KyberArgyleFakeRngMetamorphicTest, KyberArgyleInput, KyberArgylePkBitFlipMetamorphicTest, KyberArgylePkSingleBitMutation, KyberArgyleSkSingleBitMutation, KyberArgyleSkBitFlipMetamorphicTest};
use metamorphic_testing_rs::kyber_metamorphic::{KyberCipherBitFlipMetamorphicTest, KyberInput, KyberCipherSingleBitMutation, PossibleKeySize, KyberSkSingleBitMutation, KyberSkBitFlipMetamorphicTest};
use metamorphic_testing_rs::MetamorphicTest;

fn main() {
    let password = b"password";
    // number of iterations
    let n = 60;
    // Expected value of generated key
    let mut salt = BigUint::zero();

    num_iter::range_inclusive(BigUint::zero(), BigUint::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF", 16).unwrap())
        .into_iter().par_bridge()
        .for_each(|salt| {
            let key = pbkdf2_hmac_array::<Sha1, 20>(password, &salt.to_bytes_le(), n);
            std::io::stdout().write(&key);
            std::io::stdout().flush();
        });
}
