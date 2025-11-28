use crate::ffi::ffi::ErrorCodeRaw;

#[derive(Debug)]
pub enum LibTorrentError {
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

#[allow(deprecated)]
impl From<ErrorCodeRaw> for LibTorrentError {
    fn from(code: ErrorCodeRaw) -> Self {
        match code {
            ErrorCodeRaw::no_error => LibTorrentError::NoError,
            ErrorCodeRaw::file_collision => LibTorrentError::FileCollision,
            ErrorCodeRaw::failed_hash_check => LibTorrentError::FailedHashCheck,
            ErrorCodeRaw::torrent_is_no_dict => LibTorrentError::TorrentIsNoDict,
            ErrorCodeRaw::torrent_missing_info => LibTorrentError::TorrentMissingInfo,
            ErrorCodeRaw::torrent_info_no_dict => LibTorrentError::TorrentInfoNoDict,
            ErrorCodeRaw::torrent_missing_piece_length => {
                LibTorrentError::TorrentMissingPieceLength
            }
            ErrorCodeRaw::torrent_missing_name => LibTorrentError::TorrentMissingName,
            ErrorCodeRaw::torrent_invalid_name => LibTorrentError::TorrentInvalidName,
            ErrorCodeRaw::torrent_invalid_length => LibTorrentError::TorrentInvalidLength,
            ErrorCodeRaw::torrent_file_parse_failed => LibTorrentError::TorrentFileParseFailed,
            ErrorCodeRaw::torrent_missing_pieces => LibTorrentError::TorrentMissingPieces,
            ErrorCodeRaw::torrent_invalid_hashes => LibTorrentError::TorrentInvalidHashes,
            ErrorCodeRaw::too_many_pieces_in_torrent => LibTorrentError::TooManyPiecesInTorrent,
            ErrorCodeRaw::invalid_swarm_metadata => LibTorrentError::InvalidSwarmMetadata,
            ErrorCodeRaw::invalid_bencoding => LibTorrentError::InvalidBencoding,
            ErrorCodeRaw::no_files_in_torrent => LibTorrentError::NoFilesInTorrent,
            ErrorCodeRaw::invalid_escaped_string => LibTorrentError::InvalidEscapedString,
            ErrorCodeRaw::session_is_closing => LibTorrentError::SessionIsClosing,
            ErrorCodeRaw::duplicate_torrent => LibTorrentError::DuplicateTorrent,
            ErrorCodeRaw::invalid_torrent_handle => LibTorrentError::InvalidTorrentHandle,
            ErrorCodeRaw::invalid_entry_type => LibTorrentError::InvalidEntryType,
            ErrorCodeRaw::missing_info_hash_in_uri => LibTorrentError::MissingInfoHashInUri,
            ErrorCodeRaw::file_too_short => LibTorrentError::FileTooShort,
            ErrorCodeRaw::unsupported_url_protocol => LibTorrentError::UnsupportedUrlProtocol,
            ErrorCodeRaw::url_parse_error => LibTorrentError::UrlParseError,
            ErrorCodeRaw::peer_sent_empty_piece => LibTorrentError::PeerSentEmptyPiece,
            ErrorCodeRaw::parse_failed => LibTorrentError::ParseFailed,
            ErrorCodeRaw::invalid_file_tag => LibTorrentError::InvalidFileTag,
            ErrorCodeRaw::missing_info_hash => LibTorrentError::MissingInfoHash,
            ErrorCodeRaw::mismatching_info_hash => LibTorrentError::MismatchingInfoHash,
            ErrorCodeRaw::invalid_hostname => LibTorrentError::InvalidHostname,
            ErrorCodeRaw::invalid_port => LibTorrentError::InvalidPort,
            ErrorCodeRaw::port_blocked => LibTorrentError::PortBlocked,
            ErrorCodeRaw::expected_close_bracket_in_address => {
                LibTorrentError::ExpectedCloseBracketInAddress
            }
            ErrorCodeRaw::destructing_torrent => LibTorrentError::DestructingTorrent,
            ErrorCodeRaw::timed_out => LibTorrentError::TimedOut,
            ErrorCodeRaw::upload_upload_connection => LibTorrentError::UploadUploadConnection,
            ErrorCodeRaw::uninteresting_upload_peer => LibTorrentError::UninterestingUploadPeer,
            ErrorCodeRaw::invalid_info_hash => LibTorrentError::InvalidInfoHash,
            ErrorCodeRaw::torrent_paused => LibTorrentError::TorrentPaused,
            ErrorCodeRaw::invalid_have => LibTorrentError::InvalidHave,
            ErrorCodeRaw::invalid_bitfield_size => LibTorrentError::InvalidBitfieldSize,
            ErrorCodeRaw::too_many_requests_when_choked => {
                LibTorrentError::TooManyRequestsWhenChoked
            }
            ErrorCodeRaw::invalid_piece => LibTorrentError::InvalidPiece,
            ErrorCodeRaw::no_memory => LibTorrentError::NoMemory,
            ErrorCodeRaw::torrent_aborted => LibTorrentError::TorrentAborted,
            ErrorCodeRaw::self_connection => LibTorrentError::SelfConnection,
            ErrorCodeRaw::invalid_piece_size => LibTorrentError::InvalidPieceSize,
            ErrorCodeRaw::timed_out_no_interest => LibTorrentError::TimedOutNoInterest,
            ErrorCodeRaw::timed_out_inactivity => LibTorrentError::TimedOutInactivity,
            ErrorCodeRaw::timed_out_no_handshake => LibTorrentError::TimedOutNoHandshake,
            ErrorCodeRaw::timed_out_no_request => LibTorrentError::TimedOutNoRequest,
            ErrorCodeRaw::invalid_choke => LibTorrentError::InvalidChoke,
            ErrorCodeRaw::invalid_unchoke => LibTorrentError::InvalidUnchoke,
            ErrorCodeRaw::invalid_interested => LibTorrentError::InvalidInterested,
            ErrorCodeRaw::invalid_not_interested => LibTorrentError::InvalidNotInterested,
            ErrorCodeRaw::invalid_request => LibTorrentError::InvalidRequest,
            ErrorCodeRaw::invalid_hash_list => LibTorrentError::InvalidHashList,
            ErrorCodeRaw::invalid_hash_piece => LibTorrentError::InvalidHashPiece,
            ErrorCodeRaw::invalid_cancel => LibTorrentError::InvalidCancel,
            ErrorCodeRaw::invalid_dht_port => LibTorrentError::InvalidDhtPort,
            ErrorCodeRaw::invalid_suggest => LibTorrentError::InvalidSuggest,
            ErrorCodeRaw::invalid_have_all => LibTorrentError::InvalidHaveAll,
            ErrorCodeRaw::invalid_have_none => LibTorrentError::InvalidHaveNone,
            ErrorCodeRaw::invalid_reject => LibTorrentError::InvalidReject,
            ErrorCodeRaw::invalid_allow_fast => LibTorrentError::InvalidAllowFast,
            ErrorCodeRaw::invalid_extended => LibTorrentError::InvalidExtended,
            ErrorCodeRaw::invalid_message => LibTorrentError::InvalidMessage,
            ErrorCodeRaw::sync_hash_not_found => LibTorrentError::SyncHashNotFound,
            ErrorCodeRaw::invalid_encryption_constant => LibTorrentError::InvalidEncryptionConstant,
            ErrorCodeRaw::no_plaintext_mode => LibTorrentError::NoPlaintextMode,
            ErrorCodeRaw::no_rc4_mode => LibTorrentError::NoRc4Mode,
            ErrorCodeRaw::unsupported_encryption_mode => LibTorrentError::UnsupportedEncryptionMode,
            ErrorCodeRaw::unsupported_encryption_mode_selected => {
                LibTorrentError::UnsupportedEncryptionModeSelected
            }
            ErrorCodeRaw::invalid_pad_size => LibTorrentError::InvalidPadSize,
            ErrorCodeRaw::invalid_encrypt_handshake => LibTorrentError::InvalidEncryptHandshake,
            ErrorCodeRaw::no_incoming_encrypted => LibTorrentError::NoIncomingEncrypted,
            ErrorCodeRaw::no_incoming_regular => LibTorrentError::NoIncomingRegular,
            ErrorCodeRaw::duplicate_peer_id => LibTorrentError::DuplicatePeerId,
            ErrorCodeRaw::torrent_removed => LibTorrentError::TorrentRemoved,
            ErrorCodeRaw::packet_too_large => LibTorrentError::PacketTooLarge,

            ErrorCodeRaw::reserved => LibTorrentError::Reserved,

            ErrorCodeRaw::http_error => LibTorrentError::HttpError,
            ErrorCodeRaw::missing_location => LibTorrentError::MissingLocation,
            ErrorCodeRaw::invalid_redirection => LibTorrentError::InvalidRedirection,
            ErrorCodeRaw::redirecting => LibTorrentError::Redirecting,
            ErrorCodeRaw::invalid_range => LibTorrentError::InvalidRange,
            ErrorCodeRaw::no_content_length => LibTorrentError::NoContentLength,
            ErrorCodeRaw::banned_by_ip_filter => LibTorrentError::BannedByIpFilter,
            ErrorCodeRaw::too_many_connections => LibTorrentError::TooManyConnections,
            ErrorCodeRaw::peer_banned => LibTorrentError::PeerBanned,
            ErrorCodeRaw::stopping_torrent => LibTorrentError::StoppingTorrent,
            ErrorCodeRaw::too_many_corrupt_pieces => LibTorrentError::TooManyCorruptPieces,
            ErrorCodeRaw::torrent_not_ready => LibTorrentError::TorrentNotReady,
            ErrorCodeRaw::peer_not_constructed => LibTorrentError::PeerNotConstructed,
            ErrorCodeRaw::session_closing => LibTorrentError::SessionClosing,
            ErrorCodeRaw::optimistic_disconnect => LibTorrentError::OptimisticDisconnect,
            ErrorCodeRaw::torrent_finished => LibTorrentError::TorrentFinished,
            ErrorCodeRaw::no_router => LibTorrentError::NoRouter,
            ErrorCodeRaw::metadata_too_large => LibTorrentError::MetadataTooLarge,
            ErrorCodeRaw::invalid_metadata_request => LibTorrentError::InvalidMetadataRequest,
            ErrorCodeRaw::invalid_metadata_size => LibTorrentError::InvalidMetadataSize,
            ErrorCodeRaw::invalid_metadata_offset => LibTorrentError::InvalidMetadataOffset,
            ErrorCodeRaw::invalid_metadata_message => LibTorrentError::InvalidMetadataMessage,
            ErrorCodeRaw::pex_message_too_large => LibTorrentError::PexMessageTooLarge,
            ErrorCodeRaw::invalid_pex_message => LibTorrentError::InvalidPexMessage,
            ErrorCodeRaw::invalid_lt_tracker_message => LibTorrentError::InvalidLtTrackerMessage,
            ErrorCodeRaw::too_frequent_pex => LibTorrentError::TooFrequentPex,
            ErrorCodeRaw::no_metadata => LibTorrentError::NoMetadata,
            ErrorCodeRaw::invalid_dont_have => LibTorrentError::InvalidDontHave,
            ErrorCodeRaw::requires_ssl_connection => LibTorrentError::RequiresSslConnection,
            ErrorCodeRaw::invalid_ssl_cert => LibTorrentError::InvalidSslCert,
            ErrorCodeRaw::not_an_ssl_torrent => LibTorrentError::NotAnSslTorrent,
            ErrorCodeRaw::banned_by_port_filter => LibTorrentError::BannedByPortFilter,
            ErrorCodeRaw::invalid_session_handle => LibTorrentError::InvalidSessionHandle,
            ErrorCodeRaw::invalid_listen_socket => LibTorrentError::InvalidListenSocket,
            ErrorCodeRaw::invalid_hash_request => LibTorrentError::InvalidHashRequest,
            ErrorCodeRaw::invalid_hashes => LibTorrentError::InvalidHashes,
            ErrorCodeRaw::invalid_hash_reject => LibTorrentError::InvalidHashReject,

            ErrorCodeRaw::unsupported_protocol_version => {
                LibTorrentError::UnsupportedProtocolVersion
            }
            ErrorCodeRaw::natpmp_not_authorized => LibTorrentError::NatpmpNotAuthorized,
            ErrorCodeRaw::network_failure => LibTorrentError::NetworkFailure,
            ErrorCodeRaw::no_resources => LibTorrentError::NoResources,
            ErrorCodeRaw::unsupported_opcode => LibTorrentError::UnsupportedOpcode,

            ErrorCodeRaw::missing_file_sizes => LibTorrentError::MissingFileSizes,
            ErrorCodeRaw::no_files_in_resume_data => LibTorrentError::NoFilesInResumeData,
            ErrorCodeRaw::missing_pieces => LibTorrentError::MissingPieces,
            ErrorCodeRaw::mismatching_number_of_files => LibTorrentError::MismatchingNumberOfFiles,
            ErrorCodeRaw::mismatching_file_size => LibTorrentError::MismatchingFileSize,
            ErrorCodeRaw::mismatching_file_timestamp => LibTorrentError::MismatchingFileTimestamp,
            ErrorCodeRaw::not_a_dictionary => LibTorrentError::NotADictionary,
            ErrorCodeRaw::invalid_blocks_per_piece => LibTorrentError::InvalidBlocksPerPiece,
            ErrorCodeRaw::missing_slots => LibTorrentError::MissingSlots,
            ErrorCodeRaw::too_many_slots => LibTorrentError::TooManySlots,
            ErrorCodeRaw::invalid_slot_list => LibTorrentError::InvalidSlotList,
            ErrorCodeRaw::invalid_piece_index => LibTorrentError::InvalidPieceIndex,
            ErrorCodeRaw::pieces_need_reorder => LibTorrentError::PiecesNeedReorder,
            ErrorCodeRaw::resume_data_not_modified => LibTorrentError::ResumeDataNotModified,
            ErrorCodeRaw::invalid_save_path => LibTorrentError::InvalidSavePath,

            ErrorCodeRaw::http_parse_error => LibTorrentError::HttpParseError,
            ErrorCodeRaw::http_missing_location => LibTorrentError::HttpMissingLocation,
            ErrorCodeRaw::http_failed_decompress => LibTorrentError::HttpFailedDecompress,

            ErrorCodeRaw::no_i2p_router => LibTorrentError::NoI2pRouter,
            ErrorCodeRaw::no_i2p_endpoint => LibTorrentError::NoI2pEndpoint,

            ErrorCodeRaw::scrape_not_available => LibTorrentError::ScrapeNotAvailable,
            ErrorCodeRaw::invalid_tracker_response => LibTorrentError::InvalidTrackerResponse,
            ErrorCodeRaw::invalid_peer_dict => LibTorrentError::InvalidPeerDict,
            ErrorCodeRaw::tracker_failure => LibTorrentError::TrackerFailure,
            ErrorCodeRaw::invalid_files_entry => LibTorrentError::InvalidFilesEntry,
            ErrorCodeRaw::invalid_hash_entry => LibTorrentError::InvalidHashEntry,
            ErrorCodeRaw::invalid_peers_entry => LibTorrentError::InvalidPeersEntry,
            ErrorCodeRaw::invalid_tracker_response_length => {
                LibTorrentError::InvalidTrackerResponseLength
            }

            ErrorCodeRaw::invalid_tracker_transaction_id => {
                LibTorrentError::InvalidTrackerTransactionId
            }
            ErrorCodeRaw::invalid_tracker_action => LibTorrentError::InvalidTrackerAction,
            ErrorCodeRaw::announce_skipped => LibTorrentError::AnnounceSkipped,

            ErrorCodeRaw::expected_string => LibTorrentError::ExpectedString,
            ErrorCodeRaw::expected_colon => LibTorrentError::ExpectedColon,
            ErrorCodeRaw::unexpected_eof => LibTorrentError::UnexpectedEof,
            ErrorCodeRaw::expected_value => LibTorrentError::ExpectedValue,
            ErrorCodeRaw::depth_exceeded => LibTorrentError::DepthExceeded,
            ErrorCodeRaw::limit_exceeded => LibTorrentError::LimitExceeded,
            ErrorCodeRaw::overflow => LibTorrentError::Overflow,

            ErrorCodeRaw::no_entropy => LibTorrentError::NoEntropy,
            ErrorCodeRaw::ssrf_mitigation => LibTorrentError::SsrvMitigation,
            ErrorCodeRaw::blocked_by_idna => LibTorrentError::BlockedByIdna,

            ErrorCodeRaw::torrent_unknown_version => LibTorrentError::TorrentUnknownVersion,
            ErrorCodeRaw::torrent_missing_file_tree => LibTorrentError::TorrentMissingFileTree,
            ErrorCodeRaw::torrent_missing_meta_version => {
                LibTorrentError::TorrentMissingMetaVersion
            }
            ErrorCodeRaw::torrent_inconsistent_files => LibTorrentError::TorrentInconsistentFiles,
            ErrorCodeRaw::torrent_missing_piece_layer => LibTorrentError::TorrentMissingPieceLayer,
            ErrorCodeRaw::torrent_invalid_piece_layer => LibTorrentError::TorrentInvalidPieceLayer,
            ErrorCodeRaw::torrent_missing_pieces_root => LibTorrentError::TorrentMissingPiecesRoot,
            ErrorCodeRaw::torrent_inconsistent_hashes => LibTorrentError::TorrentInconsistentHashes,
            ErrorCodeRaw::torrent_invalid_pad_file => LibTorrentError::TorrentInvalidPadFile,

            ErrorCodeRaw::error_code_max => LibTorrentError::ErrorCodeMax,

            ErrorCodeRaw::unknown => LibTorrentError::UnknownErrorCode,
            _ => LibTorrentError::UnknownErrorCode,
        }
    }
}
