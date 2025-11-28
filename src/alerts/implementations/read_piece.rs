use crate::{alerts::ReadPieceAlert, torrent_handle::TorrentHandle};

impl ReadPieceAlert {
    pub fn handle<'a>(&'a self) -> TorrentHandle<'a> {
        self.as_torrent_alert().handle()
    }

    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    pub fn message(&self) -> String {
        self.as_torrent_alert().message()
    }

    pub fn piece(&self) -> u32 {
        unimplemented!()
    }

    pub fn buffer(&self) -> &[u8] {
        unimplemented!()
    }

    pub fn size(&self) -> i32 {
        unimplemented!()
    }

    pub fn error(&self) -> i32 {
        unimplemented!()
    }
}
