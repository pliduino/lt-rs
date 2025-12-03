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

impl StateChangedAlert {
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
    pub fn state(&self) -> TorrentState {
        let state = unsafe { state_changed_alert_get_state(self.0) };
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                state.into()
            } else {
                unsafe { std::mem::transmute(state) }
            }
        }
    }

    #[inline(always)]
    pub fn prev_state(&self) -> TorrentState {
        let prev_state = unsafe { state_changed_alert_get_prev_state(self.0) };
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                prev_state.into()
            } else {
                unsafe { std::mem::transmute(prev_state) }
            }
        }
    }
}
