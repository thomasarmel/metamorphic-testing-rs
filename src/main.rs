use std::io::Write;
use std::ops::Add;
use num_bigint::BigUint;
use num_traits::One;
use sha3::Digest;
use metamorphic_testing_rs::kyber_argyle_metamorphic::{KyberArgyleCipherBitFlipMetamorphicTest, KyberArgyleCipherSingleBitMutation, KyberArgyleFakeRngMutation, KyberArgyleFakeRngMetamorphicTest, KyberArgyleInput};
use metamorphic_testing_rs::kyber_metamorphic::{KyberCipherBitFlipMetamorphicTest, KyberInput, KyberCipherSingleBitMutation, PossibleKeySize, KyberSkSingleBitMutation, KyberSkBitFlipMetamorphicTest};
use metamorphic_testing_rs::MetamorphicTest;

fn main() {
    let res = KyberArgyleCipherBitFlipMetamorphicTest::test_all(&mut KyberArgyleCipherSingleBitMutation::new(&KyberArgyleInput::new()));
    println!("{}", res);

    let res = KyberArgyleFakeRngMetamorphicTest::test_all(&mut KyberArgyleFakeRngMutation::new(&KyberArgyleInput::new()));
    println!("{}", res);

    /*let mut first_value = BigUint::ZERO;
    loop {
        let mut hasher = sha3::Sha3_256::new();
        hasher.update(first_value.to_bytes_be());
        let hash_result = hasher.finalize().to_vec();
        std::io::stdout().write(&hash_result).unwrap();
        std::io::stdout().flush().unwrap();
        first_value = first_value.add(&BigUint::one());
    }*/
}
