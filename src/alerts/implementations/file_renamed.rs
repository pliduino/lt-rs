use crate::{
    alerts::FileRenamedAlert,
    ffi::alerts::file_renamed::ffi::{
        file_renamed_alert_get_new_name, file_renamed_alert_get_old_name,
    },
    torrent_handle::TorrentHandle,
};

impl FileRenamedAlert {
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

    /// The file index within the torrent that was renamed.
    #[inline(always)]
    pub fn index(&self) -> i32 {
        unsafe {
            crate::ffi::alerts::file_renamed::ffi::file_renamed_alert_get_index(self.0)
        }
    }

    pub fn old_name<'a>(&'a self) -> &'a str {
        unsafe { file_renamed_alert_get_old_name(self.0) }
    }

    pub fn new_name<'a>(&'a self) -> &'a str {
        unsafe { file_renamed_alert_get_new_name(self.0) }
    }
}
