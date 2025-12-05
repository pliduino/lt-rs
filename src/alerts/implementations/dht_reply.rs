use crate::alerts::DhtReplyAlert;
use crate::ffi::alerts::dht_reply::ffi::dht_reply_alert_get_num_peers;
use crate::torrent_handle::TorrentHandle;

impl DhtReplyAlert {
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
    pub fn tracker_url<'a>(&'a self) -> &'a str {
        self.as_tracker_alert().tracker_url()
    }

    #[inline(always)]
    pub fn local_endpoint(&self) {
        self.as_tracker_alert().local_endpoint()
    }

    #[inline(always)]
    pub fn num_peers(&self) -> i32 {
        unsafe { dht_reply_alert_get_num_peers(self.0) }
    }
}
