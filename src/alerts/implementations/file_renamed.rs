use crate::{
    alerts::FileRenamedAlert,
    ffi::alerts::file_renamed::ffi::{
        file_renamed_alert_get_new_name, file_renamed_alert_get_old_name,
    },
};

impl FileRenamedAlert {
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

    pub fn old_name<'a>(&'a self) -> &'a str {
        file_renamed_alert_get_old_name(self)
    }

    pub fn new_name<'a>(&'a self) -> &'a str {
        file_renamed_alert_get_new_name(self)
    }
}
