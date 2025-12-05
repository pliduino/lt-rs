#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum Event {
    None = 0,
    Completed,
    Started,
    Stopped,
    Paused,
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    Unknown,
}

pub struct PieceIndex(i32);

impl PieceIndex {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub fn to_inner(self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum SocketType {
    Tcp = 0,
    Socks5,
    Http,
    Utp,
    I2p,
    TcpSsl,
    Socks5Ssl,
    HttpSsl,
    UtpSsl,
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum ConnectionType {
    BitTorrent = 0,
    UrlSeed,
    HttpSeed,
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum Direction {
    In = 0,
    Out,
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    Unknown,
}

/// These are all the reasons to disconnect a peer
/// all reasons caused by the peer sending unexpected data
/// are 256 and up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u16)]
pub enum CloseReason {
    /// No reason specified. Generic close.
    None = 0,
    /// We're already connected to
    DuplicatePeerId,
    /// This torrent has been removed, paused or stopped from this client.
    TorrentRemoved,
    /// Client failed to allocate necessary memory for this peer connection
    NoMemory,
    /// The source port of this peer is blocked
    PortBlocked,
    /// The source IP has been blocked
    Blocked,
    /// Both ends of the connection are upload-only. staying connected would
    /// be redundant
    UploadToUpload,
    /// Connection was closed because the other end is upload only and does
    /// not have any pieces we're interested in
    NotInterestedUploadOnly,
    /// Peer connection timed out (generic timeout)
    Timeout,
    /// The peers have not been interested in each other for a very long time.
    /// disconnect
    TimedOutInterest,
    /// The peer has not sent any message in a long time.
    TimedOutActivity,
    /// The peer did not complete the handshake in too long
    TimedOutHandshake,
    /// The peer sent an interested message, but did not send a request
    /// after a very long time after being unchoked.
    TimedOutRequest,
    /// The encryption mode is blocked
    ProtocolBlocked,
    /// The peer was disconnected in the hopes of finding a better peer
    /// in the swarm
    PeerChurn,
    /// We have too many peers connected
    TooManyConnections,
    /// We have too many file-descriptors open
    TooManyFiles,

    /// The encryption handshake failed
    EncryptionError = 256,
    /// The info hash sent as part of the handshake was not what we expected
    InvalidInfoHash,
    SelfConnection,
    /// The metadata received matched the info-hash, but failed to parse.
    /// this is either someone finding a SHA1 collision, or the author of
    /// the magnet link creating it from an invalid torrent
    InvalidMetadata,
    /// The advertised metadata size
    MetadataTooBig,

    /// Invalid bittorrent messages
    MessageTooBig,
    InvalidMessageId,
    InvalidMessage,
    InvalidPieceMessage,
    InvalidHaveMessage,
    InvalidBitfieldMessage,
    InvalidChokeMessage,
    InvalidUnchokeMessage,
    InvalidInterestedMessage,
    InvalidNotInterestedMessage,
    InvalidRequestMessage,
    InvalidRejectMessage,
    InvalidAllowFastMessage,
    InvalidExtendedMessage,
    InvalidCancelMessage,
    InvalidDhtPortMessage,
    InvalidSuggestMessage,
    InvalidHaveAllMessage,
    InvalidDontHaveMessage,
    InvalidHaveNoneMessage,
    InvalidPexMessage,
    InvalidMetadataRequestMessage,
    InvalidMetadataMessage,
    InvalidMetadataOffset,

    /// The peer sent a request while being choked
    RequestWhenChoked,

    /// The peer sent corrupt data
    CorruptPieces,

    PexMessageTooBig,
    PexTooFrequent,

    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    Unknown,
}

impl CloseReason {
    pub(crate) fn from_u16(value: u16) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                value.into()
            } else {
                unsafe { std::mem::transmute(value) }
            }
        }
    }
}

impl Direction {
    pub(crate) fn from_u8(value: u8) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                value.into()
            } else {
                unsafe { std::mem::transmute(value) }
            }
        }
    }
}

impl ConnectionType {
    pub(crate) fn from_u8(value: u8) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                value.into()
            } else {
                unsafe { std::mem::transmute(value) }
            }
        }
    }
}

impl SocketType {
    pub(crate) fn from_u8(value: u8) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                value.into()
            } else {
                unsafe { std::mem::transmute(value) }
            }
        }
    }
}

impl Event {
    pub(crate) fn from_u8(value: u8) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                value.into()
            } else {
                unsafe { std::mem::transmute(value) }
            }
        }
    }
}
