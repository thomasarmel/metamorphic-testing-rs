use metamorphic_testing_rs::hash_metamorphic::{HashInput, Sha3BitContributionMetamorphicTest, SingleBitMutation};
use metamorphic_testing_rs::MetamorphicTest;

fn main() {
    let reference_output = vec![92, 130, 139, 51, 57, 127, 71, 98, 146, 46, 57, 166, 12, 53, 105, 157, 37, 80, 70, 106, 82, 221, 21, 237, 68, 218, 55, 235, 11, 220, 97, 230];
    let res = Sha3BitContributionMetamorphicTest::test_all(&reference_output, &mut SingleBitMutation::new(&HashInput::new(&[97, 98])));
    println!("{}", res);
}
