use crate::alerts::InvalidRequestAlert;
use crate::alerts::peer_alert::{Endpoint, PeerId};
use crate::ffi::alerts::invalid_request::ffi::{
    invalid_request_alert_get_peer_interested, invalid_request_alert_get_request,
    invalid_request_alert_get_we_have, invalid_request_alert_get_withheld,
};
use crate::ffi::ffi::PeerRequest;
use crate::torrent_handle::TorrentHandle;

impl InvalidRequestAlert {
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

    /// The request we received from the peer
    #[inline(always)]
    pub fn request(&self) -> PeerRequest {
        unsafe { invalid_request_alert_get_request(self.0) }
    }

    /// True if we have this piece
    #[inline(always)]
    pub fn we_have(&self) -> bool {
        unsafe { invalid_request_alert_get_we_have(self.0) }
    }

    /// True if the peer indicated that it was interested to download before sending the request
    #[inline(always)]
    pub fn peer_interested(&self) -> bool {
        unsafe { invalid_request_alert_get_peer_interested(self.0) }
    }

    /// If this is true, the peer is not allowed to download this piece because of super-seeding rules
    #[inline(always)]
    pub fn withheld(&self) -> bool {
        unsafe { invalid_request_alert_get_withheld(self.0) }
    }
}
