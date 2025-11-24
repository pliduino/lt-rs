use std::{fmt::Display, ptr::NonNull};

use cxx::UniquePtr;

use crate::{
    add_torrent_params::AddTorrentParams,
    ffi::ffi::{self, save_resume_data_alert},
    session::LtSession,
    settings_pack::SettingsPack,
    torrent_handle::TorrentHandle,
    torrent_status::TorrentStatus,
};

mod torrent_alert;
mod torrent_state;

pub use torrent_alert::TorrentAlert;
pub use torrent_state::TorrentState;

type TcpEndpoint = String;
type PeerId = String;
type InfoHash = String;
type UserData = String;

pub type ErrorCode = i32;
pub type PieceIndex = i32;

pub struct AlertList {
    inner: UniquePtr<ffi::AlertListCpp>,
}

pub struct AlertIter<'a> {
    inner: &'a UniquePtr<ffi::AlertListCpp>,
    index: usize,
}

impl AlertList {
    pub(crate) fn new(inner: UniquePtr<ffi::AlertListCpp>) -> AlertList {
        AlertList { inner }
    }

    pub fn iter(&self) -> AlertIter {
        AlertIter {
            inner: &self.inner,
            index: 0,
        }
    }
}

//TODO: Find a better way than manually casting every possible alert type
//TODO: Maybe only iterate over alerts that are enabled?
impl AlertIter<'_> {
    pub fn get_current(&self) -> Option<Alert> {
        let alert = self.inner.get(self.index);

        if alert.is_null() {
            return None;
        }

        match ffi::lt_alert_type(alert) {
            1 => {
                return Some(Alert::TorrentAlert {
                    handle: unsafe { ffi::lt_alert_torrent_finished_handle(alert).into() },
                    alert: TorrentAlert::PeerAlert {
                        endpoint: unsafe { ffi::lt_alert_peer_endpoint(alert).into() },
                        pid: unsafe { ffi::lt_alert_peer_pid(alert).into() },
                    },
                });
            }
            _ => return None,
        }

        // SAFETY: It's safe to cast raw pointers due to LtSession guaranteeing that the alerts are valid.
        // Calling them is safe as the cast would fail if the pointer was invalid in any form
        if let Some(torrent_finished) =
            NonNull::new(unsafe { ffi::lt_alert_torrent_finished_cast(alert) })
        {
            return Some(Alert::TorrentAlert {
                handle: unsafe {
                    ffi::lt_alert_torrent_finished_handle(torrent_finished.as_ptr()).into()
                },
                alert: TorrentAlert::TorrentFinished,
            });
        }

        if let Some(add_torrent) = NonNull::new(unsafe { ffi::lt_alert_add_torrent_cast(alert) }) {
            return Some(Alert::TorrentAlert {
                handle: unsafe { ffi::lt_alert_add_torrent_handle(add_torrent.as_ptr()).into() },
                alert: TorrentAlert::AddTorrent {
                    params: unsafe {
                        ffi::lt_alert_add_torrent_params(add_torrent.as_ptr()).into()
                    },
                    error: unsafe { ffi::lt_alert_add_torrent_error(add_torrent.as_ptr()) },
                },
            });
        }

        if let Some(state_changed) =
            NonNull::new(unsafe { ffi::lt_alert_state_changed_cast(alert) })
        {
            return Some(Alert::TorrentAlert {
                handle: unsafe {
                    ffi::lt_alert_state_changed_handle(state_changed.as_ptr()).into()
                },
                alert: TorrentAlert::StateChanged {
                    state: unsafe {
                        ffi::lt_alert_state_changed_state(state_changed.as_ptr()).into()
                    },
                    prev_state: unsafe {
                        ffi::lt_alert_state_changed_prev_state(state_changed.as_ptr()).into()
                    },
                },
            });
        }

        if let Some(state_update) = NonNull::new(unsafe { ffi::lt_alert_state_update_cast(alert) })
        {
            let raw_status = unsafe { ffi::lt_alert_state_update_status(state_update.as_ptr()) };

            let mut status = vec![];

            for i in 0..raw_status.len() {
                status.push(unsafe { raw_status.get_unchecked(i).into() });
            }

            return Some(Alert::StateUpdate { status });
        }

        if let Some(save_resume_data) =
            NonNull::new(unsafe { ffi::lt_alert_save_resume_data_cast(alert) })
        {
            return Some(Alert::TorrentAlert {
                handle: unsafe {
                    ffi::lt_alert_save_resume_data_handle(save_resume_data.as_ptr()).into()
                },
                alert: TorrentAlert::SaveResumeData {
                    params: unsafe {
                        ffi::lt_alert_save_resume_data_params(save_resume_data.as_ptr()).into()
                    },
                },
            });
        }

        if let Some(save_resume_data_failed) =
            NonNull::new(unsafe { ffi::lt_alert_save_resume_data_failed_cast(alert) })
        {
            return Some(Alert::TorrentAlert {
                handle: unsafe {
                    ffi::lt_alert_save_resume_data_failed_handle(save_resume_data_failed.as_ptr())
                        .into()
                },
                alert: TorrentAlert::SaveResumeDataFailed {
                    error: unsafe {
                        ffi::lt_alert_save_resume_data_failed_error(
                            save_resume_data_failed.as_ptr(),
                        )
                    },
                },
            });
        }

        Some(Alert::NotImplemented)
    }
}

impl Iterator for AlertIter<'_> {
    type Item = Alert;

    fn next(&mut self) -> Option<Self::Item> {
        let alert = self.get_current();
        match alert {
            Some(alert) => {
                self.index += 1;
                Some(alert)
            }
            None => None,
        }
    }
}

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
    TorrentAlert {
        handle: TorrentHandle,
        alert: TorrentAlert,
    },
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

impl Alert {
    pub fn category(&self) -> AlertCategory {
        match self {
            Alert::TorrentAlert { alert, .. } => match alert {
                TorrentAlert::AddTorrent { .. } => AlertCategory::Status,
                TorrentAlert::TorrentFinished => AlertCategory::Status,
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
