use crate::{
    alerts::{PerformanceAlert, performance_warning::PerformanceWarning},
    ffi::alerts::performance::ffi::performance_alert_get_warning_code,
    torrent_handle::TorrentHandle,
};

impl PerformanceAlert {
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
    pub fn warning_code(&self) -> PerformanceWarning {
        let warning_code = unsafe { performance_alert_get_warning_code(self.0) };
        PerformanceWarning::from_u8(warning_code)
    }
}
