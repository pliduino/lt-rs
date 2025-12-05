use crate::{
    alerts::{
        PeerBanAlert,
        peer_alert::{Endpoint, PeerId},
    },
    torrent_handle::TorrentHandle,
};

impl PeerBanAlert {
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
}
