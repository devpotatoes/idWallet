#![allow(non_snake_case)]

use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use pairing_crypto::{
    bbs::{
        ciphersuites::{
            bls12_381::KeyPair,
            bls12_381_g1_sha_256::{proof_gen, proof_verify, sign, verify}
        },
        BbsProofGenRequest,
        BbsProofGenRevealMessageRequest,
        BbsProofVerifyRequest,
        BbsSignRequest,
        BbsVerifyRequest
    }
};
use hex;

const ACCOUNTS_FILE_PATH: &str = "../../data/accounts.json";

fn check_data_file() -> std::io::Result<()> {
    let file_path = Path::new(ACCOUNTS_FILE_PATH);

    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    if !file_path.exists() {
        fs::File::create(file_path)?;
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct AccountFileData {
    accountsArray: Vec<Account>
}

#[derive(Serialize, Deserialize, Clone)]
struct Wallet {
    country: String,
    sex: String,
    dateOfBirth: String,
    placeOfBirth: String,
    documentNo: String,
    expiryDate: String
}

#[derive(Serialize, Deserialize, Clone)]
struct Account {
    surname: String,
    name: String,
    password: String,
    walletArray: Vec<Wallet>
}

#[derive(Serialize, Deserialize)]
struct IdentityCard {
    surname: String,
    name: String,
    country: String,
    sex: String,
    dateOfBirth: String,
    placeOfBirth: String,
    documentNo: String,
    expiryDate: String
}

fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);

    let result = hasher.finalize();
    format!("{:x}", result)
}

#[tauri::command]
fn login_account(surname: &str, name: &str, password: &str) -> bool {
    #[derive(Serialize, Deserialize)]
    struct Args {
        surname: String,
        name: String,
        password: String
    }

    let accounts_file_raw = fs::read_to_string(ACCOUNTS_FILE_PATH).unwrap_or_else(|_| "{\"accountsArray\":[]}".to_string());

    let accounts_file_data: AccountFileData = serde_json::from_str(&accounts_file_raw).unwrap_or(AccountFileData { accountsArray: vec![] });

    let is_account_already_created = accounts_file_data.accountsArray.iter().any(|entry| entry.surname == surname && entry.name == name);

    if is_account_already_created == true {
        let password_hash = accounts_file_data.accountsArray.iter().find(|e| e.surname == surname && e.name == name).map(|e| e.password.clone());
        
        if Some(sha256(password)) == password_hash {
            return true;
        }
    }

    false
}

#[tauri::command]
fn create_account(surname: &str, name: &str, password: &str) -> bool {
    let accounts_file_raw = fs::read_to_string(ACCOUNTS_FILE_PATH).unwrap_or_else(|_| "{\"accountsArray\":[]}".to_string());

    let mut accounts_file_data: AccountFileData = serde_json::from_str(&accounts_file_raw).unwrap_or(AccountFileData { accountsArray: vec![] });

    let is_account_already_created = accounts_file_data.accountsArray.iter().any(|entry| entry.surname == surname && entry.name == name);

    if is_account_already_created == false {
        let account = Account {
            surname: surname.to_string(),
            name: name.to_string(),
            password: sha256(password),
            walletArray: vec![],
        };

        accounts_file_data.accountsArray.push(account);

        let json_string = match serde_json::to_string_pretty(&accounts_file_data) {
            Ok(s) => s,
            Err(_) => return false
        };

        if fs::write(ACCOUNTS_FILE_PATH, json_string).is_err() {
            return false;
        }
    }

    true
}


#[tauri::command]
fn create_card(surname: &str, name: &str, country: &str, sex: &str, date_of_birth: &str, place_of_birth: &str, document_no: &str, expiry_date: &str) -> bool {
    let accounts_file_raw = fs::read_to_string(ACCOUNTS_FILE_PATH).unwrap_or_else(|_| "{\"accountsArray\":[]}".to_string());

    let mut accounts_file_data: AccountFileData = serde_json::from_str(&accounts_file_raw).unwrap_or(AccountFileData { accountsArray: vec![] });

    let new_wallet = Wallet {
        country: country.to_string(),
        sex: sex.to_string(),
        dateOfBirth: date_of_birth.to_string(),
        placeOfBirth: place_of_birth.to_string(),
        documentNo: document_no.to_string(),
        expiryDate: expiry_date.to_string()
    };

    if let Some(account) = accounts_file_data.accountsArray.iter_mut().find(|acc| acc.surname == surname && acc.name == name) {
        account.walletArray.push(new_wallet);
    } else {
        return false;
    }

    let json_string = match serde_json::to_string_pretty(&accounts_file_data) {
        Ok(s) => s,
        Err(_) => return false
    };

    if fs::write(ACCOUNTS_FILE_PATH, json_string).is_err() {
        return false;
    }

    true
}

#[tauri::command]
fn fetch_wallet_data(surname: &str, name: &str) -> Vec<IdentityCard> {
    let accounts_file_raw = fs::read_to_string(ACCOUNTS_FILE_PATH).unwrap_or_else(|_| "{\"accountsArray\":[]}".to_string());

    let accounts_file_data: AccountFileData = serde_json::from_str(&accounts_file_raw).unwrap_or(AccountFileData { accountsArray: vec![] });

    if let Some(account) = accounts_file_data.accountsArray.into_iter().find(|acc| acc.surname == surname && acc.name == name) {
        return account.walletArray.into_iter().map(|item| IdentityCard {
            surname: account.surname.clone(),
            name: account.name.clone(),
            country: item.country,
            sex: item.sex,
            dateOfBirth: item.dateOfBirth,
            placeOfBirth: item.placeOfBirth,
            documentNo: item.documentNo,
            expiryDate: item.expiryDate,
        }).collect();
    }

    vec![]
}

