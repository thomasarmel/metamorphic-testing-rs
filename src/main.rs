use metamorphic_testing_rs::{
    hash_metamorphic::{
        AsconMetamorphic, BeltMetamorphic, Blake2b512Metamorphic, Blake2s256Metamorphic,
        Blake3Metamorphic, Fsb160Metamorphic, Fsb224Metamorphic, Fsb256Metamorphic,
        Fsb384Metamorphic, Fsb512Metamorphic, Groestl224Metamorphic, Groestl256Metamorphic,
        Groestl384Metamorphic, Groestl512Metamorphic, Jh224Metamorphic, Jh256Metamorphic,
        Jh384Metamorphic, Jh512Metamorphic, Ripemd128Metamorphic, Ripemd160Metamorphic,
        Ripemd256Metamorphic, Ripemd320Metamorphic, Sha2_224Metamorphic, Sha2_256Metamorphic,
        Sha2_384Metamorphic, Sha2_512Metamorphic, Sha2_512_224Metamorphic, Sha2_512_256Metamorphic,
        Sha3_224Metamorphic, Sha3_256Metamorphic, Sha3_384Metamorphic, Sha3_512Metamorphic,
        Shabal192Metamorphic, Shabal224Metamorphic, Shabal256Metamorphic, Shabal384Metamorphic,
        Shabal512Metamorphic, Skein1024Metamorphic, Skein256Metamorphic, Skein512Metamorphic,
        Sm3Metamorphic, TigerMetamorphic, WhirlpoolMetamorphic,
    },
    MetamorphicTest,
};

fn main() {
    let test_size_bytes = 1024;

    Blake3Metamorphic::run_tests(test_size_bytes);
    WhirlpoolMetamorphic::run_tests(test_size_bytes);
    TigerMetamorphic::run_tests(test_size_bytes);
    Sm3Metamorphic::run_tests(test_size_bytes);
    Skein256Metamorphic::run_tests(test_size_bytes);
    Skein512Metamorphic::run_tests(test_size_bytes);
    Skein1024Metamorphic::run_tests(test_size_bytes);
    Shabal192Metamorphic::run_tests(test_size_bytes);
    Shabal224Metamorphic::run_tests(test_size_bytes);
    Shabal256Metamorphic::run_tests(test_size_bytes);
    Shabal384Metamorphic::run_tests(test_size_bytes);
    Shabal512Metamorphic::run_tests(test_size_bytes);
    Ripemd128Metamorphic::run_tests(test_size_bytes);
    Ripemd160Metamorphic::run_tests(test_size_bytes);
    Ripemd256Metamorphic::run_tests(test_size_bytes);
    Ripemd320Metamorphic::run_tests(test_size_bytes);
    Jh224Metamorphic::run_tests(test_size_bytes);
    Jh256Metamorphic::run_tests(test_size_bytes);
    Jh384Metamorphic::run_tests(test_size_bytes);
    Jh512Metamorphic::run_tests(test_size_bytes);
    Groestl224Metamorphic::run_tests(test_size_bytes);
    Groestl256Metamorphic::run_tests(test_size_bytes);
    Groestl384Metamorphic::run_tests(test_size_bytes);
    Groestl512Metamorphic::run_tests(test_size_bytes);
    Fsb160Metamorphic::run_tests(test_size_bytes);
    Fsb224Metamorphic::run_tests(test_size_bytes);
    Fsb256Metamorphic::run_tests(test_size_bytes);
    Fsb384Metamorphic::run_tests(test_size_bytes);
    Fsb512Metamorphic::run_tests(test_size_bytes);
    BeltMetamorphic::run_tests(test_size_bytes);
    AsconMetamorphic::run_tests(test_size_bytes);
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
