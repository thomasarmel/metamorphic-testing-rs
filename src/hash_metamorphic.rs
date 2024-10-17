use crate::HashMetamorphic;
use ascon_hash::AsconHash;
use belt_hash::BeltHash;
use blake2::{Blake2b512, Blake2s256};
use blake3::Hasher;
use fsb::{Fsb160, Fsb224, Fsb256, Fsb384, Fsb512};
use groestl::{Groestl224, Groestl256, Groestl384, Groestl512};
use jh::{Jh224, Jh256, Jh384, Jh512};
use rand;
use rand::RngCore;
use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320};
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use shabal::{Shabal192, Shabal224, Shabal256, Shabal384, Shabal512};
use skein::{consts::U32, Skein1024, Skein256, Skein512};
use sm3::Sm3;
use tiger::Tiger;
use whirlpool::Whirlpool;

macro_rules! impl_hash_metamorphic {
    ($hash_type:ty, $test_struct_name:ident, $libname:literal) => {
        pub struct $test_struct_name {}
        impl HashMetamorphic for $test_struct_name {
            type Input = Vec<u8>;
            type Output = Vec<u8>;
            type State = $hash_type;

            const LIBNAME: &str = $libname;

            fn gen_input(size: usize) -> Self::Input {
                let mut data = vec![0u8; size];
                rand::thread_rng().fill_bytes(&mut data);
                data
            }

            fn input_as_u8(input: &Self::Input) -> Vec<u8> {
                input.to_owned()
            }

            fn output_as_u8(output: &Self::Output) -> Vec<u8> {
                output.to_owned()
            }

            fn compare_output(initial_output: &Self::Output, output: &Self::Output) -> bool {
                initial_output == output
            }

            fn hash(state: Self::State, input: &Self::Input) -> Self::Output {
                let mut hasher = state;

                hasher.update(input);

                hasher.finalize().to_vec()
            }

            fn initial_state() -> Self::State {
                Self::State::new()
            }

            fn u8_as_input(
                intial_state: &Self::State,
                _initial_input: &Self::Input,
                mutated: Vec<u8>,
            ) -> (Self::State, Self::Input) {
                (intial_state.clone(), mutated)
            }

            fn hash_update(
                initial_state: &Self::State,
                _initial_input: &Self::Input,
                first_part: &[u8],
                second_part: &[u8],
            ) -> (Self::State, Self::Input) {
                let mut state = initial_state.clone();
                state.update(first_part);
                (state, second_part.to_vec())
            }
        }
    };
}

