use crate::alerts::FileCompletedAlert;

impl FileCompletedAlert {
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
}
