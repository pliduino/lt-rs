use crate::{
    alerts::{TrackerErrorAlert, operation::Operation, protocol_version::ProtocolVersion},
    errors::LtrsError,
    ffi::alerts::tracker_error::ffi::{
        tracker_error_alert_get_error, tracker_error_alert_get_failure_reason,
        tracker_error_alert_get_op, tracker_error_alert_get_times_in_row,
        tracker_error_alert_get_version,
    },
    torrent_handle::TorrentHandle,
};

impl TrackerErrorAlert {
    pub fn handle(&self) -> TorrentHandle {
        self.as_torrent_alert().handle()
    }

    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    pub fn message(&self) -> String {
        self.as_torrent_alert().message()
    }

    pub fn tracker_url<'a>(&'a self) -> &'a str {
        self.as_tracker_alert().tracker_url()
    }

    pub fn local_endpoint(&self) {
        // unsafe { tracker_error_alert_get(self.0) }
    }

    pub fn failure_reason(&self) -> &str {
        unsafe { tracker_error_alert_get_failure_reason(self.0) }
    }

    /// How many times in a row this tracker has failed.
    pub fn times_in_row(&self) -> i32 {
        unsafe { tracker_error_alert_get_times_in_row(self.0) }
    }

    pub fn error(&self) -> LtrsError {
        unsafe { tracker_error_alert_get_error(self.0) }.into()
    }

    pub fn op(&self) -> Operation {
        unsafe { tracker_error_alert_get_op(self.0) }.into()
    }

    /// The bittorrent protocol version that was announced
    pub fn version(&self) -> ProtocolVersion {
        unsafe { tracker_error_alert_get_version(self.0) }.into()
    }
}
