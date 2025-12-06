use crate::alerts::BlockDownloadingAlert;
use crate::alerts::peer_alert::{Endpoint, PeerId};
use crate::ffi::alerts::block_downloading::ffi::{
    block_downloading_alert_get_block_index, block_downloading_alert_get_piece_index,
};
use crate::ffi::ffi::PieceIndex;
use crate::torrent_handle::TorrentHandle;

impl BlockDownloadingAlert {
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
    pub fn pid(&self) -> PeerId {
        self.as_peer_alert().peer_id()
    }

    #[inline(always)]
    pub fn endpoint(&self) -> Endpoint {
        self.as_peer_alert().endpoint()
    }

    #[inline(always)]
    pub fn block_index(&self) -> i32 {
        unsafe { block_downloading_alert_get_block_index(self.0) }
    }

    #[inline(always)]
    pub fn piece_index(&self) -> PieceIndex {
        unsafe { block_downloading_alert_get_piece_index(self.0) }
    }
}
