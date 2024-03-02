use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use bytes::Bytes;
use get_if_addrs::get_if_addrs;
use hkdf::Hkdf;
use num_bigint::{BigUint, ToBigInt};
use p256::{PublicKey, SecretKey};
use rand::{thread_rng, Rng, RngCore};
use serde::{Deserialize, Serialize};
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

impl RemoteDeviceInfo {
    pub fn serialize(&self) -> Vec<u8> {
        // 1 byte: Version(3 bits)|Visibility(1 bit)|Device Type(3 bits)|Reserved(1 bit)
        let mut endpoint_info: Vec<u8> = vec![((self.device_type.clone() as u8) << 1) & 0b111];

        // 16 bytes: unknown random bytes
        endpoint_info.extend((0..16).map(|_| rand::thread_rng().gen_range(0..=255)));

        // Device name in UTF-8 prefixed with 1-byte length
        let mut name_chars = self.name.as_bytes().to_vec();
        if name_chars.len() > 255 {
            name_chars.truncate(255);
        }
        endpoint_info.push(name_chars.len() as u8);
        endpoint_info.extend(name_chars);

        endpoint_info
    }
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

pub fn parse_mdns_endpoint_info(encoded_str: &str) -> Result<(DeviceType, String), anyhow::Error> {
    let decoded_bytes = URL_SAFE_NO_PAD.decode(encoded_str)?;
    if decoded_bytes.len() < 19 {
        return Err(anyhow!("Invalid data length"));
    }

    let device_type_byte = decoded_bytes[0];
    let device_type = (device_type_byte & 0b0111) >> 4;
    let name_length = decoded_bytes[17] as usize;
    if 18 + name_length > decoded_bytes.len() {
        return Err(anyhow!("Invalid name length"));
    }

    let device_name_bytes = &decoded_bytes[18..18 + name_length];
    let device_name = String::from_utf8(device_name_bytes.to_vec())?;

    Ok((DeviceType::from_raw_value(device_type), device_name))
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
    let secret_key = SecretKey::random(&mut thread_rng());
    let public_key = secret_key.public_key();

    (secret_key, public_key)
}

pub fn encode_point(unsigned: Bytes) -> Result<Vec<u8>, anyhow::Error> {
    let big_int = BigUint::from_bytes_be(&unsigned)
        .to_bigint()
        .ok_or_else(|| anyhow!("Failed to convert to bigint"))?;

    Ok(big_int.to_signed_bytes_be())
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

pub fn is_not_self_ip(ip_address: &Ipv4Addr) -> bool {
    if let Ok(if_addrs) = get_if_addrs() {
        for if_addr in if_addrs {
            if if_addr.ip() == *ip_address {
                return false;
            }
        }
    }

    true
}
