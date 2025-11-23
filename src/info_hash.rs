use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};

use crate::ffi::ffi;

#[derive(Debug, Clone, Copy)]
pub enum InfoHash {
    V1([u8; 20]),
    V2([u8; 32]),
}

impl InfoHash {
    pub fn as_base64(&self) -> String {
        match self {
            InfoHash::V1(v1) => BASE64_URL_SAFE_NO_PAD.encode(v1),
            InfoHash::V2(v2) => BASE64_URL_SAFE_NO_PAD.encode(v2),
        }
    }

    // Convert hex string to byte array
    fn from_hex<const N: usize>(hex: &str) -> Result<[u8; N], ()> {
        if hex.len() != N * 2 {
            return Err(());
        }
        let mut bytes = [0u8; N];
        for i in 0..N {
            let hi = hex_char_to_val(hex.as_bytes()[i * 2])?;
            let lo = hex_char_to_val(hex.as_bytes()[i * 2 + 1])?;
            bytes[i] = (hi << 4) | lo;
        }
        Ok(bytes)
    }

    // Decode base32 (RFC4648, no padding)
    fn from_base32(input: &str) -> Result<[u8; 20], ()> {
        let mut bits = 0u64;
        let mut bit_count = 0;
        let mut out = [0u8; 20];
        let mut byte_index = 0;

        for c in input.chars() {
            let val = base32_char_to_val(c)?;
            bits = (bits << 5) | (val as u64);
            bit_count += 5;

            while bit_count >= 8 {
                bit_count -= 8;
                if byte_index >= 20 {
                    return Err(()); // too long
                }
                out[byte_index] = ((bits >> bit_count) & 0xFF) as u8;
                byte_index += 1;
            }
        }

        if byte_index != 20 {
            return Err(()); // too short
        }

        Ok(out)
    }

    pub fn from_magnet(magnet_uri: &str) -> Result<Self, ()> {
        let xt_start = magnet_uri.find("xt=urn:").ok_or(())?;
        let xt = &magnet_uri[xt_start + 7..];
        let end = xt.find('&').unwrap_or(xt.len());
        let urn = &xt[..end];

        if let Some(rest) = urn.strip_prefix("btih:") {
            if rest.len() == 40 {
                let bytes = Self::from_hex::<20>(rest)?;
                return Ok(InfoHash::V1(bytes));
            } else if rest.len() == 32 {
                let bytes = Self::from_base32(rest)?;
                return Ok(InfoHash::V1(bytes));
            } else {
                return Err(());
            }
        } else if let Some(rest) = urn.strip_prefix("btmh:") {
            if let Some(rest) = rest.strip_prefix("1220") {
                let bytes = Self::from_hex::<32>(rest)?;
                return Ok(InfoHash::V2(bytes));
            } else {
                return Err(());
            }
        }

        Err(())
    }
}

// Convert single hex char to value
fn hex_char_to_val(c: u8) -> Result<u8, ()> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(()),
    }
}

// Convert single base32 char to value
fn base32_char_to_val(c: char) -> Result<u8, ()> {
    match c {
        'A'..='Z' => Ok((c as u8) - b'A'),
        '2'..='7' => Ok((c as u8) - b'2' + 26),
        _ => Err(()),
    }
}

impl std::hash::Hash for InfoHash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            InfoHash::V1(v1) => v1.hash(state),
            InfoHash::V2(v2) => v2.hash(state),
        }
    }
}

impl PartialEq for InfoHash {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InfoHash::V1(lhs), InfoHash::V1(rhs)) => lhs == rhs,
            (InfoHash::V2(lhs), InfoHash::V2(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl Eq for InfoHash {}

impl From<ffi::InfoHashCpp> for InfoHash {
    fn from(value: ffi::InfoHashCpp) -> Self {
        match value.version {
            1 => InfoHash::V1(value.inner[..20].try_into().unwrap()),
            2 => InfoHash::V2(value.inner),
            _ => unreachable!(),
        }
    }
}
