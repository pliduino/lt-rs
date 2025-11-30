use crate::{
    alerts::{StateChangedAlert, TorrentState},
    ffi::alerts::state_changed::ffi::{
        state_changed_alert_get_prev_state, state_changed_alert_get_state,
    },
    torrent_handle::TorrentHandle,
};

impl StateChangedAlert {
    pub fn handle<'a>(&'a self) -> TorrentHandle<'a> {
        self.as_torrent_alert().handle()
    }

    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    pub fn message(&self) -> String {
        self.as_torrent_alert().message()
    }

    pub fn state(&self) -> TorrentState {
        unsafe { state_changed_alert_get_state(self.0) }.into()
    }

    pub fn prev_state(&self) -> TorrentState {
        unsafe { state_changed_alert_get_prev_state(self.0) }.into()
    }
}
