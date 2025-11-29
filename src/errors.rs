use crate::ffi::error::ffi::{self, ErrorCodeRaw};

pub enum LtrsError {
    LibtorrentError(LibtorrentError),
    HttpError(HttpError),
    GzipError(GzipError),
    I2pError(I2pError),
    PcpError(PcpError),
    BdecodeError(BdecodeError),
    SocksError(SocksError),
    UpnpError(UpnpError),

    /// If error category is not known
    Unknown(i32),
}

#[derive(Debug)]
pub enum LibtorrentError {
    /// Not an error
    NoError,
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
    InvalidRange,
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

    /// The NAT-PMP router responded with an unsupported protocol version
    #[deprecated]
    UnsupportedProtocolVersion,
    /// You are not authorized to map ports on this NAT-PMP router
    #[deprecated]
    NatpmpNotAuthorized,
    /// The NAT-PMP router failed because of a network failure
    #[deprecated]
    NetworkFailure,
    /// The NAT-PMP router failed because of lack of resources
    #[deprecated]
    NoResources,
    /// The NAT-PMP router failed because an unsupported opcode was sent
    #[deprecated]
    UnsupportedOpcode,

    /// The resume data file is missing the ``file sizes`` entry
    MissingFileSizes,
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
    HttpParseError,
    /// The HTTP response was in the 300-399 range but lacked a location
    /// header
    HttpMissingLocation,
    /// The HTTP response was encoded with gzip or deflate but
    /// decompressing it failed
    HttpFailedDecompress,

    /// The URL specified an i2p address, but no i2p router is configured
    NoI2pRouter,
    /// i2p acceptor is not available yet, can't announce without endpoint
    NoI2pEndpoint,

    /// The tracker URL doesn't support transforming it into a scrape
    /// URL. i.e. it doesn't contain "announce.
    ScrapeNotAvailable,
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
    #[deprecated]
    ExpectedString,
    /// expected colon in bencoded string
    #[deprecated]
    ExpectedColon,
    /// unexpected end of file in bencoded string
    #[deprecated]
    UnexpectedEof,
    /// expected value (list, dict, int or string) in bencoded string
    #[deprecated]
    ExpectedValue,
    /// bencoded recursion depth limit exceeded
    #[deprecated]
    DepthExceeded,
    /// bencoded item count limit exceeded
    #[deprecated]
    LimitExceeded,
    /// integer overflow
    #[deprecated]
    Overflow,

    /// random number generation failed
    NoEntropy,
    /// blocked by SSRF mitigation
    SsrvMitigation,
    /// blocked because IDNA host names are banned
    BlockedByIdna,

    /// the torrent file has an unknown meta version
    TorrentUnknownVersion,
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

    /// the number of error codes
    ErrorCodeMax,

    //// The value is not used by libtorrent itself, it is only used
    //// internally to match againt C++ integers and prevent crashes
    //// as otherwise we would need a unreachable!() in the match arms.
    UnknownErrorCode,
}

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
}

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
    Unknown,
}

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
    ErrorCodeMax,
}

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
}

pub enum BdecodeError {
    NoError = 0,
    ExpectedDigit,
    ExpectedColon,
    UnexpectedEof,
    ExpectedValue,
    DepthExceeded,
    LimitExceeded,
    Overflow,
    ErrorCodeMax,
}

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
    NumErrors,
}

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
}

impl From<ffi::Error> for LtrsError {
    fn from(err: ffi::Error) -> Self {
        match err.category {
            ffi::ErrorCategory::LibtorrentError => LtrsError::LibtorrentError(err.code.into()),
            ffi::ErrorCategory::HttpError => LtrsError::HttpError(err.code.into()),
            ffi::ErrorCategory::GzipError => LtrsError::GzipError(err.code.into()),
            ffi::ErrorCategory::PcpError => LtrsError::PcpError(err.code.into()),
            ffi::ErrorCategory::BdecodeError => LtrsError::BdecodeError(err.code.into()),
            ffi::ErrorCategory::SocksError => LtrsError::SocksError(err.code.into()),
            ffi::ErrorCategory::UpnpError => LtrsError::UpnpError(err.code.into()),
            ffi::ErrorCategory::I2pError => LtrsError::I2pError(err.code.into()),
            _ => LtrsError::Unknown(err.code),
        }
    }
}

