use crate::{
    add_torrent_params::AddTorrentParamsRef,
    alerts::AddTorrentAlert,
    errors::LtrsError,
    ffi::alerts::add_torrent::ffi::{
        add_torrent_alert_get_add_torrent_params, add_torrent_alert_get_error,
    },
    torrent_handle::TorrentHandle,
};

impl AddTorrentAlert {
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

    /// This contains copies of the most important fields from the original add_torrent_params
    /// object, passed to add_torrent() or async_add_torrent(). Specifically, these fields are copied:
    ///
    /// * version
    /// * ti
    /// * name
    /// * save_path
    /// * userdata
    /// * tracker_id
    /// * flags
    /// * info_hash
    #[inline(always)]
    pub fn params<'a>(&'a self) -> AddTorrentParamsRef<'a> {
        unsafe { add_torrent_alert_get_add_torrent_params(self.0) }.into()
    }

    #[inline(always)]
    pub fn error(&self) -> LtrsError {
        unsafe { add_torrent_alert_get_error(self.0) }.into()
    }
}
