use crate::{
    alerts::FileRenameFailedAlert, errors::LtrsError,
    ffi::alerts::file_rename_failed::ffi::file_rename_failed_alert_get_error,
};

impl FileRenameFailedAlert {
    pub fn handle<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().handle()
    }

    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    pub fn message(&self) -> String {
        self.as_torrent_alert().message()
    }

    pub fn index(&self) {
        unimplemented!()
    }

    pub fn error(&self) -> LtrsError {
        file_rename_failed_alert_get_error(self).into()
    }
}
