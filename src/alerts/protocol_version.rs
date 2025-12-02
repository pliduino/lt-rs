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
