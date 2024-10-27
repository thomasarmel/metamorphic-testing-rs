use crate::KEMMetamorphic;

pub struct PQCKyberMetamorphic {}

impl KEMMetamorphic for PQCKyberMetamorphic {
    type SecretKey = [u8; 2400];

    type PublicKey = [u8; 1184];

    type CipherText = [u8; 1088];

    type SharedSecret = [u8; 32];

    const PKSIZE: usize = 1184;

    const SKSIZE: usize = 2400;

    const CTSIZE: usize = 1088;

    const SSSIZE: usize = 32;

    type State = ();

    const LIBNAME: &str = "Kyber Argyle";

    fn gen_keys() -> (Self::SecretKey, Self::PublicKey) {
        let mut rng = rand::thread_rng();
        let keys = pqc_kyber::keypair(&mut rng).unwrap();
        (keys.secret, keys.public)
    }

    fn decaps(sk: &Self::SecretKey, ct: &Self::CipherText) -> Self::SharedSecret {
        pqc_kyber::decapsulate(ct, sk).unwrap()
    }

    fn encaps(pk: &Self::PublicKey) -> (Self::SharedSecret, Self::CipherText) {
        let mut rng = rand::thread_rng();
        let res = pqc_kyber::encapsulate(pk, &mut rng).unwrap();
        (res.1, res.0)
    }

    fn gen_state() -> Self::State {
        ()
    }

    fn get_skey_from_input_as_u8(
        input: &(Self::SecretKey, Self::PublicKey, Self::CipherText),
    ) -> Vec<u8> {
        input.0.to_vec()
    }

    fn get_pkey_from_input_as_u8(
        input: &(Self::SecretKey, Self::PublicKey, Self::CipherText),
    ) -> Vec<u8> {
        input.1.to_vec()
    }

    fn set_skey_from_input_as_u8(
        _state: &Self::State,
        input: &(Self::SecretKey, Self::PublicKey, Self::CipherText),
        key: Vec<u8>,
    ) -> (
        Self::State,
        (Self::SecretKey, Self::PublicKey, Self::CipherText),
    ) {
        let mut out = input.clone();
        out.0 = key.try_into().unwrap();
        ((), out)
    }

    fn set_pkey_from_input_as_u8(
        _state: &Self::State,
        input: &(Self::SecretKey, Self::PublicKey, Self::CipherText),
        key: Vec<u8>,
    ) -> (
        Self::State,
        (Self::SecretKey, Self::PublicKey, Self::CipherText),
    ) {
        let mut out = input.clone();
        out.1 = key.try_into().unwrap();
        ((), out)
    }

    fn output_as_u8(output: Self::SharedSecret) -> Vec<u8> {
        output.to_vec()
    }
}
