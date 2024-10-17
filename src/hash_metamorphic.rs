use crate::HashMetamorphic;
use blake2::{Blake2b512, Blake2s256};
use rand;
use rand::RngCore;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};

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
