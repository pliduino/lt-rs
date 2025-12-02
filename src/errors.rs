use crate::ffi::error::ffi::{self};

#[derive(Debug)]
pub enum LtrsError {
    LibtorrentError(LibtorrentError),
    HttpError(HttpError),
    GzipError(GzipError),
    I2pError(I2pError),
    PcpError(PcpError),
    BdecodeError(BdecodeError),
    SocksError(SocksError),
    UpnpError(UpnpError),
    // If error category is not known
    // This is enable even without safe_enums feature because we need to check
    // libtorrent to see if we already covered all possible variants
    Unknown(i32),
}

impl LtrsError {
    pub fn is_ok(&self) -> bool {
        match self {
            LtrsError::LibtorrentError(e) => {
                matches!(e, LibtorrentError::NoError)
            }
            LtrsError::HttpError(e) => {
                matches!(e, HttpError::Ok)
            }
            LtrsError::GzipError(e) => {
                matches!(e, GzipError::NoError)
            }
            LtrsError::I2pError(e) => {
                matches!(e, I2pError::NoError)
            }
            LtrsError::PcpError(e) => {
                matches!(e, PcpError::Success)
            }
            LtrsError::BdecodeError(e) => {
                matches!(e, BdecodeError::NoError)
            }
            LtrsError::SocksError(e) => {
                matches!(e, SocksError::NoError)
            }
            LtrsError::UpnpError(e) => {
                matches!(e, UpnpError::NoError)
            }
            LtrsError::Unknown(e) => *e == 0,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum LibtorrentError {
    /// Not an error
    NoError = 0,
    /// Two torrents has files which end up overwriting each other
    FileCollision,
    /// A piece did not match its piece hash
    FailedHashCheck,
    /// The .torrent file does not contain a bencoded dictionary at
    /// its top level
    TorrentIsNoDict,
    /// The .torrent file does not have an ``info`` dictionary
    TorrentMissingInfo,
    /// The .torrent file's ``info`` entry is not a dictionary
    TorrentInfoNoDict,
    /// The .torrent file does not have a ``piece length`` entry
    TorrentMissingPieceLength,
    /// The .torrent file does not have a ``name`` entry
    TorrentMissingName,
    /// The .torrent file's name entry is invalid
    TorrentInvalidName,
    /// The length of a file, or of the whole .torrent file is invalid.
    /// Either negative or not an integer
    TorrentInvalidLength,
    /// Failed to parse a file entry in the .torrent
    TorrentFileParseFailed,
    /// The ``pieces`` field is missing or invalid in the .torrent file
    TorrentMissingPieces,
    /// The ``pieces`` string has incorrect length
    TorrentInvalidHashes,
    /// The .torrent file has more pieces than is supported by libtorrent
    TooManyPiecesInTorrent,
    /// The metadata (.torrent file) that was received from the swarm
    /// matched the info-hash, but failed to be parsed
    InvalidSwarmMetadata,
    /// The file or buffer is not correctly bencoded
    InvalidBencoding,
    /// The .torrent file does not contain any files
    NoFilesInTorrent,
    /// The string was not properly url-encoded as expected
    InvalidEscapedString,
    /// Operation is not permitted since the session is shutting down
    SessionIsClosing,
    /// There's already a torrent with that info-hash added to the
    /// session
    DuplicateTorrent,
    /// The supplied torrent_handle is not referring to a valid torrent
    InvalidTorrentHandle,
    /// The type requested from the entry did not match its type
    InvalidEntryType,
    /// The specified URI does not contain a valid info-hash
    MissingInfoHashInUri,
    /// One of the files in the torrent was unexpectedly small. This
    /// might be caused by files being changed by an external process
    FileTooShort,
    /// The URL used an unknown protocol. Currently ``http`` and
    /// ``https`` (if built with openssl support) are recognized. For
    /// trackers ``udp`` is recognized as well.
    UnsupportedUrlProtocol,
    /// The URL did not conform to URL syntax and failed to be parsed
    UrlParseError,
    /// The peer sent a piece message of length 0
    PeerSentEmptyPiece,
    /// A bencoded structure was corrupt and failed to be parsed
    ParseFailed,
    /// The fast resume file was missing or had an invalid file version
    /// tag
    InvalidFileTag,
    /// The fast resume file was missing or had an invalid info-hash
    MissingInfoHash,
    /// The info-hash did not match the torrent
    MismatchingInfoHash,
    /// The URL contained an invalid hostname
    InvalidHostname,
    /// The URL had an invalid port
    InvalidPort,
    /// The port is blocked by the port-filter, and prevented the
    /// connection
    PortBlocked,
    /// The IPv6 address was expected to end with "]"
    ExpectedCloseBracketInAddress,
    /// The torrent is being destructed, preventing the operation to
    /// succeed
    DestructingTorrent,
    /// The connection timed out
    TimedOut,
    /// The peer is upload only, and we are upload only. There's no point
    /// in keeping the connection
    UploadUploadConnection,
    /// The peer is upload only, and we're not interested in it. There's
    /// no point in keeping the connection
    UninterestingUploadPeer,
    /// The peer sent an unknown info-hash
    InvalidInfoHash,
    /// The torrent is paused, preventing the operation from succeeding
    TorrentPaused,
    /// The peer sent an invalid have message, either wrong size or
    /// referring to a piece that doesn't exist in the torrent
    InvalidHave,
    /// The bitfield message had the incorrect size
    InvalidBitfieldSize,
    /// The peer kept requesting pieces after it was choked, possible
    /// abuse attempt.
    TooManyRequestsWhenChoked,
    /// The peer sent a piece message that does not correspond to a
    /// piece request sent by the client
    InvalidPiece,
    /// memory allocation failed
    NoMemory,
    /// The torrent is aborted, preventing the operation to succeed
    TorrentAborted,
    /// The peer is a connection to ourself, no point in keeping it
    SelfConnection,
    /// The peer sent a piece message with invalid size, either negative
    /// or greater than one block
    InvalidPieceSize,
    /// The peer has not been interesting or interested in us for too
    /// long, no point in keeping it around
    TimedOutNoInterest,
    /// The peer has not said anything in a long time, possibly dead
    TimedOutInactivity,
    /// The peer did not send a handshake within a reasonable amount of
    /// time, it might not be a bittorrent peer
    TimedOutNoHandshake,
    /// The peer has been unchoked for too long without requesting any
    /// data. It might be lying about its interest in us
    TimedOutNoRequest,
    /// The peer sent an invalid choke message
    InvalidChoke,
    /// The peer send an invalid unchoke message
    InvalidUnchoke,
    /// The peer sent an invalid interested message
    InvalidInterested,
    /// The peer sent an invalid not-interested message
    InvalidNotInterested,
    /// The peer sent an invalid piece request message
    InvalidRequest,
    /// The peer sent an invalid hash-list message (this is part of the
    /// merkle-torrent extension)
    InvalidHashList,
    /// The peer sent an invalid hash-piece message (this is part of the
    /// merkle-torrent extension)
    InvalidHashPiece,
    /// The peer sent an invalid cancel message
    InvalidCancel,
    /// The peer sent an invalid DHT port-message
    InvalidDhtPort,
    /// The peer sent an invalid suggest piece-message
    InvalidSuggest,
    /// The peer sent an invalid have all-message
    InvalidHaveAll,
    /// The peer sent an invalid have none-message
    InvalidHaveNone,
    /// The peer sent an invalid reject message
    InvalidReject,
    /// The peer sent an invalid allow fast-message
    InvalidAllowFast,
    /// The peer sent an invalid extension message ID
    InvalidExtended,
    /// The peer sent an invalid message ID
    InvalidMessage,
    /// The synchronization hash was not found in the encrypted handshake
    SyncHashNotFound,
    /// The encryption constant in the handshake is invalid
    InvalidEncryptionConstant,
    /// The peer does not support plain text, which is the selected mode
    NoPlaintextMode,
    /// The peer does not support RC4, which is the selected mode
    NoRc4Mode,
    /// The peer does not support any of the encryption modes that the
    /// client supports
    UnsupportedEncryptionMode,
    /// The peer selected an encryption mode that the client did not
    /// advertise and does not support
    UnsupportedEncryptionModeSelected,
    /// The pad size used in the encryption handshake is of invalid size
    InvalidPadSize,
    /// The encryption handshake is invalid
    InvalidEncryptHandshake,
    /// The client is set to not support incoming encrypted connections
    /// and this is an encrypted connection
    NoIncomingEncrypted,
    /// The client is set to not support incoming regular bittorrent
    /// connections, and this is a regular connection
    NoIncomingRegular,
    /// The client is already connected to this peer-ID
    DuplicatePeerId,
    /// Torrent was removed
    TorrentRemoved,
    /// The packet size exceeded the upper sanity check-limit
    PacketTooLarge,

    Reserved,

    /// The web server responded with an error
    HttpError,
    /// The web server response is missing a location header
    MissingLocation,
    /// The web seed redirected to a path that no longer matches the
    /// .torrent directory structure
    InvalidRedirection,
    /// The connection was closed because it redirected to a different
    /// URL
    Redirecting,
    /// The HTTP range header is invalid
    InvalidRangeLtrsError,
    /// The HTTP response did not have a content length
    NoContentLength,
    /// The IP is blocked by the IP filter
    BannedByIpFilter,
    /// At the connection limit
    TooManyConnections,
    /// The peer is marked as banned
    PeerBanned,
    /// The torrent is stopping, causing the operation to fail
    StoppingTorrent,
    /// The peer has sent too many corrupt pieces and is banned
    TooManyCorruptPieces,
    /// The torrent is not ready to receive peers
    TorrentNotReady,
    /// The peer is not completely constructed yet
    PeerNotConstructed,
    /// The session is closing, causing the operation to fail
    SessionClosing,
    /// The peer was disconnected in order to leave room for a
    /// potentially better peer
    OptimisticDisconnect,
    /// The torrent is finished
    TorrentFinished,
    /// No UPnP router found
    NoRouter,
    /// The metadata message says the metadata exceeds the limit
    MetadataTooLarge,
    /// The peer sent an invalid metadata request message
    InvalidMetadataRequest,
    /// The peer advertised an invalid metadata size
    InvalidMetadataSize,
    /// The peer sent a message with an invalid metadata offset
    InvalidMetadataOffset,
    /// The peer sent an invalid metadata message
    InvalidMetadataMessage,
    /// The peer sent a peer exchange message that was too large
    PexMessageTooLarge,
    /// The peer sent an invalid peer exchange message
    InvalidPexMessage,
    /// The peer sent an invalid tracker exchange message
    InvalidLtTrackerMessage,
    /// The peer sent pex messages too often. This is a possible
    /// attempt of and attack
    TooFrequentPex,
    /// The operation failed because it requires the torrent to have
    /// the metadata (.torrent file) and it doesn't have it yet.
    /// This happens for magnet links before they have downloaded the
    /// metadata, and also torrents added by URL.
    NoMetadata,
    /// The peer sent an invalid ``dont_have`` message. The don't have
    /// message is an extension to allow peers to advertise that the
    /// no longer has a piece they previously had.
    InvalidDontHave,
    /// The peer tried to connect to an SSL torrent without connecting
    /// over SSL.
    RequiresSslConnection,
    /// The peer tried to connect to a torrent with a certificate
    /// for a different torrent.
    InvalidSslCert,
    /// the torrent is not an SSL torrent, and the operation requires
    /// an SSL torrent
    NotAnSslTorrent,
    /// peer was banned because its listen port is within a banned port
    /// range, as specified by the port_filter.
    BannedByPortFilter,
    /// The session_handle is not referring to a valid session_impl
    InvalidSessionHandle,
    /// the listen socket associated with this request was closed
    InvalidListenSocket,
    InvalidHashRequest,
    InvalidHashes,
    InvalidHashReject,

    /// these error codes are deprecated, NAT-PMP/PCP error codes have
    /// been moved to their own category

    // /// The NAT-PMP router responded with an unsupported protocol version
    // #[deprecated]
    // UnsupportedProtocolVersion = 120,
    // /// You are not authorized to map ports on this NAT-PMP router
    // #[deprecated]
    // NatpmpNotAuthorized,
    // /// The NAT-PMP router failed because of a network failure
    // #[deprecated]
    // NetworkFailure,
    // /// The NAT-PMP router failed because of lack of resources
    // #[deprecated]
    // NoResources,
    // /// The NAT-PMP router failed because an unsupported opcode was sent
    // #[deprecated]
    // UnsupportedOpcode,

    /// The resume data file is missing the ``file sizes`` entry
    MissingFileSizes = 130,
    /// The resume data file ``file sizes`` entry is empty
    NoFilesInResumeData,
    /// The resume data file is missing the ``pieces`` and ``slots`` entry
    MissingPieces,
    /// The number of files in the resume data does not match the number
    /// of files in the torrent
    MismatchingNumberOfFiles,
    /// One of the files on disk has a different size than in the fast
    /// resume file
    MismatchingFileSize,
    /// One of the files on disk has a different timestamp than in the
    /// fast resume file
    MismatchingFileTimestamp,
    /// The resume data file is not a dictionary
    NotADictionary,
    /// The ``blocks per piece`` entry is invalid in the resume data file
    InvalidBlocksPerPiece,
    /// The resume file is missing the ``slots`` entry, which is required
    /// for torrents with compact allocation. *DEPRECATED*
    MissingSlots,
    /// The resume file contains more slots than the torrent
    TooManySlots,
    /// The ``slot`` entry is invalid in the resume data
    InvalidSlotList,
    /// One index in the ``slot`` list is invalid
    InvalidPieceIndex,
    /// The pieces on disk needs to be re-ordered for the specified
    /// allocation mode. This happens if you specify sparse allocation
    /// and the files on disk are using compact storage. The pieces needs
    /// to be moved to their right position. *DEPRECATED*
    PiecesNeedReorder,
    /// this error is returned when asking to save resume data and
    /// specifying the flag to only save when there's anything new to save
    /// (torrent_handle::only_if_modified) and there wasn't anything changed.
    ResumeDataNotModified,
    /// the save_path in add_torrent_params is not valid
    InvalidSavePath,

    /// The HTTP header was not correctly formatted
    HttpParseError = 150,
    /// The HTTP response was in the 300-399 range but lacked a location
    /// header
    HttpMissingLocation,
    /// The HTTP response was encoded with gzip or deflate but
    /// decompressing it failed
    HttpFailedDecompress,

    /// The URL specified an i2p address, but no i2p router is configured
    NoI2pRouter = 160,
    /// i2p acceptor is not available yet, can't announce without endpoint
    NoI2pEndpoint,

    /// The tracker URL doesn't support transforming it into a scrape
    /// URL. i.e. it doesn't contain "announce.
    ScrapeNotAvailable = 170,
    /// invalid tracker response
    InvalidTrackerResponse,
    /// invalid peer dictionary entry. Not a dictionary
    InvalidPeerDict,
    /// tracker sent a failure message
    TrackerFailure,
    /// missing or invalid ``files`` entry
    InvalidFilesEntry,
    /// missing or invalid ``hash`` entry
    InvalidHashEntry,
    /// missing or invalid ``peers`` and ``peers6`` entry
    InvalidPeersEntry,
    /// UDP tracker response packet has invalid size
    InvalidTrackerResponseLength,
    /// invalid transaction id in UDP tracker response
    InvalidTrackerTransactionId,
    /// invalid action field in UDP tracker response
    InvalidTrackerAction,
    /// skipped announce (because it's assumed to be unreachable over the
    /// given source network interface)
    AnnounceSkipped,

    /// expected string in bencoded string
    // #[deprecated]
    // ExpectedString190,
    // /// expected colon in bencoded string
    // #[deprecated]
    // ExpectedColon,
    // /// unexpected end of file in bencoded string
    // #[deprecated]
    // UnexpectedEof,
    // /// expected value (list, dict, int or string) in bencoded string
    // #[deprecated]
    // ExpectedValue,
    // /// bencoded recursion depth limit exceeded
    // #[deprecated]
    // DepthExceeded,
    // /// bencoded item count limit exceeded
    // #[deprecated]
    // LimitExceeded,
    // /// integer overflow
    // #[deprecated]
    // Overflow,

    /// random number generation failed
    NoEntropy = 200,
    /// blocked by SSRF mitigation
    SsrvMitigation,
    /// blocked because IDNA host names are banned
    BlockedByIdna,

    /// the torrent file has an unknown meta version
    TorrentUnknownVersion = 210,
    /// the v2 torrent file has no file tree
    TorrentMissingFileTree,
    /// the torrent contains v2 keys but does not specify meta version 2
    TorrentMissingMetaVersion,
    /// the v1 and v2 file metadata does not match
    TorrentInconsistentFiles,
    /// one or more files are missing piece layer hashes
    TorrentMissingPieceLayer,
    /// a piece layer has the wrong size or failed hash check
    TorrentInvalidPieceLayer,
    /// a v2 file entry has no root hash
    TorrentMissingPiecesRoot,
    /// the v1 and v2 hashes do not describe the same data
    TorrentInconsistentHashes,
    /// a file in the v2 metadata has the pad attribute set
    TorrentInvalidPadFile,
    /// The value is not used by libtorrent itself, it is only used
    /// internally to match againt C++ integers and prevent crashes
    /// as otherwise we would need a unreachable!() in the match arms.
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    UnknownErrorCode,
}

#[derive(Debug)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u16)]
pub enum HttpError {
    Cont = 100,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    MultipleChoices = 300,
    MovedPermanently = 301,
    MovedTemporarily = 302,
    NotModified = 304,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    UnknownErrorCode,
}

#[derive(Debug)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum I2pError {
    NoError = 0,
    ParseFailed,
    CantReachPeer,
    I2pError,
    InvalidKey,
    InvalidId,
    Timeout,
    KeyNotFound,
    DuplicatedId,
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    UnknownErrorCode,
}

#[derive(Debug)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum GzipError {
    NoError = 0,
    InvalidGzipHeader,
    InflatedDataTooLarge,
    DataDidNotTerminate,
    SpaceExhausted,
    InvalidBlockType,
    InvalidStoredBlockLength,
    TooManyLengthOrDistanceCodes,
    CodeLengthsCodesIncomplete,
    RepeatLengthsWithNoFirstLength,
    RepeatMoreThanSpecifiedLengths,
    InvalidLiteralLengthCodeLengths,
    InvalidDistanceCodeLengths,
    InvalidLiteralCodeInBlock,
    DistanceTooFarBackInBlock,
    UnknownGzipError,
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    UnknownErrorCode,
}

#[derive(Debug)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum PcpError {
    Success = 0,
    UnsupportedVersion,
    NotAuthorized,
    MalformedRequest,
    UnsupportedOpcode,
    UnsupportedOption,
    MalformedOption,
    NetworkFailure,
    NoResources,
    UnsupportedProtocol,
    UserExQuota,
    CannotProvideExternal,
    AddressMismatch,
    ExcessiveRemotePeers,
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    UnknownErrorCode,
}

