use crate::{
    alerts::{TrackerWarningAlert, protocol_version::ProtocolVersion},
    ffi::alerts::tracker_warning::ffi::{
        tracker_warning_alert_get_version, tracker_warning_alert_get_warning_message,
    },
    torrent_handle::TorrentHandle,
};

impl TrackerWarningAlert {
    #[inline(always)]
    pub fn handle(&self) -> TorrentHandle {
        self.as_torrent_alert().handle()
    }

    #[inline(always)]
    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    #[inline(always)]
    pub fn message(&self) -> String {
        self.as_torrent_alert().message()
    }

    #[inline(always)]
    pub fn tracker_url<'a>(&'a self) -> &'a str {
        self.as_tracker_alert().tracker_url()
    }

    #[inline(always)]
    pub fn warning_message<'a>(&'a self) -> &'a str {
        unsafe { tracker_warning_alert_get_warning_message(self.0) }
    }

    #[inline(always)]
    pub fn version(&self) -> ProtocolVersion {
        ProtocolVersion::from_u8(unsafe { tracker_warning_alert_get_version(self.0) })
    }
}
