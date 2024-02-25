use std::path::{Path, PathBuf};

use anyhow::anyhow;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use hkdf::Hkdf;
use p256::{PublicKey, SecretKey};
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::digest::generic_array::GenericArray;
use sha2::Sha256;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[allow(dead_code)]
pub enum DeviceType {
    Unknown = 0,
    Phone = 1,
    Tablet = 2,
    Laptop = 3,
}

#[allow(dead_code)]
impl DeviceType {
    pub fn from_raw_value(value: u8) -> Self {
        match value {
            0 => DeviceType::Unknown,
            1 => DeviceType::Phone,
            2 => DeviceType::Tablet,
            3 => DeviceType::Laptop,
            _ => DeviceType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct RemoteDeviceInfo {
    pub name: String,
    pub device_type: DeviceType,
}

pub fn gen_mdns_name(endpoint_id: [u8; 4]) -> String {
    let mut name_b = Vec::new();

    let pcp: [u8; 1] = [0x23];
    name_b.extend_from_slice(&pcp);

    name_b.extend_from_slice(&endpoint_id);

    let service_id: [u8; 3] = [0xFC, 0x9F, 0x5E];
    name_b.extend_from_slice(&service_id);

    let unknown_bytes: [u8; 2] = [0x00, 0x00];
    name_b.extend_from_slice(&unknown_bytes);

    URL_SAFE_NO_PAD.encode(&name_b)
}

pub fn gen_mdns_endpoint_info(device_type: u8, device_name: &str) -> String {
    let mut record = Vec::new();

    // 1 byte: Version(3 bits)|Visibility(1 bit)|Device Type(3 bits)|Reserved(1 bits)
    // Device types: unknown=0, phone=1, tablet=2, laptop=3
    record.push(device_type << 1);

    let unknown_bytes = rand::thread_rng().gen::<[u8; 16]>();
    record.extend_from_slice(&unknown_bytes);

    let device_name = device_name.as_bytes();
    let length = device_name.len() as u8;
    record.push(length);
    record.extend_from_slice(device_name);

    URL_SAFE_NO_PAD.encode(&record)
}

pub async fn stream_read_exact(
    socket: &mut TcpStream,
    buf: &mut [u8],
) -> Result<(), anyhow::Error> {
    match socket.read_exact(buf).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub fn gen_ecdsa_keypair() -> (SecretKey, PublicKey) {
    // TODO - not sure why, but when using a random generator, 90% of the keys
    // generated won't works with android (the error doesn't make sense as it's u8):
    // Cannot parse public key: Point encoding must use only non-negative integers
    // So for now, use a hardcoded key that for some reason works.
    let ga = GenericArray::from_slice(&[
        105, 205, 243, 134, 222, 182, 205, 89, 155, 24, 188, 47, 119, 109, 222, 245, 25, 52, 8,
        195, 162, 68, 9, 241, 138, 225, 80, 106, 111, 224, 254, 32,
    ]);
    let secret_key = SecretKey::from_bytes(ga).unwrap();
    let public_key = secret_key.public_key();

    (secret_key, public_key)
}

pub fn hkdf_extract_expand(
    salt: &[u8],
    input: &[u8],
    info: &[u8],
    output_len: usize,
) -> Result<Vec<u8>, anyhow::Error> {
    let hkdf = Hkdf::<Sha256>::new(Some(salt), input);
    let mut okm = vec![0u8; output_len];
    hkdf.expand(info, &mut okm)
        .map_err(|e| anyhow!("HKDF expand failed: {}", e))?;
    Ok(okm)
}

pub fn to_four_digit_string(bytes: &Vec<u8>) -> String {
    let k_hash_modulo = 9973;
    let k_hash_base_multiplier = 31;

    let mut hash = 0;
    let mut multiplier = 1;
    for &byte in bytes {
        // Cast u8 to i8 (assuming that's what's intended by static_cast<int8_t>(byte))
        // Rust by default treats i8 as signed, matching the C++ behavior here
        let byte = byte as i8 as i32; // `as i32` to perform calculation correctly
        hash = (hash + byte * multiplier) % k_hash_modulo;
        multiplier = (multiplier * k_hash_base_multiplier) % k_hash_modulo;
    }

    format!("{:04}", hash.abs())
}

pub fn gen_random(size: usize) -> Vec<u8> {
    let mut data = vec![0; size];
    rand::thread_rng().fill_bytes(&mut data);

    data
}

pub fn get_download_dir() -> PathBuf {
    if let Some(user_dirs) = directories::UserDirs::new() {
        if let Some(dd) = user_dirs.download_dir() {
            return dd.to_path_buf();
        }

        return user_dirs.home_dir().to_path_buf();
    }

    Path::new("/").to_path_buf()
}
