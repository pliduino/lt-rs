use crate::{
    add_torrent_params::AddTorrentParams,
    ffi::ffi::{self},
    session::LtSession,
    settings_pack::SettingsPack,
    torrent_handle::TorrentHandle,
    torrent_status::TorrentStatus,
};

mod torrent_alert;
mod torrent_state;

pub use torrent_alert::{AddTorrentAlert, TorrentAlert, TorrentFinishedAlert};
pub use torrent_state::TorrentState;

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
    StateUpdate {
        /// Contains the torrent status of all torrents that changed since last time this message was posted.
        /// Note that you can map a torrent status to a specific torrent via its handle member.
        ///
        /// The receiving end is suggested to have all torrents sorted by the [`TorrentHandle`] or hashed by it,
        /// for efficient updates.
        status: Vec<TorrentStatus>,
    },
    // TorrentError,
    // InvalidRequest,
    // TrackerReply,
}

impl From<ffi::CastAlertRaw> for Alert {
    fn from(value: ffi::CastAlertRaw) -> Self {
        match value.type_ {
            ffi::AlertType::TorrentFinished => Alert::TorrentAlert(TorrentAlert::TorrentFinished(
                TorrentFinishedAlert(value.alert),
            )),
            ffi::AlertType::AddTorrent => {
                Alert::TorrentAlert(TorrentAlert::AddTorrent(AddTorrentAlert(value.alert)))
            }
            ffi::AlertType::Unknown => Alert::NotImplemented,
            _ => Alert::NotImplemented,
        }
    }
}

impl Alert {
    pub fn category(&self) -> AlertCategory {
        match self {
            Alert::TorrentAlert(alert) => match alert {
                TorrentAlert::AddTorrent { .. } => AlertCategory::Status,
                TorrentAlert::TorrentFinished(_) => AlertCategory::Status,
                TorrentAlert::TorrentRemoved { .. } => AlertCategory::Status,
                TorrentAlert::ReadPiece { .. } => AlertCategory::Storage,
                TorrentAlert::StateChanged { .. } => AlertCategory::Status,
                TorrentAlert::SaveResumeData { .. } => AlertCategory::Storage,
                TorrentAlert::SaveResumeDataFailed { .. } => {
                    AlertCategory::Storage | AlertCategory::Error
                }
                TorrentAlert::PeerAlert { .. } => todo!(),
                TorrentAlert::TrackerAlert { .. } => todo!(),
            },
            Alert::StateUpdate { .. } => AlertCategory::Status,
            Alert::NotImplemented => AlertCategory::empty(),
        }
    }
}

pub enum TrackerAlert {}

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
