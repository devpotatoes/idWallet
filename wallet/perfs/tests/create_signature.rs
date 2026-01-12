use serde::{Serialize, Deserialize};
use pairing_crypto::{
    bbs::{
        ciphersuites::{
            bls12_381::KeyPair,
            bls12_381_g1_sha_256::{sign}
        },
        BbsSignRequest
    }
};

#[derive(Serialize, Deserialize)]
struct Signature {
    signature: String,
    public_key: String
}

const SIGNATURE_HEADER: &[u8; 13] = b"wallet-header";
const KEY_GEN_IKM: &[u8; 49] = b"PKGVYkAHJrc95mGeRwLLVN7JxmHwEPwxEnvhyB7UCQsE4k9Py";
const SIGNATURE_KEY_INFO: &[u8; 15] = b"wallet-key-info";

fn generate_keypair() -> Result<([u8; 32], [u8; 96]), String> {
    let keypair = KeyPair::new(KEY_GEN_IKM, SIGNATURE_KEY_INFO).ok_or("KeyPair generation failed")?;

    Ok((
        keypair.secret_key.to_bytes(),
        keypair.public_key.to_octets()
    ))
}

fn sign_messages(secret_key: &[u8; 32], public_key: &[u8; 96], messages: &[&[u8]]) -> Result<[u8; 80], String> {
    sign(&BbsSignRequest {
        secret_key,
        public_key,
        header: Some(SIGNATURE_HEADER.as_ref()),
        messages: Some(messages)
    }).map_err(|e| e.to_string())
}

#[test]
fn create_signature_10() {
    let messages_array: [&str; 10] = [
        "age",
        "country",
        "phone_number",
        "email_verified",
        "driver_license_valid",
        "student_status",
        "residency",
        "employment_status",
        "voting_rights",
        "subscription_active"
    ];

    let messages_bytes: Vec<Vec<u8>> = messages_array.iter().map(|s| s.as_bytes().to_vec()).collect();
    let messages: Vec<&[u8]> = messages_bytes.iter().map(|m| m.as_slice()).collect();

    let (secret_key, public_key) = generate_keypair().unwrap();
    
    sign_messages(&secret_key, &public_key, &messages).unwrap();
}

#[test]
fn create_signature_100() {
    let messages_array: [&str; 100] = [
        "attribute_01", "attribute_02", "attribute_03", "attribute_04", "attribute_05",
        "attribute_06", "attribute_07", "attribute_08", "attribute_09", "attribute_10",
        "attribute_11", "attribute_12", "attribute_13", "attribute_14", "attribute_15",
        "attribute_16", "attribute_17", "attribute_18", "attribute_19", "attribute_20",
        "attribute_21", "attribute_22", "attribute_23", "attribute_24", "attribute_25",
        "attribute_26", "attribute_27", "attribute_28", "attribute_29", "attribute_30",
        "attribute_31", "attribute_32", "attribute_33", "attribute_34", "attribute_35",
        "attribute_36", "attribute_37", "attribute_38", "attribute_39", "attribute_40",
        "attribute_41", "attribute_42", "attribute_43", "attribute_44", "attribute_45",
        "attribute_46", "attribute_47", "attribute_48", "attribute_49", "attribute_50",
        "attribute_51", "attribute_52", "attribute_53", "attribute_54", "attribute_55",
        "attribute_56", "attribute_57", "attribute_58", "attribute_59", "attribute_60",
        "attribute_61", "attribute_62", "attribute_63", "attribute_64", "attribute_65",
        "attribute_66", "attribute_67", "attribute_68", "attribute_69", "attribute_70",
        "attribute_71", "attribute_72", "attribute_73", "attribute_74", "attribute_75",
        "attribute_76", "attribute_77", "attribute_78", "attribute_79", "attribute_80",
        "attribute_81", "attribute_82", "attribute_83", "attribute_84", "attribute_85",
        "attribute_86", "attribute_87", "attribute_88", "attribute_89", "attribute_90",
        "attribute_91", "attribute_92", "attribute_93", "attribute_94", "attribute_95",
        "attribute_96", "attribute_97", "attribute_98", "attribute_99", "attribute_100"
    ];

    let messages_bytes: Vec<Vec<u8>> = messages_array.iter().map(|s| s.as_bytes().to_vec()).collect();
    let messages: Vec<&[u8]> = messages_bytes.iter().map(|m| m.as_slice()).collect();

    let (secret_key, public_key) = generate_keypair().unwrap();

    sign_messages(&secret_key, &public_key, &messages).unwrap();
}