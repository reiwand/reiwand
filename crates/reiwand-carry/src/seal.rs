use crate::{Error, Result};
use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use zeroize::Zeroizing;

const MAGIC: &[u8; 8] = b"REIWANDE";
const HEAD: usize = 8 + 1 + 4 + 4 + 16 + 19;
const SEGMENT: usize = 65_536;
const TAG: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cost {
    pub memory_kib: u32,
    pub passes: u32,
}

impl Default for Cost {
    fn default() -> Self {
        Cost { memory_kib: 65_536, passes: 3 }
    }
}

pub fn is_sealed(data: &[u8]) -> bool {
    data.starts_with(MAGIC)
}

fn derive(password: &str, salt: &[u8], cost: Cost) -> Result<Zeroizing<[u8; 32]>> {
    let odd = || Error::Invalid("the file asks for an unusable key derivation".into());
    if !(8..=1 << 20).contains(&cost.memory_kib) || !(1..=10).contains(&cost.passes) {
        return Err(odd());
    }
    let params = Params::new(cost.memory_kib, cost.passes, 1, Some(32)).map_err(|_| odd())?;
    let mut key = Zeroizing::new([0; 32]);
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
        .hash_password_into(password.as_bytes(), salt, &mut *key)
        .map_err(|_| odd())?;
    Ok(key)
}

fn nonce(prefix: &[u8], index: u32, last: bool) -> XNonce {
    let mut bytes = [0; 24];
    bytes[..19].copy_from_slice(prefix);
    bytes[19..23].copy_from_slice(&index.to_be_bytes());
    bytes[23] = u8::from(last);
    XNonce::from(bytes)
}

fn cipher(key: &[u8; 32]) -> XChaCha20Poly1305 {
    XChaCha20Poly1305::new_from_slice(key).expect("the key has 32 bytes")
}

pub fn seal(plain: &[u8], password: &str, cost: Cost) -> Result<Vec<u8>> {
    let mut salt = [0u8; 16];
    let mut prefix = [0u8; 19];
    getrandom::fill(&mut salt).expect("the system random source failed");
    getrandom::fill(&mut prefix).expect("the system random source failed");
    let key = derive(password, &salt, cost)?;
    let mut out = Vec::with_capacity(HEAD + plain.len() + plain.len() / SEGMENT * TAG + 2 * TAG);
    out.extend_from_slice(MAGIC);
    out.push(1);
    out.extend_from_slice(&cost.memory_kib.to_be_bytes());
    out.extend_from_slice(&cost.passes.to_be_bytes());
    out.extend_from_slice(&salt);
    out.extend_from_slice(&prefix);
    let head = out.clone();
    let cipher = cipher(&key);
    let chunks: Vec<&[u8]> = if plain.is_empty() { vec![&[]] } else { plain.chunks(SEGMENT).collect() };
    let count = chunks.len();
    for (index, chunk) in chunks.into_iter().enumerate() {
        let n = nonce(&prefix, index as u32, index + 1 == count);
        let sealed = cipher.encrypt(&n, Payload { msg: chunk, aad: &head });
        out.extend(sealed.map_err(|_| Error::Invalid("cannot encrypt a segment".into()))?);
    }
    Ok(out)
}

pub fn open(data: &[u8], password: &str) -> Result<Zeroizing<Vec<u8>>> {
    if data.len() < HEAD + TAG {
        return Err(Error::Truncated);
    }
    let num = |at: usize| u32::from_be_bytes(data[at..at + 4].try_into().expect("four bytes"));
    if data[8] != 1 {
        return Err(Error::Future { found: u32::from(data[8]), known: 1 });
    }
    let cost = Cost { memory_kib: num(9), passes: num(13) };
    let (head, body) = data.split_at(HEAD);
    let key = derive(password, &head[17..33], cost)?;
    let cipher = cipher(&key);
    let prefix = &head[33..];
    let mut plain = Zeroizing::new(Vec::with_capacity(body.len()));
    let chunks: Vec<&[u8]> = body.chunks(SEGMENT + TAG).collect();
    let count = chunks.len();
    for (index, chunk) in chunks.into_iter().enumerate() {
        let last = index + 1 == count;
        let try_with = |flag: bool| {
            cipher.decrypt(&nonce(prefix, index as u32, flag), Payload { msg: chunk, aad: head })
        };
        match try_with(last) {
            Ok(part) => plain.extend(part),
            Err(_) if last && try_with(false).is_ok() => return Err(Error::Truncated),
            Err(_) => return Err(Error::WrongPassword),
        }
    }
    Ok(plain)
}
