use crate::{
    alerts::StateUpdateAlert,
    ffi::alerts::state_update::ffi::state_update_alert_get_status,
    torrent_status::{TorrentStatus, TorrentStatusVecRef},
};

impl StateUpdateAlert {
    pub fn status<'a>(&'a self) -> TorrentStatusVecRef<'a> {
        unsafe { state_update_alert_get_status(self.0) }.into()
    }
}
