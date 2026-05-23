use crate::ffi::ffi::PeerInfoSnapshot as FfiPeerInfoSnapshot;

/// Snapshot of a connected peer's state, returned via `peer_info_alert`
/// after calling `TorrentHandle::post_peer_info()`.
#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub ip:             String,
    pub port:           u16,
    /// BitTorrent client user-agent string.
    pub client:         String,
    /// Bytes/sec download rate from this peer.
    pub down_speed:     i32,
    /// Bytes/sec upload rate to this peer.
    pub up_speed:       i32,
    pub total_download: i64,
    pub total_upload:   i64,
    /// 0.0–1.0 fraction of torrent this peer has.
    pub progress:       f32,
    /// Bitfield: `1`=seed, `2`=local, `4`=interesting, `8`=choked,
    /// `16`=remote_interested, `32`=remote_choked, `64`=supports_extensions, `128`=snubbed
    pub flags:          u32,
    /// Bitfield: `1`=tracker, `2`=dht, `4`=pex, `8`=lsd, `16`=resume_data
    pub source:         u32,
    /// 2-character ISO 3166 country code (may be empty if not available).
    pub country:        String,
}

impl PeerInfo {
    pub(crate) fn from_ffi(s: FfiPeerInfoSnapshot) -> Self {
        PeerInfo {
            ip:             s.ip,
            port:           s.port,
            client:         s.client,
            down_speed:     s.down_speed,
            up_speed:       s.up_speed,
            total_download: s.total_download,
            total_upload:   s.total_upload,
            progress:       s.progress,
            flags:          s.flags,
            source:         s.source,
            country:        s.country,
        }
    }

    pub fn is_seed(&self)       -> bool { self.flags & 1 != 0 }
    pub fn is_local(&self)      -> bool { self.flags & 2 != 0 }
    pub fn is_interesting(&self)-> bool { self.flags & 4 != 0 }
    pub fn is_choked(&self)     -> bool { self.flags & 8 != 0 }
    pub fn from_tracker(&self)  -> bool { self.source & 1 != 0 }
    pub fn from_dht(&self)      -> bool { self.source & 2 != 0 }
    pub fn from_pex(&self)      -> bool { self.source & 4 != 0 }
}
