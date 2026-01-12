use serde::{Serialize, Deserialize};
use pairing_crypto::{
    bbs::{
        ciphersuites::{
            bls12_381_g1_sha_256::{proof_gen, proof_verify, verify}
        },
        BbsProofGenRequest,
        BbsProofGenRevealMessageRequest,
        BbsProofVerifyRequest,
        BbsVerifyRequest
    }
};

const SIGNATURE_HEADER: &[u8; 13] = b"wallet-header";
const SIGNATURE_PRESENTATION_HEADER: &[u8; 26] = b"wallet-presentation-header";

fn verify_messages(public_key: &[u8; 96], signature: &[u8; 80], messages: &[&[u8]]) -> Result<bool, String> {
    verify(&BbsVerifyRequest {
        public_key,
        header: Some(SIGNATURE_HEADER.as_ref()),
        messages: Some(messages),
        signature
    }).map_err(|e| e.to_string())
}

#[test]
fn verify_signature_10() {
    let signature_hex: &str = "b18f6ddee374717374f3d0a14cf8a9c0fa2af86c584f2d299bd41447f1fa01c1f78f840e799d5839a25c006a848d85f5612bc846c3b34c7c68833105cf3d4f157da18897551aa3125ab89a817d3aaa6b";
    let public_key_hex: &str = "b674d5ec0c2e709637193ec97a39f8f757789a5dae6bb77f9150d4f6053d09f2f32c9a51b38eb811557c8fed5bda3b18065d64be925e89c522c01b5c7ffcd036af98899dc7ded6733e8f6ab8dd904e03ef7a2a1923ad3dbbb0e3353d49288ab9"; 
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

    let public_key: [u8; 96] = hex::decode(public_key_hex).expect("Error decode.").try_into().expect("Wrong size.");

    let signature: [u8; 80] = hex::decode(signature_hex).expect("Error decode.").try_into().expect("Wrong size.");

    let messages_bytes: Vec<Vec<u8>> = messages_array.iter().map(|s| s.as_bytes().to_vec()).collect();
    let messages: Vec<&[u8]> = messages_bytes.iter().map(|m| m.as_slice()).collect();

    verify_messages(&public_key, &signature, &messages).unwrap_or(false);
}

#[test]
fn verify_signature_100() {
    let signature_hex: &str = "ae3f46984fed12ec29f38f514fd8260d6b5331d8cca4f80834bc61ee06a01de857387b7e3c721dba9fe3a79217e7255a3b1aa3f37942cfe454956c06aa8a09823b3a047932ac00543f74120d4028730a";
    let public_key_hex: &str = "b674d5ec0c2e709637193ec97a39f8f757789a5dae6bb77f9150d4f6053d09f2f32c9a51b38eb811557c8fed5bda3b18065d64be925e89c522c01b5c7ffcd036af98899dc7ded6733e8f6ab8dd904e03ef7a2a1923ad3dbbb0e3353d49288ab9"; 
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

    let public_key: [u8; 96] = hex::decode(public_key_hex).expect("Error decode.").try_into().expect("Wrong size.");

    let signature: [u8; 80] = hex::decode(signature_hex).expect("Error decode.").try_into().expect("Wrong size.");

    let messages_bytes: Vec<Vec<u8>> = messages_array.iter().map(|s| s.as_bytes().to_vec()).collect();
    let messages: Vec<&[u8]> = messages_bytes.iter().map(|m| m.as_slice()).collect();

    verify_messages(&public_key, &signature, &messages).unwrap_or(false);
}

#[derive(Serialize, Deserialize)]
pub struct DisclosedMessage {
    pub index: usize,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartialSignatureResult {
    pub verified: bool,
    pub disclosed_messages: Vec<DisclosedMessage>,
}

#[test]
fn verify_signature_indices_10() {
    let signature_hex: &str = "b18f6ddee374717374f3d0a14cf8a9c0fa2af86c584f2d299bd41447f1fa01c1f78f840e799d5839a25c006a848d85f5612bc846c3b34c7c68833105cf3d4f157da18897551aa3125ab89a817d3aaa6b";
    let public_key_hex: &str = "b674d5ec0c2e709637193ec97a39f8f757789a5dae6bb77f9150d4f6053d09f2f32c9a51b38eb811557c8fed5bda3b18065d64be925e89c522c01b5c7ffcd036af98899dc7ded6733e8f6ab8dd904e03ef7a2a1923ad3dbbb0e3353d49288ab9";
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
        "subscription_active",
    ];
    let indices_array: [usize; 5] = [0, 2, 4, 6, 8];

