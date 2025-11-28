use cxx::UniquePtr;

use crate::{alerts::TorrentState, torrent_handle::TorrentHandle};

/// Holds a snapshot of the status of a torrent, as queried by [`TorrentHandle::status()`].
#[derive(Debug)]
pub struct TorrentStatus<'a> {
    pub handle: TorrentHandle<'a>,
    pub state: TorrentState,
    pub progress: f64,
}
