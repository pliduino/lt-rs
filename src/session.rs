use std::mem;

use cxx::UniquePtr;

use crate::{
    add_torrent_params::{AddTorrentParamsIntoPtr, AddTorrentParamsRef},
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
        // Default alert mask covers what most applications need.
        // libtorrent's default only covers status and error; storage, tracker, and peer alerts need opt-in.
        use crate::alerts::AlertCategory;
        let mask = AlertCategory::Error
            | AlertCategory::Status
            | AlertCategory::Storage
            | AlertCategory::Tracker
            | AlertCategory::Connect
            | AlertCategory::Peer
            | AlertCategory::Dht;
        let mut settings = crate::settings_pack::SettingsPack::new();
        settings.set_alert_mask(mask);
        LtSession {
            inner: ffi::lt_create_session_with_settings(settings.inner()),
            alerts: Vec::new(),
        }
    }

    pub fn new_with_settings(settings: &SettingsPack) -> LtSession {
        LtSession {
            inner: ffi::lt_create_session_with_settings(settings.inner()),
            alerts: Vec::new(),
        }
    }

    pub fn add_torrent<'a>(&'a mut self, _params: &AddTorrentParamsRef) -> TorrentHandle {
        unimplemented!()
        // ffi::lt_session_add_torrent(self.inner.pin_mut(),
        // params.inner()).into()
    }

    pub fn remove_torrent<'a>(&'a mut self, handle: &TorrentHandle, options: RemoveFlags) {
        ffi::lt_session_delete_torrent(self.inner.pin_mut(), handle.inner(), options.bits());
    }

    pub fn async_add_torrent<T: AddTorrentParamsIntoPtr>(&mut self, params: &T) {
        unsafe { ffi::lt_session_async_add_torrent(self.inner.pin_mut(), params.as_ptr()) };
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

    /// This functions instructs the session to post the state_update_alert,
    /// containing the status of all torrents whose state changed since the
    /// last time this function was called.
    ///
    /// Only torrents who has the state subscription flag set will be included.
    /// This flag is on by default.
    pub fn post_torrent_updates(&mut self, flags: StatusFlags) {
        ffi::lt_session_post_torrent_updates(self.inner.pin_mut(), flags.bits());
    }

    /// Returns the info-hashes of all torrents currently in the session.
    pub fn get_torrent_hashes(&mut self) -> Vec<crate::info_hash::InfoHash> {
        ffi::lt_session_get_torrent_hashes(self.inner.pin_mut())
            .into_iter()
            .map(Into::into)
            .collect()
    }

    /// Look up a torrent handle by its hex info-hash string (40-char v1 hash).
    /// Returns None if the torrent is not in the session.
    pub fn find_torrent(
        &mut self,
        info_hash_hex: &str,
    ) -> Option<crate::torrent_handle::TorrentHandle> {
        let h = ffi::lt_session_find_torrent(self.inner.pin_mut(), info_hash_hex);
        if h.is_null() {
            return None;
        }
        Some(crate::torrent_handle::TorrentHandle::from_inner(h))
    }

    /// Apply an IP filter to the session. All future connections from blocked
    /// ranges will be rejected.
    pub fn set_ip_filter(&mut self, filter: &crate::ip_filter::IpFilter) {
        ffi::lt_session_set_ip_filter(self.inner.pin_mut(), filter.inner());
    }

    pub fn get_ip_filter(&mut self) -> crate::ip_filter::IpFilter {
        crate::ip_filter::IpFilter::from_inner(ffi::lt_session_get_ip_filter(self.inner.pin_mut()))
    }

    /// Save session state (DHT routing table, settings) to bytes for persistence.
    pub fn save_state(&mut self) -> Vec<u8> {
        ffi::lt_session_save_state(self.inner.pin_mut())
    }

    /// Restore session state from previously saved bytes.
    pub fn load_state(&mut self, data: &[u8]) {
        ffi::lt_session_load_state(self.inner.pin_mut(), data);
    }

    /// Apply updated settings to the live session without restarting.
    /// Safe to call at any time; changes take effect immediately.
    pub fn apply_settings(&mut self, settings: &crate::settings_pack::SettingsPack) {
        ffi::lt_session_apply_settings(self.inner.pin_mut(), settings.inner());
    }

    /// Add a torrent from raw `.torrent` file bytes (bencoded).
    /// Returns `AddTorrentParams` ready for `set_path` + `async_add_torrent`.
    /// Returns an error if the bytes are not a valid torrent.
    pub fn parse_torrent_bytes(
        data: &[u8],
    ) -> Result<crate::add_torrent_params::AddTorrentParams, crate::errors::LtrsError> {
        crate::add_torrent_params::AddTorrentParams::from_torrent_bytes(data)
    }

    /// Marked as unsafe because it takes ownership of the alerts. If the
    /// session pops alerts again the alerts will become invalid.
    ///
    /// As long [`LtSession::pop_alerts()`] is not called again the alerts are
    /// valid
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

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RemoveFlags: u32 {
        /// Delete the files belonging to the torrent from disk.
        /// including the part-file, if there is one
        const DeleteFiles = 1 << 0;
        /// Delete just the part-file associated with this torrent
        const DeletePartfile = 1 << 1;
    }
}

#[cfg(test)]
mod tests {
    // use crate::add_torrent_params::AddTorrentParams;

    // pub use super::*;

    #[test]
    fn add_torrent() {
        // let mut session = LtSession::new();
        // let params = AddTorrentParams::parse_magnet_uri(
        //     "magnet:?xt=urn:btih:a4224d0f8de94e5f9633e8f7e6b49d2fa0a1c9f3&
        // dn=ubuntu-16.04.1-desktop-amd64.iso.torrent", );
        // let handle = session.add_torrent(&params);
        // assert!(handle.is_valid());
    }
}
