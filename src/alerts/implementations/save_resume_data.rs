use crate::add_torrent_params::AddTorrentParamsRef;
use crate::alerts::SaveResumeDataAlert;
use crate::ffi::alerts::save_resume_data::ffi::save_resume_data_alert_get_params;
use crate::torrent_handle::TorrentHandle;

impl SaveResumeDataAlert {
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
    pub fn params<'a>(&'a self) -> AddTorrentParamsRef<'a> {
        unsafe { save_resume_data_alert_get_params(self.0) }.into()
    }
}
