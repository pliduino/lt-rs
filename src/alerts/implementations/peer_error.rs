use crate::alerts::PeerErrorAlert;
use crate::alerts::operation::Operation;
use crate::alerts::peer_alert::{Endpoint, PeerId};
use crate::errors::LtrsError;
use crate::ffi::alerts::peer_error::ffi::{peer_error_alert_get_error, peer_error_alert_get_op};
use crate::torrent_handle::TorrentHandle;

impl PeerErrorAlert {
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
    pub fn op(&self) -> Operation {
        unsafe { peer_error_alert_get_op(self.0) }.into()
    }

    #[inline(always)]
    pub fn error(&self) -> LtrsError {
        unsafe { peer_error_alert_get_error(self.0) }.into()
    }
}