#[derive(Serialize, Deserialize)]
struct Signature {
    signature: String,
    public_key: String
}

const KEY_GEN_IKM: &[u8; 49] = b"PKGVYkAHJrc95mGeRwLLVN7JxmHwEPwxEnvhyB7UCQsE4k9Py";
const SIGNATURE_KEY_INFO: &[u8; 15] = b"wallet-key-info";
const SIGNATURE_HEADER: &[u8; 13] = b"wallet-header";
const SIGNATURE_PRESENTATION_HEADER: &[u8; 26] = b"wallet-presentation-header";

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

fn verify_messages(public_key: &[u8; 96], signature: &[u8; 80], messages: &[&[u8]]) -> Result<bool, String> {
    verify(&BbsVerifyRequest {
        public_key,
        header: Some(SIGNATURE_HEADER.as_ref()),
        messages: Some(messages),
        signature
    }).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_signature(messagesArray: Vec<String>) -> Result<Signature, String> {
    let messages_bytes: Vec<Vec<u8>> = messagesArray.into_iter().map(|s| s.into_bytes()).collect();
    let messages: Vec<&[u8]> = messages_bytes.iter().map(|m| m.as_slice()).collect();

    let (secret_key, public_key) = generate_keypair()?;

    let signature = sign_messages(&secret_key, &public_key, &messages)?;

    Ok(Signature {
        signature: hex::encode(signature),
        public_key: hex::encode(public_key)
    })
}

#[tauri::command]
fn verify_signature(signatureHex: String, publicKeyHex: String, messagesArray: Vec<String>) -> bool {
    let public_key: [u8; 96] = match hex::decode(publicKeyHex).ok().and_then(|v| v.try_into().ok()){
        Some(pk) => pk,
        None => return false
    };

    let signature: [u8; 80] = match hex::decode(signatureHex).ok().and_then(|v| v.try_into().ok()) {
        Some(sig) => sig,
        None => return false
    };

    let messages_bytes: Vec<Vec<u8>> = messagesArray.into_iter().map(|s| s.into_bytes()).collect();
    let messages: Vec<&[u8]> = messages_bytes.iter().map(|m| m.as_slice()).collect();

    verify_messages(&public_key, &signature, &messages).unwrap_or(false)
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

#[tauri::command]
fn verify_signature_indices(signatureHex: String, publicKeyHex: String, messagesArray: Vec<String>, indicesArray: Vec<usize>) -> PartialSignatureResult {

    let public_key: [u8; 96] = match hex::decode(publicKeyHex).ok().and_then(|v| v.try_into().ok()) {
        Some(pk) => pk,
        None => return PartialSignatureResult { verified: false, disclosed_messages: vec![] }
    };

    let signature: [u8; 80] = match hex::decode(signatureHex).ok().and_then(|v| v.try_into().ok()) {
        Some(sig) => sig,
        None => return PartialSignatureResult { verified: false, disclosed_messages: vec![] }
    };

    let proof_messages: Vec<BbsProofGenRevealMessageRequest<_>> = messagesArray.iter().enumerate().map(|(i, msg)| {
        BbsProofGenRevealMessageRequest {
            reveal: indicesArray.contains(&i),
            value: msg.as_bytes()
        }
    }).collect();

    let mut disclosed_messages: Vec<DisclosedMessage> = Vec::new();

    let disclosed_for_verify: Vec<(usize, &[u8])> = proof_messages.iter().enumerate().filter(|(_, m)| m.reveal).map(|(i, m)| {
        if let Ok(value) = std::str::from_utf8(m.value) {
            disclosed_messages.push(DisclosedMessage {
                index: i,
                value: value.to_string(),
            });
        } (i, m.value)
    }).collect();

    if disclosed_for_verify.is_empty() {
        return PartialSignatureResult {
            verified: false,
            disclosed_messages
        };
    }

    let proof = match proof_gen(&BbsProofGenRequest {
        public_key: &public_key,
        header: Some(SIGNATURE_HEADER.as_ref()),
        messages: Some(&proof_messages),
        signature: &signature,
        presentation_header: Some(SIGNATURE_PRESENTATION_HEADER.as_ref()),
        verify_signature: None
    }) {
        Ok(p) => p,
        Err(_) => return PartialSignatureResult { verified: false, disclosed_messages }
    };

    let verified = match proof_verify(&BbsProofVerifyRequest {
        public_key: &public_key,
        header: Some(SIGNATURE_HEADER.as_ref()),
        presentation_header: Some(SIGNATURE_PRESENTATION_HEADER.as_ref()),
        proof: &proof,
        messages: Some(&disclosed_for_verify)
    }) {
        Ok(result) => result,
        Err(_) => false
    };

    PartialSignatureResult {
        verified,
        disclosed_messages
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    check_data_file().expect("Failed to check or create data file");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            login_account,
            create_account,
            create_card,
            fetch_wallet_data,
            create_signature,
            verify_signature,
            verify_signature_indices
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
