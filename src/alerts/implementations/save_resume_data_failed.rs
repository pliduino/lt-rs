use crate::{
    alerts::SaveResumeDataFailedAlert, errors::LtrsError,
    ffi::alerts::save_resume_data_failed::ffi::save_resume_data_failed_alert_get_error,
    torrent_handle::TorrentHandle,
};

impl SaveResumeDataFailedAlert {
    pub fn handle(&self) -> TorrentHandle {
        self.as_torrent_alert().handle()
    }

    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    pub fn message(&self) -> String {
        self.as_torrent_alert().message()
    }

    pub fn error(&self) -> LtrsError {
        unsafe { save_resume_data_failed_alert_get_error(self.0) }.into()
    }
}
