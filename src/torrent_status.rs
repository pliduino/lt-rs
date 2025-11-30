use crate::{alerts::TorrentState, torrent_handle::TorrentHandle};

/// Holds a snapshot of the status of a torrent, as queried by [`TorrentHandle::status()`].
pub struct TorrentStatus {
    pub handle: TorrentHandle,
    pub state: TorrentState,
    pub progress: f64,
}
