/// BitTorrent version enumerator
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum ProtocolVersion {
    /// The original BitTorrent version, using SHA-1 hashes
    V1,
    /// Version 2 of the BitTorrent protocol, using SHA-256 hashes
    V2,
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    Unknown,
}

impl ProtocolVersion {
    pub(crate) fn from_u8(v: u8) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                v.into()
            } else {
                unsafe { std::mem::transmute(v) }
            }
        }
    }
}
