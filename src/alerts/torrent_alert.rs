use crate::{
    add_torrent_params::AddTorrentParams,
    alerts::{ErrorCode, InfoHash, PeerId, PieceIndex, TcpEndpoint, TorrentState, UserData},
    ffi::ffi,
    torrent_handle::TorrentHandle,
};

use cxx::UniquePtr;

pub struct TorrentFinishedAlert(pub(super) *mut ffi::alert);
pub struct AddTorrentAlert(pub(super) *mut ffi::alert);
pub struct StateChangedAlert(pub(super) *mut ffi::alert);
pub struct StateUpdateAlert(pub(super) *mut ffi::alert);
pub struct SaveResumeDataAlert(pub(super) *mut ffi::alert);

pub enum TorrentAlert {
    /// This alert is generated when a torrent switches from being a downloader to a seed.
    /// It will only be generated once per torrent.
    /// It contains a [`TorrentHandle`] to the torrent in question.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    TorrentFinished(TorrentFinishedAlert),
    /// # NOT IMPLEMENTED
    /// The [`TorrentAlert::TorrentRemoved`] is posted whenever a torrent is removed.
    /// Since the torrent handle in its base variant will usually be invalid (since the torrent is already removed)
    /// it has the info_hash as a member, to identify it.
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    ///
    /// ## Notes
    /// Note that the handle remains valid for some time after [`TorrentAlert::TorrentRemoved`] is posted,
    /// as long as some internal libtorrent task (such as an I/O task) refers to it.
    /// Additionally, other alerts like save_resume_data_alert may be posted after [`TorrentAlert::TorrentRemoved`].
    /// To synchronize on whether the torrent has been removed or not, call [`TorrentHandle::in_session`].
    /// This will return true before [`TorrentAlert::TorrentRemoved`] is posted, and false afterward.
    ///
    /// Even though the handle member doesn't point to an existing torrent anymore,
    /// it is still useful for comparing to other handles, which may also no longer point to existing torrents,
    /// but to the same non-existing torrents.
    ///
    /// The [`TorrentHandle`] acts as a weak_ptr, even though its object no longer exists,
    /// it can still compare equal to another weak pointer which points to the same non-existent object.
    TorrentRemoved {},
    /// # NOT IMPLEMENTED
    /// This alert is posted when the asynchronous read operation initiated by a call to [`TorrentHandle::read_piece()`] is completed.
    /// If the read failed, the torrent is paused and an error state is set and the buffer member of the alert is 0.
    /// If successful, buffer points to a buffer containing all the data of the piece. piece is the piece index that was read.
    /// size is the number of bytes that was read.
    ///
    /// If the operation fails, error will indicate what went wrong.
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    ReadPiece {
        error: ErrorCode,
        piece: PieceIndex,
        size: i32,
    },
    /// This alert is always posted when a torrent was attempted to be added and contains the return status of the add operation.
    /// The torrent handle of the new torrent can be found as the handle member in the base class.
    /// If adding the torrent failed, error contains the error code.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    AddTorrent(AddTorrentAlert),
    /// Generated whenever a torrent changes its state.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    StateChanged {
        /// The new state of the torrent
        state: TorrentState,
        /// The previous state of the torrent
        prev_state: TorrentState,
    },
    /// This alert is generated as a response to a [`TorrentHandle::save_resume_data`] request.
    /// It is generated once the disk IO thread is done writing the state for this torrent.
    ///
    /// The params struct is populated with the torrent file whose resume data was saved. It is suitable to be:
    ///
    /// * Added to a session with [`Session::add_torrent()`] or [`Session::async_add_torrent()`]
    /// * Saved to disk with write_resume_data()
    /// * Turned into a magnet link with make_magnet_uri()
    /// * Saved as a .torrent file with write_torrent_file()
    SaveResumeData { params: AddTorrentParams },
    /// This alert is generated instead of [`TorrentAlert::SaveResumeData`] if there was an error generating
    /// the resume data. error describes what went wrong.
    SaveResumeDataFailed { error: ErrorCode },
    /// # NOT IMPLEMENTED
    /// The peer alert is a base variant for alerts that refer to a specific peer.
    /// It includes all the information to identify the peer. i.e. ip and peer-id.
    PeerAlert {
        /// The peer's IP address and port
        endpoint: TcpEndpoint,
        /// The peer ID, if known
        pid: PeerId,
    },
    /// # NOT IMPLEMENTED
    /// This is a base class used for alerts that are associated with a specific tracker.
    /// It is a variant of [`TorrentAlert`] since a tracker is also associated with a specific torrent.
    TrackerAlert {
        /// Endpoint of the listen interface being announced
        local_endpoint: TcpEndpoint,
        tracker_url: String,
    },
}
