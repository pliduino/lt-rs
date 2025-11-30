use crate::{
    alerts::{StateChangedAlert, TorrentState},
    ffi::alerts::state_changed::ffi::{
        state_changed_alert_get_prev_state, state_changed_alert_get_state,
    },
    torrent_handle::TorrentHandle,
};

pub struct StateChangedValues {
    torrent_handle: TorrentHandle,
    state: TorrentState,
    prev_state: TorrentState,
}

impl StateChangedValues {
    pub fn torrent_handle(&self) -> &TorrentHandle {
        &self.torrent_handle
    }

    pub fn state(&self) -> TorrentState {
        self.state
    }

    pub fn prev_state(&self) -> TorrentState {
        self.prev_state
    }
}
impl<'a> From<TorrentHandle> for StateChangedValues {
    fn from(torrent_handle: TorrentHandle) -> Self {
        Self {
            torrent_handle,
            state: TorrentState::Unknown,
            prev_state: TorrentState::Unknown,
        }
    }
}

impl StateChangedAlert {
    pub fn handle(&self) -> TorrentHandle {
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