#[derive(Debug)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum BdecodeError {
    NoError = 0,
    ExpectedDigit,
    ExpectedColon,
    UnexpectedEof,
    ExpectedValue,
    DepthExceeded,
    LimitExceeded,
    Overflow,
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    UnknownErrorCode,
}

#[derive(Debug)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum SocksError {
    NoError = 0,
    UnsupportedVersion,
    UnsupportedAuthenticationMethod,
    UnsupportedAuthenticationVersion,
    AuthenticationError,
    UsernameRequired,
    GeneralFailure,
    CommandNotSupported,
    NoIdentd,
    IdentdError,
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    UnknownErrorCode,
}

#[derive(Debug)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u16)]
pub enum UpnpError {
    NoError = 0,
    InvalidArgument = 402,
    ActionFailed = 501,
    ValueNotInArray = 714,
    SourceIpCannotBeWildcarded = 715,
    ExternalPortCannotBeWildcarded = 716,
    PortMappingConflict = 718,
    InternalPortMustMatchExternal = 724,
    OnlyPermanentLeasesSupported = 725,
    RemoteHostMustBeWildcarded = 726,
    ExternalPortMustBeWildcarded = 727,
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    UnknownErrorCode,
}

impl From<ffi::Error> for LtrsError {
    fn from(err: ffi::Error) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                match err.category {
                    ffi::ErrorCategory::LibtorrentError => {
                        LtrsError::LibtorrentError(err.code.into())
                    }
                    ffi::ErrorCategory::HttpError => {
                        LtrsError::HttpError(err.code.into())
                    }
                    ffi::ErrorCategory::GzipError => {
                        LtrsError::GzipError(err.code.into())
                    }
                    ffi::ErrorCategory::PcpError => {
                        LtrsError::PcpError(err.code.into())
                    }
                    ffi::ErrorCategory::BdecodeError => {
                        LtrsError::BdecodeError(err.code.into())
                    }
                    ffi::ErrorCategory::SocksError => {
                        LtrsError::SocksError(err.code.into())
                    }
                    ffi::ErrorCategory::UpnpError => {
                        LtrsError::UpnpError(err.codee.into())
                    }
                    ffi::ErrorCategory::I2pError => {
                        LtrsError::I2pError(err.code.into())
                    }
                    _ => LtrsError::Unknown(err.code),
                }
            } else {
                // SAFETY: We trust that libtorrent will not return invalid error codes
                unsafe {
                    match err.category {
                        ffi::ErrorCategory::LibtorrentError => {
                            LtrsError::LibtorrentError(std::mem::transmute(err.code as u8))
                        }
                        ffi::ErrorCategory::HttpError => {
                            LtrsError::HttpError(std::mem::transmute(err.code as u16))
                        }
                        ffi::ErrorCategory::GzipError => {
                            LtrsError::GzipError(std::mem::transmute(err.code as u8))
                        }
                        ffi::ErrorCategory::PcpError => {
                            LtrsError::PcpError(std::mem::transmute(err.code as u8))
                        }
                        ffi::ErrorCategory::BdecodeError => {
                            LtrsError::BdecodeError(std::mem::transmute(err.code as u8))
                        }
                        ffi::ErrorCategory::SocksError => {
                            LtrsError::SocksError(std::mem::transmute(err.code as u8))
                        }
                        ffi::ErrorCategory::UpnpError => {
                            LtrsError::UpnpError(std::mem::transmute(err.code as u16))
                        }
                        ffi::ErrorCategory::I2pError => {
                            LtrsError::I2pError(std::mem::transmute(err.code as u8))
                        }
                        _ => LtrsError::Unknown(err.code),
                    }
                }
            }
        }
    }
}
