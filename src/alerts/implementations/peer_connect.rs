use crate::alerts::PeerConnectAlert;
use crate::alerts::peer_alert::{Endpoint, PeerId};
use crate::alerts::types::{Direction, SocketType};
use crate::ffi::alerts::peer_connect::ffi::{
    peer_connect_alert_get_direction, peer_connect_alert_get_socket_type,
};
use crate::torrent_handle::TorrentHandle;

impl PeerConnectAlert {
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
    pub fn direction(&self) -> Direction {
        Direction::from_u8(unsafe { peer_connect_alert_get_direction(self.0) })
    }

    #[inline(always)]
    pub fn socket_type(&self) -> SocketType {
        SocketType::from_u8(unsafe { peer_connect_alert_get_socket_type(self.0) })
    }
}
