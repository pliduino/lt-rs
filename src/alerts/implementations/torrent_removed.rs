use crate::{
    alerts::TorrentRemovedAlert,
    ffi::alerts::torrent_removed_alert::ffi::torrent_removed_alert_get_info_hashes,
    info_hash::InfoHash, torrent_handle::TorrentHandle,
};

impl TorrentRemovedAlert {
    pub fn handle<'a>(&'a self) -> TorrentHandle<'a> {
        self.as_torrent_alert().handle()
    }

    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    pub fn message(&self) -> String {
        self.as_torrent_alert().message()
    }

    pub fn info_hashes(&self) -> InfoHash {
        let hash = unsafe { torrent_removed_alert_get_info_hashes(self.0) };
        hash.into()
    }

    pub fn userdata(&self) -> Option<&String> {
        unimplemented!()
    }
}
