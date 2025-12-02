use crate::{
    alerts::{PerformanceAlert, performance_warning::PerformanceWarning},
    ffi::alerts::performance::ffi::performance_alert_get_warning_code,
    torrent_handle::TorrentHandle,
};

impl PerformanceAlert {
    pub fn handle(&self) -> TorrentHandle {
        self.as_torrent_alert().handle()
    }

    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    pub fn message(&self) -> String {
        self.as_torrent_alert().message()
    }

    pub fn warning_code(&self) -> PerformanceWarning {
        let warning_code = unsafe { performance_alert_get_warning_code(self.0) };
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                warning_code.into()
            } else {
                unsafe { std::mem::transmute(warning_code) }
            }
        }
    }
}
