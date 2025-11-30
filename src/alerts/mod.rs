use crate::{
    alerts::{peer_alert::PeerAlert, tracker_alert::TrackerAlert},
    ffi::ffi::{self},
};

pub mod implementations;
pub mod peer_alert;
pub mod performance_warning;
pub mod torrent_alert;
pub mod torrent_state;
pub mod tracker_alert;

pub use torrent_alert::TorrentAlert;
pub use torrent_state::TorrentState;

use peer_alert::PeerAlertRaw;
use torrent_alert::TorrentAlertRaw;
use tracker_alert::TrackerAlertRaw;

macro_rules! define_alerts {
[
    $(
        $variant:ident = $value:expr
    ),* $(,)?
] => {
    paste::paste! {
        $(
            pub struct [<$variant Alert>] (pub(super) *mut ffi::[<$variant:snake _alert>]);
            impl [<$variant Alert>] {

            }
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
// ! Alerts should only exist inside sessions so lifetimes are
// ! easier to manage
pub enum Alert {
    NotImplemented,
    /// This alert is posted when there is an error on a UDP socket. The
    /// UDP sockets are used for all uTP, DHT and UDP tracker traffic. They are
    /// global to the session.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    UdpError(UdpErrorAlert),
    /// Whenever libtorrent learns about the machines external IP, this alert is
    /// generated. The external IP address can be acquired from the tracker (if it
    /// supports that) or from peers that supports the extension protocol.
    /// The address can be accessed through the ``external_address`` member.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    ExternalIp(ExternalIpAlert),
    /// This alert is generated when none of the ports, given in the port range, to
    /// session can be opened for listening. The ``listen_interface`` member is the
    /// interface that failed, ``error`` is the error code describing the failure.
    ///
    /// In the case an endpoint was created before generating the alert, it is
    /// represented by ``address`` and ``port``. The combinations of socket type
    /// and operation in which such address and port are not valid are:
    /// accept  - i2p
    /// accept  - socks5
    /// enum_if - tcp
    ///
    /// libtorrent may sometimes try to listen on port 0, if all other ports failed.
    /// Port 0 asks the operating system to pick a port that's free). If that fails
    /// you may see a [`Alert::ListenFailed`] with port 0 even if you didn't ask to
    /// listen on it.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`] | [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    ListenFailed(ListenFailedAlert),
    /// This alert is posted when the listen port succeeds to be opened on a
    /// particular interface. ``address`` and ``port`` is the endpoint that
    /// successfully was opened for listening.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    ListenSucceeded(ListenSucceededAlert),
    /// This alert is generated when a NAT router was successfully found but some
    /// part of the port mapping request failed. It contains a text message that
    /// may help the user figure out what is wrong. This alert is not generated in
    /// case it appears the client is not running on a NAT:ed network or if it
    /// appears there is no NAT router that can be remote controlled to add port
    /// mappings.
    ///
    /// ## Alert Category
    /// [`AlertCategory::PortMapping`] | [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    PortmapError(PortmapErrorAlert),
    /// This alert is generated when a NAT router was successfully found and
    /// a port was successfully mapped on it. On a NAT:ed network with a NAT-PMP
    /// capable router, this is typically generated once when mapping the TCP
    /// port and, if DHT is enabled, when the UDP port is mapped.
    ///
    /// ## Alert Category
    /// [`AlertCategory::PortMapping`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    Portmap(PortmapAlert),
    /// This alert is generated to log informational events related to either
    /// UPnP or NAT-PMP. They contain a log line and the type (0 = NAT-PMP
    /// and 1 = UPnP). Displaying these messages to an end user is only useful
    /// for debugging the UPnP or NAT-PMP implementation. This alert is only
    /// posted if the alert_category::port_mapping_log flag is enabled in
    /// the alert mask.
    ///
    /// ## Alert Category
    /// [`AlertCategory::PortMappingLog`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    PortmapLog(PortmapLogAlert),
    /// This alert is generated when a DHT node announces to an info-hash on our
    /// DHT node.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Dht`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtAnnounce(DhtAnnounceAlert),
    /// This alert is generated when a DHT node sends a ``get_peers`` message to
    /// our DHT node.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Dht`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtGetPeers(DhtGetPeersAlert),
    /// This alert is posted when the initial DHT bootstrap is done.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Dht`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtBootstrap(DhtBootstrapAlert),
    /// The incoming connection alert is posted every time we successfully accept
    /// an incoming connection, through any mean. The most straight-forward ways
    /// of accepting incoming connections are through the TCP listen socket and
    /// the UDP listen socket for uTP sockets. However, connections may also be
    /// accepted through a Socks5 or i2p listen socket, or via an SSL listen
    /// socket.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Peer`]
    IncomingConnection(IncomingConnectionAlert),
    /// This alert is only posted when requested by the user, by calling [`LtSession::post_torrent_updates()`] on the session.
    /// It contains the torrent status of all torrents that changed since last time this message was posted.
    /// Its category is [`AlertCategory::Status`], but it's not subject to filtering, since it's only manually posted anyway.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::High`]
    StateUpdate(StateUpdateAlert),
    /// The [`Alert::SessionStats`] is posted when the user requests session statistics by
    /// calling post_session_stats() on the session object. This alert does not
    /// have a category, since it's only posted in response to an API call. It
    /// is not subject to the alert_mask filter.
    ///
    /// the ``message()`` member function returns a string representation of the values that
    /// properly match the line returned in ``session_stats_header_alert::message()``.
    ///
    /// this specific output is parsed by tools/parse_session_stats.py
    /// if this is changed, that parser should also be changed
    ///
    /// ## Alert Category
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    // Erm what the sigma? Why is there no category?
    // I don't even think this is actually used but only the alert prio
    // is removed with conditional TORRENT_ABI_VERSION ¯\_(ツ)_/¯
    SessionStats(SessionStatsAlert),
    /// Posted when something fails in the DHT. This is not necessarily a fatal
    /// error, but it could prevent proper operation
    ///
    /// ## Alert Category
    /// [`AlertCategory::Dht`] [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtError(DhtErrorAlert),
    /// This alert is posted as a response to a call to [`LtSession::get_item()`],
    /// specifically the overload for looking up immutable items in the DHT.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Dht`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    DhtImmutableItem(DhtImmutableItemAlert),
    /// This alert is posted as a response to a call to [`LtSession::get_item()`],
    /// specifically the overload for looking up mutable items in the DHT.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Dht`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    DhtMutableItem(DhtMutableItemAlert),
    /// This is posted when a DHT put operation completes. This is useful if the
    /// client is waiting for a put to complete before shutting down for instance.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Dht`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtPut(DhtPutAlert),
    /// This alert is used to report errors in the i2p SAM connection
    ///
    /// ## Alert Category
    /// [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    I2p(I2pAlert),
    /// This alert is generated when we send a get_peers request
    ///
    /// ## Alert Category
    /// [`AlertCategory::Dht`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtOutgoingGetPeers(DhtOutgoingGetPeersAlert),
    /// This alert is posted by some session wide event. Its main purpose is
    /// trouble shooting and debugging. It's not enabled by the default alert
    /// mask and is enabled by the [`AlertCategory::SessionLog`] bit.
    /// Furthermore, it's by default disabled as a build configuration.
    ///
    /// ## Alert Category
    /// [`AlertCategory::SessionLog`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    Log(LogAlert),
    /// Posted if the local service discovery socket fails to start properly.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    LsdError(LsdErrorAlert),
    /// Contains current DHT state. Posted in response to [`LtSession::post_dht_stats()`].
    ///
    /// ## Alert Category
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    // No category, no idea why ¯\_(ツ)_/¯
    DhtStats(DhtStatsAlert),
    /// Debug logging of the DHT when [`AlertCategory::DhtLog`] is set in the alert
    /// mask.
    ///
    /// ## Alert Category
    /// [`AlertCategory::DhtLog`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtLog(DhtLogAlert),
    /// This alert is posted every time a DHT message is sent or received. It is
    /// only posted if the [`AlertCategory::DhtLog`] alert category is
    /// enabled. It contains a verbatim copy of the message.
    ///
    /// ## Alert Category
    /// [`AlertCategory::DhtLog`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtPkt(DhtPktAlert),
    /// Posted when we receive a response to a DHT get_peers request.
    ///
    /// ## Alert Category
    /// [`AlertCategory::DhtOperation`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtGetPeersReply(DhtGetPeersReplyAlert),
    /// This is posted exactly once for every call to session_handle::dht_direct_request.
    /// If the request failed, response() will return a default constructed bdecode_node.
    ///
    /// ## Alert Category
    /// [`AlertCategory::DhtOperation`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtDirectResponse(DhtDirectResponseAlert),
    /// This alert is posted when the session encounters a serious error,
    /// potentially fatal
    ///
    /// ## Alert Category
    /// [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    SessionError(SessionErrorAlert),
    /// Posted in response to a call to [`LtSession::dht_live_nodes()`]. It contains the
    /// live nodes from the DHT routing table of one of the DHT nodes running
    /// locally.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Dht`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtLiveNodes(DhtLiveNodesAlert),
    /// The session_stats_header alert is posted the first time
    /// post_session_stats() is called
    ///
    /// the ``message()`` member function returns a string representation of the
    /// header that properly match the stats values string returned in
    /// ``session_stats_alert::message()``.
    ///
    /// ## Alert Category
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    SessionStatsHeader(SessionStatsHeaderAlert),
    /// Posted as a response to a call to [`LtSession::dht_sample_infohashes()`] with
    /// the information from the DHT response message.
    ///
    /// ## Alert Category
    /// [`AlertCategory::DhtOperation`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtSampleInfohashes(DhtSampleInfohashesAlert),
    /// This alert is posted to indicate to the client that some alerts were
    /// dropped. Dropped meaning that the alert failed to be delivered to the
    /// client. The most common cause of such failure is that the internal alert
    /// queue grew too big (controlled by alert_queue_size).
    ///
    /// ## Alert Category
    /// [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Meta`]
    AlertsDropped(AlertsDroppedAlert),
    /// This alert is posted with SOCKS5 related errors, when a SOCKS5 proxy is
    /// configured. It's enabled with the AlertCategory::Error alert category.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    Socks5(Socks5Alert),

    /// This is a base variant for alerts that are associated with a specific torrent. It contains a handle to the torrent.
    ///
    /// Note that by the time the client receives a TorrentAlert, its handle member may be invalid.
    TorrentAlert(TorrentAlert),
}

macro_rules! type_match_int {
    {
        $value:ident,
        $(
            $name:ident : [$first:ident $(, $rest:ident)*]
        ),* $(,)?
    } => {
        match $value.type_ {
            $(
                ffi::AlertType::$name => {
                    type_match_int!(@wrap $name, $value; $first $(, $rest)*)
                }
            )*
            ffi::AlertType::Unknown => Alert::NotImplemented,
            _ => Alert::NotImplemented,
        }
    };

    (@wrap $name:ident, $value:ident; $wrapper:ident, $next:ident $(, $rest:ident)*) => {
        $wrapper::$next(
            type_match_int!(@wrap $name, $value; $next $(, $rest)*)
        )
    };

    (@wrap $name:ident, $value:ident; $wrapper:ident) => {
        $wrapper::$name(
            paste::paste! {
                [<$name Alert>]($value.alert.cast())
            }
        )
    };
}

macro_rules! type_match {
    {
        $(
            $name:ident : [ $( $rest:ident ),* ]
        ),* $(,)?
    } => {
        impl From<ffi::CastAlertRaw> for Alert {
            fn from(value: ffi::CastAlertRaw) -> Self {
                type_match_int! {
                    value,
                    $(
                        $name: [Alert $(, $rest)*],
                    )*
                }
            }
        }
        paste::paste! {
            $(
                impl [<$name Alert>] {
                    $(
                        fn [<as_ $rest:snake>]<'a>(&'a self) -> [<$rest Raw>]<'a> {
                            [<$rest Raw>]::new(self.0.cast())
                        }
                    )*
                }
            )*
        }
    };
}

type_match! {
    TorrentRemoved: [TorrentAlert],
    ReadPiece: [TorrentAlert],
    FileCompleted: [TorrentAlert],
    FileRenamed: [TorrentAlert],
    FileRenameFailed: [TorrentAlert],
    Performance: [TorrentAlert],
    StateChanged: [TorrentAlert],
    TrackerError: [TorrentAlert, TrackerAlert],
    TrackerWarning: [TorrentAlert, TrackerAlert],
    ScrapeReply: [TorrentAlert, TrackerAlert],
    ScrapeFailed: [TorrentAlert, TrackerAlert],
    TrackerReply: [TorrentAlert, TrackerAlert],
    DhtReply: [TorrentAlert, TrackerAlert],
    TrackerAnnounce: [TorrentAlert, TrackerAlert],
    HashFailed: [TorrentAlert],
    PeerBan: [TorrentAlert, PeerAlert],
    PeerUnsnubbed: [TorrentAlert, PeerAlert],
    PeerSnubbed: [TorrentAlert, PeerAlert],
    PeerError: [TorrentAlert, PeerAlert],
    PeerConnect: [TorrentAlert, PeerAlert],
    PeerDisconnected: [TorrentAlert, PeerAlert],
    InvalidRequest: [TorrentAlert, PeerAlert],
    TorrentFinished: [TorrentAlert],
    PieceFinished: [TorrentAlert],
    RequestDropped: [TorrentAlert, PeerAlert],
    BlockTimeout: [TorrentAlert, PeerAlert],
    BlockFinished: [TorrentAlert, PeerAlert],
    BlockDownloading: [TorrentAlert, PeerAlert],
    UnwantedBlock: [TorrentAlert, PeerAlert],
    StorageMoved: [TorrentAlert],
    StorageMovedFailed: [TorrentAlert],
    TorrentDeleted: [TorrentAlert],
    TorrentDeleteFailed: [TorrentAlert],
    SaveResumeData: [TorrentAlert],
    SaveResumeDataFailed: [TorrentAlert],
    TorrentPaused: [TorrentAlert],
    TorrentResumed: [TorrentAlert],
    TorrentChecked: [TorrentAlert],
    UrlSeed: [TorrentAlert],
    FileError: [TorrentAlert],
    MetadataFailed: [TorrentAlert],
    MetadataReceived: [TorrentAlert],
    UdpError: [],
    ExternalIp: [],
    ListenFailed: [],
    ListenSucceeded: [],
    PortmapError: [],
    Portmap: [],
    PortmapLog: [],
    FastresumeRejected: [TorrentAlert],
    PeerBlocked: [TorrentAlert, PeerAlert],
    DhtAnnounce: [],
    DhtGetPeers: [],
    CacheFlushed: [TorrentAlert],
    LsdPeer: [TorrentAlert, PeerAlert],
    Trackerid: [TorrentAlert, TrackerAlert],
    DhtBootstrap: [],
    TorrentError: [TorrentAlert],
    TorrentNeedCert: [TorrentAlert],
    IncomingConnection: [],
    AddTorrent: [TorrentAlert],
    StateUpdate: [],
    SessionStats: [],
    DhtError: [],
    DhtImmutableItem: [],
    DhtMutableItem: [],
    DhtPut: [],
    I2p: [],
    DhtOutgoingGetPeers: [],
    Log: [],
    TorrentLog: [TorrentAlert],
    PeerLog: [TorrentAlert, PeerAlert],
    LsdError: [],
    DhtStats: [],
    IncomingRequest: [TorrentAlert, PeerAlert],
    DhtLog: [],
    DhtPkt: [],
    DhtGetPeersReply: [],
    DhtDirectResponse: [],
    PickerLog: [TorrentAlert, PeerAlert],
    SessionError: [],
    DhtLiveNodes: [],
    SessionStatsHeader: [],
    DhtSampleInfohashes: [],
    BlockUploaded: [TorrentAlert, PeerAlert],
    AlertsDropped: [],
    Socks5: [],
    FilePrio: [TorrentAlert],
    OversizedFile: [TorrentAlert],
    TorrentConflict: [TorrentAlert],
    PeerInfo: [TorrentAlert],
    FileProgress: [TorrentAlert],
    PieceInfo: [TorrentAlert],
    PieceAvailability: [TorrentAlert],
    TrackerList: [TorrentAlert],
}

impl Alert {
    // pub fn category(&self) -> AlertCategory {
    //     match self {
    //         Alert::TorrentAlert(alert) => match alert {
    //             TorrentAlert::TorrentRemoved(_) => AlertCategory::Status,
    //             TorrentAlert::ReadPiece(_) => AlertCategory::Storage,
    //             TorrentAlert::FileCompleted(_) => AlertCategory::FileProgress,
    //             TorrentAlert::FileRenamed(_) => AlertCategory::Storage,
    //             TorrentAlert::FileRenameFailed(_) => AlertCategory::Storage,
    //             TorrentAlert::Performance(_) => AlertCategory::PerformanceWarning,
    //             TorrentAlert::StateChanged(_) => AlertCategory::Status,
    //             TorrentAlert::HashFailed(_) => AlertCategory::Status,

    //             TorrentAlert::AddTorrent(_) => AlertCategory::Status,
    //             TorrentAlert::TorrentFinished(_) => AlertCategory::Status,
    //             TorrentAlert::SaveResumeData(_) => AlertCategory::Storage,
    //             TorrentAlert::SaveResumeDataFailed(_) => {
    //                 AlertCategory::Storage | AlertCategory::Error
    //             }

    //             TorrentAlert::PeerAlert(alert) => match alert {
    //                 PeerAlert::PeerBan(_) => AlertCategory::Peer,
    //             },
    //             TorrentAlert::TrackerAlert(alert) => match alert {
    //                 TrackerAlert::TrackerError(_) => AlertCategory::Tracker | AlertCategory::Error,
    //                 TrackerAlert::TrackerWarning(_) => {
    //                     AlertCategory::Tracker | AlertCategory::Error
    //                 }
    //                 TrackerAlert::ScrapeReply(_) => AlertCategory::Tracker,
    //                 TrackerAlert::ScrapeFailed(_) => AlertCategory::Tracker | AlertCategory::Error,
    //                 TrackerAlert::TrackerReply(_) => AlertCategory::Tracker,
    //                 TrackerAlert::DhtReply(_) => AlertCategory::Dht | AlertCategory::Tracker,
    //                 TrackerAlert::TrackerAnnounce(_) => AlertCategory::Tracker,
    //             },
    //         },
    //         Alert::StateUpdate(_) => AlertCategory::Status,
    //         Alert::NotImplemented => AlertCategory::empty(),
    //     }
    // }
}

pub enum AlertPriority {
    Normal = 0,
    High,
    Critical,
    Meta,
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
        const PerformanceWarning = 1 << 9;
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
