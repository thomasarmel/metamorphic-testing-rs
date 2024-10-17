use metamorphic_testing_rs::{
    hash_metamorphic::{
        Blake2b512Metamorphic, Blake2s256Metamorphic, Sha2_224Metamorphic, Sha2_256Metamorphic,
        Sha2_384Metamorphic, Sha2_512Metamorphic, Sha2_512_224Metamorphic, Sha2_512_256Metamorphic,
        Sha3_224Metamorphic, Sha3_256Metamorphic, Sha3_384Metamorphic, Sha3_512Metamorphic,
    },
    MetamorphicTest,
};

fn main() {
    let test_size_bytes = 1024;
    Blake2s256Metamorphic::run_tests(test_size_bytes);
    Blake2b512Metamorphic::run_tests(test_size_bytes);
    Sha2_256Metamorphic::run_tests(test_size_bytes);
    Sha2_224Metamorphic::run_tests(test_size_bytes);
    Sha2_256Metamorphic::run_tests(test_size_bytes);
    Sha2_384Metamorphic::run_tests(test_size_bytes);
    Sha2_512Metamorphic::run_tests(test_size_bytes);
    Sha2_512_224Metamorphic::run_tests(test_size_bytes);
    Sha2_512_256Metamorphic::run_tests(test_size_bytes);
    Sha3_224Metamorphic::run_tests(test_size_bytes);
    Sha3_256Metamorphic::run_tests(test_size_bytes);
    Sha3_384Metamorphic::run_tests(test_size_bytes);
    Sha3_512Metamorphic::run_tests(test_size_bytes);
}
