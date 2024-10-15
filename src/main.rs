use metamorphic_testing_rs::hash_metamorphic::{
    HashInput, Sha2BitContributionMetamorphicTest, Sha3BitContributionMetamorphicTest,
    SingleBitMutation,
};
use metamorphic_testing_rs::MetamorphicTest;

fn main() {
    let res = Sha2BitContributionMetamorphicTest::test_all(&mut SingleBitMutation::new(
        &HashInput::new(&[97, 98]),
    ));
    println!("{}", res);
    let res = Sha3BitContributionMetamorphicTest::test_all(&mut SingleBitMutation::new(
        &HashInput::new(&[97, 98]),
    ));
    println!("{}", res);
}
