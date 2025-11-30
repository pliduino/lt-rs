use crate::{
    alerts::{PerformanceAlert, performance_warning::PerformanceWarning},
    torrent_handle::TorrentHandle,
};

impl PerformanceAlert {
    pub fn handle<'a>(&'a self) -> TorrentHandle<'a> {
        self.as_torrent_alert().handle()
    }

    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    pub fn message<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().message()
    }

    pub fn warning_code(&self) -> PerformanceWarning {
        PerformanceWarning::from(unsafe { ffi::performance_alert_get_warning_code(self.as_ptr()) })
    }
}
