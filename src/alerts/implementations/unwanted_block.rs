use crate::alerts::UnwantedBlockAlert;
use crate::alerts::peer_alert::{Endpoint, PeerId};
use crate::ffi::alerts::unwanted_block::ffi::{
    unwanted_block_alert_get_block_index, unwanted_block_alert_get_piece_index,
};
use crate::ffi::ffi::PieceIndex;
use crate::torrent_handle::TorrentHandle;

impl UnwantedBlockAlert {
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
        unsafe { unwanted_block_alert_get_block_index(self.0) }
    }

    #[inline(always)]
    pub fn piece_index(&self) -> PieceIndex {
        unsafe { unwanted_block_alert_get_piece_index(self.0) }
    }
}
