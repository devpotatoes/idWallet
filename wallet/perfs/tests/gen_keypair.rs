use pairing_crypto::{
    bbs::{
        ciphersuites::{
            bls12_381::KeyPair
        }
    }
};

const KEY_GEN_IKM: &[u8; 49] = b"PKGVYkAHJrc95mGeRwLLVN7JxmHwEPwxEnvhyB7UCQsE4k9Py";
const SIGNATURE_KEY_INFO: &[u8; 15] = b"wallet-key-info";

#[test]
fn generate_keypair() {
    let _ = KeyPair::new(KEY_GEN_IKM, SIGNATURE_KEY_INFO);
}

#[test]
fn generate_keypair_1000() {
    let mut n = 0;

    while n < 1000 {
        let _ = KeyPair::new(KEY_GEN_IKM, SIGNATURE_KEY_INFO);
    
        n += 1;
    }
}