impl From<i32> for LibtorrentError {
    fn from(code: i32) -> Self {
        ErrorCodeRaw { repr: code }.into()
    }
}

impl From<i32> for HttpError {
    fn from(code: i32) -> Self {
        unimplemented!()
    }
}

impl From<i32> for UpnpError {
    fn from(code: i32) -> Self {
        unimplemented!()
    }
}

impl From<i32> for I2pError {
    fn from(code: i32) -> Self {
        unimplemented!()
    }
}

impl From<i32> for BdecodeError {
    fn from(code: i32) -> Self {
        unimplemented!()
    }
}

impl From<i32> for SocksError {
    fn from(code: i32) -> Self {
        unimplemented!()
    }
}

impl From<i32> for GzipError {
    fn from(code: i32) -> Self {
        unimplemented!()
    }
}

impl From<i32> for PcpError {
    fn from(code: i32) -> Self {
        unimplemented!()
    }
}

#[allow(deprecated)]
impl From<ErrorCodeRaw> for LibtorrentError {
    fn from(code: ErrorCodeRaw) -> Self {
        match code {
            ErrorCodeRaw::no_error => LibtorrentError::NoError,
            ErrorCodeRaw::file_collision => LibtorrentError::FileCollision,
            ErrorCodeRaw::failed_hash_check => LibtorrentError::FailedHashCheck,
            ErrorCodeRaw::torrent_is_no_dict => LibtorrentError::TorrentIsNoDict,
            ErrorCodeRaw::torrent_missing_info => LibtorrentError::TorrentMissingInfo,
            ErrorCodeRaw::torrent_info_no_dict => LibtorrentError::TorrentInfoNoDict,
            ErrorCodeRaw::torrent_missing_piece_length => {
                LibtorrentError::TorrentMissingPieceLength
            }
            ErrorCodeRaw::torrent_missing_name => LibtorrentError::TorrentMissingName,
            ErrorCodeRaw::torrent_invalid_name => LibtorrentError::TorrentInvalidName,
            ErrorCodeRaw::torrent_invalid_length => LibtorrentError::TorrentInvalidLength,
            ErrorCodeRaw::torrent_file_parse_failed => LibtorrentError::TorrentFileParseFailed,
            ErrorCodeRaw::torrent_missing_pieces => LibtorrentError::TorrentMissingPieces,
            ErrorCodeRaw::torrent_invalid_hashes => LibtorrentError::TorrentInvalidHashes,
            ErrorCodeRaw::too_many_pieces_in_torrent => LibtorrentError::TooManyPiecesInTorrent,
            ErrorCodeRaw::invalid_swarm_metadata => LibtorrentError::InvalidSwarmMetadata,
            ErrorCodeRaw::invalid_bencoding => LibtorrentError::InvalidBencoding,
            ErrorCodeRaw::no_files_in_torrent => LibtorrentError::NoFilesInTorrent,
            ErrorCodeRaw::invalid_escaped_string => LibtorrentError::InvalidEscapedString,
            ErrorCodeRaw::session_is_closing => LibtorrentError::SessionIsClosing,
            ErrorCodeRaw::duplicate_torrent => LibtorrentError::DuplicateTorrent,
            ErrorCodeRaw::invalid_torrent_handle => LibtorrentError::InvalidTorrentHandle,
            ErrorCodeRaw::invalid_entry_type => LibtorrentError::InvalidEntryType,
            ErrorCodeRaw::missing_info_hash_in_uri => LibtorrentError::MissingInfoHashInUri,
            ErrorCodeRaw::file_too_short => LibtorrentError::FileTooShort,
            ErrorCodeRaw::unsupported_url_protocol => LibtorrentError::UnsupportedUrlProtocol,
            ErrorCodeRaw::url_parse_error => LibtorrentError::UrlParseError,
            ErrorCodeRaw::peer_sent_empty_piece => LibtorrentError::PeerSentEmptyPiece,
            ErrorCodeRaw::parse_failed => LibtorrentError::ParseFailed,
            ErrorCodeRaw::invalid_file_tag => LibtorrentError::InvalidFileTag,
            ErrorCodeRaw::missing_info_hash => LibtorrentError::MissingInfoHash,
            ErrorCodeRaw::mismatching_info_hash => LibtorrentError::MismatchingInfoHash,
            ErrorCodeRaw::invalid_hostname => LibtorrentError::InvalidHostname,
            ErrorCodeRaw::invalid_port => LibtorrentError::InvalidPort,
            ErrorCodeRaw::port_blocked => LibtorrentError::PortBlocked,
            ErrorCodeRaw::expected_close_bracket_in_address => {
                LibtorrentError::ExpectedCloseBracketInAddress
            }
            ErrorCodeRaw::destructing_torrent => LibtorrentError::DestructingTorrent,
            ErrorCodeRaw::timed_out => LibtorrentError::TimedOut,
            ErrorCodeRaw::upload_upload_connection => LibtorrentError::UploadUploadConnection,
            ErrorCodeRaw::uninteresting_upload_peer => LibtorrentError::UninterestingUploadPeer,
            ErrorCodeRaw::invalid_info_hash => LibtorrentError::InvalidInfoHash,
            ErrorCodeRaw::torrent_paused => LibtorrentError::TorrentPaused,
            ErrorCodeRaw::invalid_have => LibtorrentError::InvalidHave,
            ErrorCodeRaw::invalid_bitfield_size => LibtorrentError::InvalidBitfieldSize,
            ErrorCodeRaw::too_many_requests_when_choked => {
                LibtorrentError::TooManyRequestsWhenChoked
            }
            ErrorCodeRaw::invalid_piece => LibtorrentError::InvalidPiece,
            ErrorCodeRaw::no_memory => LibtorrentError::NoMemory,
            ErrorCodeRaw::torrent_aborted => LibtorrentError::TorrentAborted,
            ErrorCodeRaw::self_connection => LibtorrentError::SelfConnection,
            ErrorCodeRaw::invalid_piece_size => LibtorrentError::InvalidPieceSize,
            ErrorCodeRaw::timed_out_no_interest => LibtorrentError::TimedOutNoInterest,
            ErrorCodeRaw::timed_out_inactivity => LibtorrentError::TimedOutInactivity,
            ErrorCodeRaw::timed_out_no_handshake => LibtorrentError::TimedOutNoHandshake,
            ErrorCodeRaw::timed_out_no_request => LibtorrentError::TimedOutNoRequest,
            ErrorCodeRaw::invalid_choke => LibtorrentError::InvalidChoke,
            ErrorCodeRaw::invalid_unchoke => LibtorrentError::InvalidUnchoke,
            ErrorCodeRaw::invalid_interested => LibtorrentError::InvalidInterested,
            ErrorCodeRaw::invalid_not_interested => LibtorrentError::InvalidNotInterested,
            ErrorCodeRaw::invalid_request => LibtorrentError::InvalidRequest,
            ErrorCodeRaw::invalid_hash_list => LibtorrentError::InvalidHashList,
            ErrorCodeRaw::invalid_hash_piece => LibtorrentError::InvalidHashPiece,
            ErrorCodeRaw::invalid_cancel => LibtorrentError::InvalidCancel,
            ErrorCodeRaw::invalid_dht_port => LibtorrentError::InvalidDhtPort,
            ErrorCodeRaw::invalid_suggest => LibtorrentError::InvalidSuggest,
            ErrorCodeRaw::invalid_have_all => LibtorrentError::InvalidHaveAll,
            ErrorCodeRaw::invalid_have_none => LibtorrentError::InvalidHaveNone,
            ErrorCodeRaw::invalid_reject => LibtorrentError::InvalidReject,
            ErrorCodeRaw::invalid_allow_fast => LibtorrentError::InvalidAllowFast,
            ErrorCodeRaw::invalid_extended => LibtorrentError::InvalidExtended,
            ErrorCodeRaw::invalid_message => LibtorrentError::InvalidMessage,
            ErrorCodeRaw::sync_hash_not_found => LibtorrentError::SyncHashNotFound,
            ErrorCodeRaw::invalid_encryption_constant => LibtorrentError::InvalidEncryptionConstant,
            ErrorCodeRaw::no_plaintext_mode => LibtorrentError::NoPlaintextMode,
            ErrorCodeRaw::no_rc4_mode => LibtorrentError::NoRc4Mode,
            ErrorCodeRaw::unsupported_encryption_mode => LibtorrentError::UnsupportedEncryptionMode,
            ErrorCodeRaw::unsupported_encryption_mode_selected => {
                LibtorrentError::UnsupportedEncryptionModeSelected
            }
            ErrorCodeRaw::invalid_pad_size => LibtorrentError::InvalidPadSize,
            ErrorCodeRaw::invalid_encrypt_handshake => LibtorrentError::InvalidEncryptHandshake,
            ErrorCodeRaw::no_incoming_encrypted => LibtorrentError::NoIncomingEncrypted,
            ErrorCodeRaw::no_incoming_regular => LibtorrentError::NoIncomingRegular,
            ErrorCodeRaw::duplicate_peer_id => LibtorrentError::DuplicatePeerId,
            ErrorCodeRaw::torrent_removed => LibtorrentError::TorrentRemoved,
            ErrorCodeRaw::packet_too_large => LibtorrentError::PacketTooLarge,

            ErrorCodeRaw::reserved => LibtorrentError::Reserved,

            ErrorCodeRaw::http_error => LibtorrentError::HttpError,
            ErrorCodeRaw::missing_location => LibtorrentError::MissingLocation,
            ErrorCodeRaw::invalid_redirection => LibtorrentError::InvalidRedirection,
            ErrorCodeRaw::redirecting => LibtorrentError::Redirecting,
            ErrorCodeRaw::invalid_range => LibtorrentError::InvalidRange,
            ErrorCodeRaw::no_content_length => LibtorrentError::NoContentLength,
            ErrorCodeRaw::banned_by_ip_filter => LibtorrentError::BannedByIpFilter,
            ErrorCodeRaw::too_many_connections => LibtorrentError::TooManyConnections,
            ErrorCodeRaw::peer_banned => LibtorrentError::PeerBanned,
            ErrorCodeRaw::stopping_torrent => LibtorrentError::StoppingTorrent,
            ErrorCodeRaw::too_many_corrupt_pieces => LibtorrentError::TooManyCorruptPieces,
            ErrorCodeRaw::torrent_not_ready => LibtorrentError::TorrentNotReady,
            ErrorCodeRaw::peer_not_constructed => LibtorrentError::PeerNotConstructed,
            ErrorCodeRaw::session_closing => LibtorrentError::SessionClosing,
            ErrorCodeRaw::optimistic_disconnect => LibtorrentError::OptimisticDisconnect,
            ErrorCodeRaw::torrent_finished => LibtorrentError::TorrentFinished,
            ErrorCodeRaw::no_router => LibtorrentError::NoRouter,
            ErrorCodeRaw::metadata_too_large => LibtorrentError::MetadataTooLarge,
            ErrorCodeRaw::invalid_metadata_request => LibtorrentError::InvalidMetadataRequest,
            ErrorCodeRaw::invalid_metadata_size => LibtorrentError::InvalidMetadataSize,
            ErrorCodeRaw::invalid_metadata_offset => LibtorrentError::InvalidMetadataOffset,
            ErrorCodeRaw::invalid_metadata_message => LibtorrentError::InvalidMetadataMessage,
            ErrorCodeRaw::pex_message_too_large => LibtorrentError::PexMessageTooLarge,
            ErrorCodeRaw::invalid_pex_message => LibtorrentError::InvalidPexMessage,
            ErrorCodeRaw::invalid_lt_tracker_message => LibtorrentError::InvalidLtTrackerMessage,
            ErrorCodeRaw::too_frequent_pex => LibtorrentError::TooFrequentPex,
            ErrorCodeRaw::no_metadata => LibtorrentError::NoMetadata,
            ErrorCodeRaw::invalid_dont_have => LibtorrentError::InvalidDontHave,
            ErrorCodeRaw::requires_ssl_connection => LibtorrentError::RequiresSslConnection,
            ErrorCodeRaw::invalid_ssl_cert => LibtorrentError::InvalidSslCert,
            ErrorCodeRaw::not_an_ssl_torrent => LibtorrentError::NotAnSslTorrent,
            ErrorCodeRaw::banned_by_port_filter => LibtorrentError::BannedByPortFilter,
            ErrorCodeRaw::invalid_session_handle => LibtorrentError::InvalidSessionHandle,
            ErrorCodeRaw::invalid_listen_socket => LibtorrentError::InvalidListenSocket,
            ErrorCodeRaw::invalid_hash_request => LibtorrentError::InvalidHashRequest,
            ErrorCodeRaw::invalid_hashes => LibtorrentError::InvalidHashes,
            ErrorCodeRaw::invalid_hash_reject => LibtorrentError::InvalidHashReject,

            ErrorCodeRaw::unsupported_protocol_version => {
                LibtorrentError::UnsupportedProtocolVersion
            }
            ErrorCodeRaw::natpmp_not_authorized => LibtorrentError::NatpmpNotAuthorized,
            ErrorCodeRaw::network_failure => LibtorrentError::NetworkFailure,
            ErrorCodeRaw::no_resources => LibtorrentError::NoResources,
            ErrorCodeRaw::unsupported_opcode => LibtorrentError::UnsupportedOpcode,

            ErrorCodeRaw::missing_file_sizes => LibtorrentError::MissingFileSizes,
            ErrorCodeRaw::no_files_in_resume_data => LibtorrentError::NoFilesInResumeData,
            ErrorCodeRaw::missing_pieces => LibtorrentError::MissingPieces,
            ErrorCodeRaw::mismatching_number_of_files => LibtorrentError::MismatchingNumberOfFiles,
            ErrorCodeRaw::mismatching_file_size => LibtorrentError::MismatchingFileSize,
            ErrorCodeRaw::mismatching_file_timestamp => LibtorrentError::MismatchingFileTimestamp,
            ErrorCodeRaw::not_a_dictionary => LibtorrentError::NotADictionary,
            ErrorCodeRaw::invalid_blocks_per_piece => LibtorrentError::InvalidBlocksPerPiece,
            ErrorCodeRaw::missing_slots => LibtorrentError::MissingSlots,
            ErrorCodeRaw::too_many_slots => LibtorrentError::TooManySlots,
            ErrorCodeRaw::invalid_slot_list => LibtorrentError::InvalidSlotList,
            ErrorCodeRaw::invalid_piece_index => LibtorrentError::InvalidPieceIndex,
            ErrorCodeRaw::pieces_need_reorder => LibtorrentError::PiecesNeedReorder,
            ErrorCodeRaw::resume_data_not_modified => LibtorrentError::ResumeDataNotModified,
            ErrorCodeRaw::invalid_save_path => LibtorrentError::InvalidSavePath,

            ErrorCodeRaw::http_parse_error => LibtorrentError::HttpParseError,
            ErrorCodeRaw::http_missing_location => LibtorrentError::HttpMissingLocation,
            ErrorCodeRaw::http_failed_decompress => LibtorrentError::HttpFailedDecompress,

            ErrorCodeRaw::no_i2p_router => LibtorrentError::NoI2pRouter,
            ErrorCodeRaw::no_i2p_endpoint => LibtorrentError::NoI2pEndpoint,

            ErrorCodeRaw::scrape_not_available => LibtorrentError::ScrapeNotAvailable,
            ErrorCodeRaw::invalid_tracker_response => LibtorrentError::InvalidTrackerResponse,
            ErrorCodeRaw::invalid_peer_dict => LibtorrentError::InvalidPeerDict,
            ErrorCodeRaw::tracker_failure => LibtorrentError::TrackerFailure,
            ErrorCodeRaw::invalid_files_entry => LibtorrentError::InvalidFilesEntry,
            ErrorCodeRaw::invalid_hash_entry => LibtorrentError::InvalidHashEntry,
            ErrorCodeRaw::invalid_peers_entry => LibtorrentError::InvalidPeersEntry,
            ErrorCodeRaw::invalid_tracker_response_length => {
                LibtorrentError::InvalidTrackerResponseLength
            }

            ErrorCodeRaw::invalid_tracker_transaction_id => {
                LibtorrentError::InvalidTrackerTransactionId
            }
            ErrorCodeRaw::invalid_tracker_action => LibtorrentError::InvalidTrackerAction,
            ErrorCodeRaw::announce_skipped => LibtorrentError::AnnounceSkipped,

            ErrorCodeRaw::expected_string => LibtorrentError::ExpectedString,
            ErrorCodeRaw::expected_colon => LibtorrentError::ExpectedColon,
            ErrorCodeRaw::unexpected_eof => LibtorrentError::UnexpectedEof,
            ErrorCodeRaw::expected_value => LibtorrentError::ExpectedValue,
            ErrorCodeRaw::depth_exceeded => LibtorrentError::DepthExceeded,
            ErrorCodeRaw::limit_exceeded => LibtorrentError::LimitExceeded,
            ErrorCodeRaw::overflow => LibtorrentError::Overflow,

            ErrorCodeRaw::no_entropy => LibtorrentError::NoEntropy,
            ErrorCodeRaw::ssrf_mitigation => LibtorrentError::SsrvMitigation,
            ErrorCodeRaw::blocked_by_idna => LibtorrentError::BlockedByIdna,

            ErrorCodeRaw::torrent_unknown_version => LibtorrentError::TorrentUnknownVersion,
            ErrorCodeRaw::torrent_missing_file_tree => LibtorrentError::TorrentMissingFileTree,
            ErrorCodeRaw::torrent_missing_meta_version => {
                LibtorrentError::TorrentMissingMetaVersion
            }
            ErrorCodeRaw::torrent_inconsistent_files => LibtorrentError::TorrentInconsistentFiles,
            ErrorCodeRaw::torrent_missing_piece_layer => LibtorrentError::TorrentMissingPieceLayer,
            ErrorCodeRaw::torrent_invalid_piece_layer => LibtorrentError::TorrentInvalidPieceLayer,
            ErrorCodeRaw::torrent_missing_pieces_root => LibtorrentError::TorrentMissingPiecesRoot,
            ErrorCodeRaw::torrent_inconsistent_hashes => LibtorrentError::TorrentInconsistentHashes,
            ErrorCodeRaw::torrent_invalid_pad_file => LibtorrentError::TorrentInvalidPadFile,
            ErrorCodeRaw::error_code_max => LibtorrentError::ErrorCodeMax,
            _ => LibtorrentError::UnknownErrorCode,
        }
    }
}
