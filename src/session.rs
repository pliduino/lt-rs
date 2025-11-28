use std::mem;

use cxx::UniquePtr;

use crate::{
    add_torrent_params::AddTorrentParams,
    alerts::Alert,
    ffi::ffi,
    settings_pack::SettingsPack,
    torrent_handle::{StatusFlags, TorrentHandle},
};

pub struct LtSession {
    inner: UniquePtr<ffi::session>,
    alerts: Vec<Alert>,
}

impl LtSession {
    pub fn new() -> LtSession {
        LtSession {
            inner: ffi::lt_create_session(),
            alerts: Vec::new(),
        }
    }

    pub fn new_with_settings(settings: &SettingsPack) -> LtSession {
        LtSession {
            inner: ffi::lt_create_session_with_settings(settings.inner()),
            alerts: Vec::new(),
        }
    }

    pub fn add_torrent(&mut self, params: &AddTorrentParams) -> TorrentHandle {
        unimplemented!()
        // ffi::lt_session_add_torrent(self.inner.pin_mut(), params.inner()).into()
    }

    pub fn async_add_torrent(&mut self, params: &AddTorrentParams) {
        ffi::lt_session_async_add_torrent(self.inner.pin_mut(), params.inner());
    }

    pub fn pop_alerts(&mut self) {
        let alerts = ffi::lt_session_pop_alerts(self.inner.pin_mut());
        self.alerts.clear();

        for alert in alerts {
            self.alerts.push(alert.into());
        }
    }

    pub fn alerts(&self) -> &Vec<Alert> {
        &self.alerts
    }

    /// This functions instructs the session to post the state_update_alert, containing the status of
    /// all torrents whose state changed since the last time this function was called.
    ///
    /// Only torrents who has the state subscription flag set will be included. This flag is on by default.
    pub fn post_torrent_updates(&mut self, flags: StatusFlags) {
        ffi::lt_session_post_torrent_updates(self.inner.pin_mut(), flags.bits());
    }

    /// Marked as unsafe because it takes ownership of the alerts. If the session pops alerts again
    /// the alerts will become invalid.
    ///
    /// As long [`LtSession::pop_alerts()`] is not called again the alerts are valid
    pub unsafe fn take_alerts(&mut self) -> Vec<Alert> {
        let alerts = mem::replace(&mut self.alerts, Vec::new());
        alerts
    }
}

// TODO: Check if this is safe
unsafe impl Send for LtSession {}

// impl Drop for LtSession {
//     fn drop(&mut self) {
//         ffi::lt_destroy_session(self.inner.pin_mut());
//     }
// }
