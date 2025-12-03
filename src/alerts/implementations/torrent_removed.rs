use crate::{
    alerts::TorrentRemovedAlert,
    ffi::alerts::torrent_removed::ffi::torrent_removed_alert_get_info_hashes, info_hash::InfoHash,
    torrent_handle::TorrentHandle,
};

impl TorrentRemovedAlert {
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

    /// The info-hash of the torrent that was removed.
    #[inline(always)]
    pub fn info_hash(&self) -> InfoHash {
        let hash = unsafe { torrent_removed_alert_get_info_hashes(self.0) };
        hash.into()
    }

    #[inline(always)]
    pub fn userdata(&self) {
        unimplemented!()
    }
}
