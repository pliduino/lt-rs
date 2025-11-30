use num_enum::FromPrimitive;

/// BitTorrent version enumerator
#[derive(FromPrimitive)]
#[repr(u8)]
pub enum ProtocolVersion {
    /// The original BitTorrent version, using SHA-1 hashes
    V1,
    /// Version 2 of the BitTorrent protocol, using SHA-256 hashes
    V2,
    #[num_enum(default)]
    Unknown,
}
