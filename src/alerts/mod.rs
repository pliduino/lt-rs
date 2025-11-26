use crate::ffi::ffi::{self};

mod torrent_alert;
mod torrent_state;
mod tracker_alert;

pub use torrent_alert::TorrentAlert;
pub use torrent_state::TorrentState;

macro_rules! define_alerts {
[
        $(
            $variant:ident = $value:expr
        ),* $(,)?
] => {
    paste::paste! {
        $(
            pub struct [<$variant Alert>](pub(super) *mut ffi::[<$variant:snake _alert>]);
        )*

        // Sadly this macro does not work with CXX, so we have to manually
        // copy and paste the types...
        // macro_rules! define_alert_enum {
            // () => {
                // enum AlertType {
                    // $(
                        // $variant = $value,
                    // )*

                    // Unknown,
                // }
            // };
        // }
        //};
    }
}
}

define_alerts![
    //TorrentAdded = 3,
    TorrentRemoved = 4,
    ReadPiece = 5,
    FileCompleted = 6,
    FileRenamed = 7,
    FileRenameFailed = 8,
    Performance = 9,
    StateChanged = 10,
    TrackerError = 11,
    TrackerWarning = 12,
    ScrapeReply = 13,
    ScrapeFailed = 14,
    TrackerReply = 15,
    DhtReply = 16,
    TrackerAnnounce = 17,
    HashFailed = 18,
    PeerBan = 19,
    PeerUnsnubbed = 20,
    PeerSnubbed = 21,
    PeerError = 22,
    PeerConnect = 23,
    PeerDisconnected = 24,
    InvalidRequest = 25,
    TorrentFinished = 26,
    PieceFinished = 27,
    RequestDropped = 28,
    BlockTimeout = 29,
    BlockFinished = 30,
    BlockDownloading = 31,
    UnwantedBlock = 32,
    StorageMoved = 33,
    StorageMovedFailed = 34,
    TorrentDeleted = 35,
    TorrentDeleteFailed = 36,
    SaveResumeData = 37,
    SaveResumeDataFailed = 38,
    TorrentPaused = 39,
    TorrentResumed = 40,
    TorrentChecked = 41,
    UrlSeed = 42,
    FileError = 43,
    MetadataFailed = 44,
    MetadataReceived = 45,
    UdpError = 46,
    ExternalIp = 47,
    ListenFailed = 48,
    ListenSucceeded = 49,
    PortmapError = 50,
    Portmap = 51,
    PortmapLog = 52,
    FastresumeRejected = 53,
    PeerBlocked = 54,
    DhtAnnounce = 55,
    DhtGetPeers = 56,
    // Stats = 57,
    CacheFlushed = 58,
    // AnonymousMode = 59,
    LsdPeer = 60,
    Trackerid = 61,
    DhtBootstrap = 62,
    TorrentError = 64,
    TorrentNeedCert = 65,
    IncomingConnection = 66,
    AddTorrent = 67,
    StateUpdate = 68,
    // MmapCache = 69,
    SessionStats = 70,
    DhtError = 73,
    DhtImmutableItem = 74,
    DhtMutableItem = 75,
    DhtPut = 76,
    I2p = 77,
    DhtOutgoingGetPeers = 78,
    Log = 79,
    TorrentLog = 80,
    PeerLog = 81,
    LsdError = 82,
    DhtStats = 83,
    IncomingRequest = 84,
    DhtLog = 85,
    DhtPkt = 86,
    DhtGetPeersReply = 87,
    DhtDirectResponse = 88,
    PickerLog = 89,
    SessionError = 90,
    DhtLiveNodes = 91,
    SessionStatsHeader = 92,
    DhtSampleInfohashes = 93,
    BlockUploaded = 94,
    AlertsDropped = 95,
    Socks5 = 96,
    FilePrio = 97,
    OversizedFile = 98,
    TorrentConflict = 99,
    PeerInfo = 100,
    FileProgress = 101,
    PieceInfo = 102,
    PieceAvailability = 103,
    TrackerList = 104,
];

type TcpEndpoint = String;
type PeerId = String;
type InfoHash = String;
type UserData = String;

pub type ErrorCode = i32;
pub type PieceIndex = i32;

/// Struct to hold information about a single DHT routing table bucket
struct DhtRoutingBucket {
    /// Total number of nodes in the routing table
    num_nodes: i32,

    /// Total number of replacement nodes in the routing table
    num_replacements: i32,

    /// Number of seconds since last activity
    last_active: i32,
}

