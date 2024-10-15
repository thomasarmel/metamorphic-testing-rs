use metamorphic_testing_rs::kyber_metamorphic::{KyberBitFlipMetamorphicTest, KyberInput, KyberSingleBitMutation};
use metamorphic_testing_rs::MetamorphicTest;

fn main() {
    let res = KyberBitFlipMetamorphicTest::test_all(&mut KyberSingleBitMutation::new(&KyberInput::new()));
    println!("{}", res);
}