impl_hash_metamorphic! {Whirlpool, WhirlpoolMetamorphic, "Whirlpool"}
impl_hash_metamorphic! {Tiger, TigerMetamorphic, "Tiger"}
impl_hash_metamorphic! {Sm3, Sm3Metamorphic, "Sm3"}
impl_hash_metamorphic! {Skein256<U32>, Skein256Metamorphic, "Skein256"}
impl_hash_metamorphic! {Skein512<U32>, Skein512Metamorphic, "Skein512"}
impl_hash_metamorphic! {Skein1024<U32>, Skein1024Metamorphic, "Skein1024"}
impl_hash_metamorphic! {Shabal192, Shabal192Metamorphic, "Shabal192"}
impl_hash_metamorphic! {Shabal224, Shabal224Metamorphic, "Shabal224"}
impl_hash_metamorphic! {Shabal256, Shabal256Metamorphic, "Shabal256"}
impl_hash_metamorphic! {Shabal384, Shabal384Metamorphic, "Shabal384"}
impl_hash_metamorphic! {Shabal512, Shabal512Metamorphic, "Shabal512"}
impl_hash_metamorphic! {Ripemd128, Ripemd128Metamorphic, "Ripemd128"}
impl_hash_metamorphic! {Ripemd160, Ripemd160Metamorphic, "Ripemd160"}
impl_hash_metamorphic! {Ripemd256, Ripemd256Metamorphic, "Ripemd256"}
impl_hash_metamorphic! {Ripemd320, Ripemd320Metamorphic, "Ripemd320"}
impl_hash_metamorphic! {Jh224, Jh224Metamorphic, "Jh224"}
impl_hash_metamorphic! {Jh256, Jh256Metamorphic, "Jh256"}
impl_hash_metamorphic! {Jh384, Jh384Metamorphic, "Jh384"}
impl_hash_metamorphic! {Jh512, Jh512Metamorphic, "Jh512"}
impl_hash_metamorphic! {Groestl224, Groestl224Metamorphic, "Groestl224"}
impl_hash_metamorphic! {Groestl256, Groestl256Metamorphic, "Groestl256"}
impl_hash_metamorphic! {Groestl384, Groestl384Metamorphic, "Groestl384"}
impl_hash_metamorphic! {Groestl512, Groestl512Metamorphic, "Groestl512"}
impl_hash_metamorphic! {Fsb160, Fsb160Metamorphic, "Fsb160"}
impl_hash_metamorphic! {Fsb224, Fsb224Metamorphic, "Fsb224"}
impl_hash_metamorphic! {Fsb256, Fsb256Metamorphic, "Fsb256"}
impl_hash_metamorphic! {Fsb384, Fsb384Metamorphic, "Fsb384"}
impl_hash_metamorphic! {Fsb512, Fsb512Metamorphic, "Fsb512"}
impl_hash_metamorphic! {AsconHash, AsconMetamorphic, "Ascon"}
impl_hash_metamorphic! {BeltHash, BeltMetamorphic, "BelT"}
impl_hash_metamorphic! {Blake2b512, Blake2b512Metamorphic, "Blake2b512"}
impl_hash_metamorphic! {Blake2s256, Blake2s256Metamorphic, "Blake2s256"}
impl_hash_metamorphic! {Sha224, Sha2_224Metamorphic, "Sha2_224"}
impl_hash_metamorphic! {Sha256, Sha2_256Metamorphic, "Sha2_256"}
impl_hash_metamorphic! {Sha384, Sha2_384Metamorphic, "Sha2_384"}
impl_hash_metamorphic! {Sha512, Sha2_512Metamorphic, "Sha2_512"}
impl_hash_metamorphic! {Sha512_224, Sha2_512_224Metamorphic, "Sha2_512_224"}
impl_hash_metamorphic! {Sha512_256, Sha2_512_256Metamorphic, "Sha2_512_256"}
impl_hash_metamorphic! {Sha3_224, Sha3_224Metamorphic, "Sha3_224"}
impl_hash_metamorphic! {Sha3_256, Sha3_256Metamorphic, "Sha3_256"}
impl_hash_metamorphic! {Sha3_384, Sha3_384Metamorphic, "Sha3_384"}
impl_hash_metamorphic! {Sha3_512, Sha3_512Metamorphic, "Sha3_512"}

pub struct Blake3Metamorphic {}
impl HashMetamorphic for Blake3Metamorphic {
    type Input = Vec<u8>;
    type Output = Vec<u8>;
    type State = Hasher;

    const LIBNAME: &str = "Blake3";

    fn gen_input(size: usize) -> Self::Input {
        let mut data = vec![0u8; size];
        rand::thread_rng().fill_bytes(&mut data);
        data
    }

    fn input_as_u8(input: &Self::Input) -> Vec<u8> {
        input.to_owned()
    }

    fn output_as_u8(output: &Self::Output) -> Vec<u8> {
        output.to_owned()
    }

    fn compare_output(initial_output: &Self::Output, output: &Self::Output) -> bool {
        initial_output == output
    }

    fn hash(state: Self::State, input: &Self::Input) -> Self::Output {
        let mut hasher = state;

        hasher.update(input);

        hasher.finalize().as_bytes().to_vec()
    }

    fn initial_state() -> Self::State {
        Self::State::new()
    }

    fn u8_as_input(
        intial_state: &Self::State,
        _initial_input: &Self::Input,
        mutated: Vec<u8>,
    ) -> (Self::State, Self::Input) {
        (intial_state.clone(), mutated)
    }

    fn hash_update(
        initial_state: &Self::State,
        _initial_input: &Self::Input,
        first_part: &[u8],
        second_part: &[u8],
    ) -> (Self::State, Self::Input) {
        let mut state = initial_state.clone();
        state.update(first_part);
        (state, second_part.to_vec())
    }
}