/// The [`LtSession::pop_alerts()`] function on session is the main interface for retrieving alerts
/// (warnings, messages and errors from libtorrent).
/// If no alerts have been posted by libtorrent [`LtSession::pop_alerts()`] will return an empty list.
///
/// By default, only errors are reported. [`SettingsPack::set_alert_mask()`] can be used to specify
/// which kinds of events should be reported. The alert mask is a combination of the [`AlertCategory`] flags.
///
/// Every alert belongs to one or more category. There is a cost associated with posting alerts.
/// Only alerts that belong to an enabled category are posted. Setting the alert bitmask to 0 will disable all alerts
/// (except those that are non-discardable).
/// Alerts that are responses to API calls such as save_resume_data() and post_session_stats() are //TODO function reference
/// non-discardable and will be posted even if their category is disabled.
///
/// There are other alert base classes that some alerts derive from, all the alerts that are
/// generated for a specific torrent are derived from [`TorrentAlert`],
/// and tracker events derive from [`TrackerAlert`].
///
/// Alerts returned by [`LtSession::pop_alerts()`] are only valid until the next call to [`LtSession::pop_alerts()`].
/// You may not copy an alert object to access it after the next call to [`LtSession::pop_alerts()`].
/// Internal members of alerts also become invalid once [`LtSession::pop_alerts()`] is called again.
pub enum Alert {
    NotImplemented,
    /// This is a base variant for alerts that are associated with a specific torrent. It contains a handle to the torrent.
    ///
    /// Note that by the time the client receives a TorrentAlert, its handle member may be invalid.
    TorrentAlert(TorrentAlert),
    /// This alert is only posted when requested by the user, by calling [`LtSession::post_torrent_updates()`] on the session.
    /// It contains the torrent status of all torrents that changed since last time this message was posted.
    /// Its category is [`AlertCategory::Status`], but it's not subject to filtering, since it's only manually posted anyway.
    StateUpdate(StateUpdateAlert), // TorrentError,
                                   // InvalidRequest,
                                   // TrackerReply,
}

impl From<ffi::CastAlertRaw> for Alert {
    fn from(value: ffi::CastAlertRaw) -> Self {
        macro_rules! type_match {
            {
                $(
                    $name:ident : [$first:ident $(, $rest:ident)*]
                ),* $(,)?
            } => {
                match value.type_ {
                    $(
                        ffi::AlertType::$name => {
                            Alert::$first(
                                type_match!(@wrap $name; $first $(, $rest)*)
                            )
                        }
                    )*

                    ffi::AlertType::Unknown => Alert::NotImplemented,
                    _ => Alert::NotImplemented,
                }
            };

            (@wrap $name:ident; $wrapper:ident, $next:ident $(, $rest:ident)*) => {
                $wrapper::$next(
                    type_match_helper!(@wrap $name; $next $(, $rest)*)
                )
            };

            (@wrap $name:ident; $wrapper:ident) => {
                $wrapper::$name(
                    paste::paste! {
                        [<$name Alert>](value.alert.cast())
                    }
                )
            };
        }

        type_match! {
            TorrentFinished: [TorrentAlert],
            TorrentRemoved: [TorrentAlert],
            ReadPiece: [TorrentAlert],
            AddTorrent: [TorrentAlert],
            StateChanged: [TorrentAlert],
            SaveResumeData: [TorrentAlert],
            SaveResumeDataFailed: [TorrentAlert],
        }
    }
}

impl Alert {
    pub fn category(&self) -> AlertCategory {
        match self {
            Alert::TorrentAlert(alert) => match alert {
                TorrentAlert::AddTorrent(_) => AlertCategory::Status,
                TorrentAlert::TorrentFinished(_) => AlertCategory::Status,
                TorrentAlert::TorrentRemoved(_) => AlertCategory::Status,
                TorrentAlert::ReadPiece(_) => AlertCategory::Storage,
                TorrentAlert::StateChanged(_) => AlertCategory::Status,
                TorrentAlert::SaveResumeData(_) => AlertCategory::Storage,
                TorrentAlert::SaveResumeDataFailed(_) => {
                    AlertCategory::Storage | AlertCategory::Error
                }
                TorrentAlert::PeerAlert(_) => todo!(),
                TorrentAlert::TrackerAlert(_) => todo!(),
            },
            Alert::StateUpdate(_) => AlertCategory::Status,
            Alert::NotImplemented => AlertCategory::empty(),
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AlertCategory: u32 {
        const Error = 1 << 0;
        const Peer = 1 << 1;
        const PortMapping = 1 << 2;
        const Storage = 1 << 3;
        const Tracker = 1 << 4;
        const Connect = 1 << 5;
        const Status = 1 << 6;
        #[deprecated(note = "used only for libtorrent < 1.2")]
        const Progress = 1 << 7;
        const IpBlock = 1 << 8;
        const Performance = 1 << 9;
        const Dht = 1 << 10;
        #[deprecated(note = "used only for libtorrent < 2.0")]
        const Stats = 1 << 11;
        const SessionLog = 1 << 13;
        const TorrentLog = 1 << 14;
        const PeerLog = 1 << 15;
        const IncomingRequest = 1 << 16;
        const DhtLog = 1 << 17;
        const DhtOperation = 1 << 18;
        const PortMappingLog = 1 << 19;
        const PickerLog = 1 << 20;
        const FileProgress = 1 << 21;
        const PieceProgress = 1 << 22;
        const Upload = 1 << 23;
        const BlockProgress = 1 << 24;
    }
}
