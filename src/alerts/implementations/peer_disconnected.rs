use crate::alerts::PeerDisconnectedAlert;
use crate::alerts::operation::Operation;
use crate::alerts::peer_alert::{Endpoint, PeerId};
use crate::alerts::types::{CloseReason, SocketType};
use crate::errors::LtrsError;
use crate::ffi::alerts::peer_disconnected::ffi::{
    peer_disconnected_alert_get_error, peer_disconnected_alert_get_op,
    peer_disconnected_alert_get_reason, peer_disconnected_alert_get_socket_type,
};
use crate::torrent_handle::TorrentHandle;

impl PeerDisconnectedAlert {
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
    pub fn socket_type(&self) -> SocketType {
        SocketType::from_u8(unsafe { peer_disconnected_alert_get_socket_type(self.0) })
    }

    #[inline(always)]
    pub fn op(&self) -> Operation {
        unsafe { peer_disconnected_alert_get_op(self.0) }.into()
    }

    #[inline(always)]
    pub fn error(&self) -> LtrsError {
        unsafe { peer_disconnected_alert_get_error(self.0) }.into()
    }

    #[inline(always)]
    pub fn reason(&self) -> CloseReason {
        CloseReason::from_u16(unsafe { peer_disconnected_alert_get_reason(self.0) })
    }
}