    let public_key: [u8; 96] = hex::decode(public_key_hex).expect("Error decode.").try_into().expect("Wrong size.");

    let signature: [u8; 80] = hex::decode(signature_hex).expect("Error decode.").try_into().expect("Wrong size.");

    let proof_messages: Vec<BbsProofGenRevealMessageRequest<_>> = messages_array.iter().enumerate().map(|(i, msg)| {
        BbsProofGenRevealMessageRequest {
            reveal: indices_array.contains(&i),
            value: msg.as_bytes()
        }
    }).collect();

    let disclosed_for_verify: Vec<(usize, &[u8])> = proof_messages.iter().enumerate().filter(|(_, m)| m.reveal).map(|(i, m)| (i, m.value)).collect();

    let proof = proof_gen(&BbsProofGenRequest {
        public_key: &public_key,
        header: Some(SIGNATURE_HEADER.as_ref()),
        messages: Some(&proof_messages),
        signature: &signature,
        presentation_header: Some(SIGNATURE_PRESENTATION_HEADER.as_ref()),
        verify_signature: None
    }).expect("Error proof generation.");

    proof_verify(&BbsProofVerifyRequest {
        public_key: &public_key,
        header: Some(SIGNATURE_HEADER.as_ref()),
        presentation_header: Some(SIGNATURE_PRESENTATION_HEADER.as_ref()),
        proof: &proof,
        messages: Some(&disclosed_for_verify)
    }).unwrap_or(false);
}

#[test]
fn verify_signature_indices_100() {
    let signature_hex: &str = "ae3f46984fed12ec29f38f514fd8260d6b5331d8cca4f80834bc61ee06a01de857387b7e3c721dba9fe3a79217e7255a3b1aa3f37942cfe454956c06aa8a09823b3a047932ac00543f74120d4028730a";
    let public_key_hex: &str = "b674d5ec0c2e709637193ec97a39f8f757789a5dae6bb77f9150d4f6053d09f2f32c9a51b38eb811557c8fed5bda3b18065d64be925e89c522c01b5c7ffcd036af98899dc7ded6733e8f6ab8dd904e03ef7a2a1923ad3dbbb0e3353d49288ab9"; 
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
    let indices_array: [usize; 50] = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94, 96, 98];

    let public_key: [u8; 96] = hex::decode(public_key_hex).expect("Error decode.").try_into().expect("Wrong size.");

    let signature: [u8; 80] = hex::decode(signature_hex).expect("Error decode.").try_into().expect("Wrong size.");

    let proof_messages: Vec<BbsProofGenRevealMessageRequest<_>> = messages_array.iter().enumerate().map(|(i, msg)| {
        BbsProofGenRevealMessageRequest {
            reveal: indices_array.contains(&i),
            value: msg.as_bytes()
        }
    }).collect();

    let disclosed_for_verify: Vec<(usize, &[u8])> = proof_messages.iter().enumerate().filter(|(_, m)| m.reveal).map(|(i, m)| (i, m.value)).collect();

    let proof = proof_gen(&BbsProofGenRequest {
        public_key: &public_key,
        header: Some(SIGNATURE_HEADER.as_ref()),
        messages: Some(&proof_messages),
        signature: &signature,
        presentation_header: Some(SIGNATURE_PRESENTATION_HEADER.as_ref()),
        verify_signature: None
    }).expect("Error proof generation.");

    proof_verify(&BbsProofVerifyRequest {
        public_key: &public_key,
        header: Some(SIGNATURE_HEADER.as_ref()),
        presentation_header: Some(SIGNATURE_PRESENTATION_HEADER.as_ref()),
        proof: &proof,
        messages: Some(&disclosed_for_verify)
    }).unwrap_or(false);
}