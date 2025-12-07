use crate::alerts::FileErrorAlert;
use crate::alerts::operation::Operation;
use crate::errors::LtrsError;
use crate::ffi::alerts::file_error::ffi::{
    file_error_alert_get_error, file_error_alert_get_filename, file_error_alert_get_op,
};
use crate::torrent_handle::TorrentHandle;

impl FileErrorAlert {
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
    pub fn get_filename<'a>(&'a self) -> &'a str {
        unsafe { file_error_alert_get_filename(self.0) }
    }

    #[inline(always)]
    pub fn get_error(&self) -> LtrsError {
        unsafe { file_error_alert_get_error(self.0) }.into()
    }

    #[inline(always)]
    pub fn get_op(&self) -> Operation {
        unsafe { file_error_alert_get_op(self.0) }.into()
    }
}
