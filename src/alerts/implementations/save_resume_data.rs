use crate::{
    add_torrent_params::AddTorrentParamsRef, alerts::SaveResumeDataAlert,
    ffi::alerts::save_resume_data::ffi::save_resume_data_alert_get_params,
    torrent_handle::TorrentHandle,
};

impl SaveResumeDataAlert {
    pub fn handle(&self) -> TorrentHandle {
        self.as_torrent_alert().handle()
    }

    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    pub fn message(&self) -> String {
        self.as_torrent_alert().message()
    }

    pub fn params<'a>(&'a self) -> AddTorrentParamsRef<'a> {
        unsafe { save_resume_data_alert_get_params(self.0) }.into()
    }
}
