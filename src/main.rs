use metamorphic_testing_rs::kyber_metamorphic::{KyberBitFlipMetamorphicTest, KyberInput, KyberSingleBitMutation, PossibleKeySize};
use metamorphic_testing_rs::MetamorphicTest;

fn main() {
    let res = KyberBitFlipMetamorphicTest::test_all(&mut KyberSingleBitMutation::new(&KyberInput::new(PossibleKeySize::Key512)));
    println!("{}", res);
}
