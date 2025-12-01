use std::marker::PhantomData;

use crate::{
    alerts::{
        AddTorrentAlert, CacheFlushedAlert, FastresumeRejectedAlert, FileCompletedAlert,
        FileErrorAlert, FilePrioAlert, FileProgressAlert, FileRenameFailedAlert, FileRenamedAlert,
        HashFailedAlert, MetadataFailedAlert, MetadataReceivedAlert, OversizedFileAlert,
        PeerInfoAlert, PerformanceAlert, PieceAvailabilityAlert, PieceFinishedAlert,
        PieceInfoAlert, ReadPieceAlert, SaveResumeDataAlert, SaveResumeDataFailedAlert,
        StateChangedAlert, StorageMovedAlert, StorageMovedFailedAlert, TorrentCheckedAlert,
        TorrentConflictAlert, TorrentDeleteFailedAlert, TorrentDeletedAlert, TorrentErrorAlert,
        TorrentFinishedAlert, TorrentLogAlert, TorrentNeedCertAlert, TorrentPausedAlert,
        TorrentRemovedAlert, TorrentResumedAlert, TrackerListAlert, UrlSeedAlert,
        peer_alert::PeerAlert, tracker_alert::TrackerAlert,
    },
    ffi::{
        alerts::torrent_alert::ffi::{
            torrent_alert_handle, torrent_alert_message, torrent_alert_torrent_name,
        },
        ffi::torrent_alert,
    },
    torrent_handle::TorrentHandle,
};

pub(crate) struct TorrentAlertRaw<'a>(*mut torrent_alert, PhantomData<&'a ()>);

impl<'a> TorrentAlertRaw<'a> {
    pub(crate) fn new(alert: *mut torrent_alert) -> TorrentAlertRaw<'a> {
        TorrentAlertRaw(alert, PhantomData)
    }
    pub(crate) fn message(&self) -> String {
        unsafe { torrent_alert_message(self.0) }
    }
    pub(crate) fn torrent_name(&self) -> &'a str {
        unsafe { torrent_alert_torrent_name::<'a>(self.0) }
    }
    pub fn handle(&self) -> TorrentHandle {
        TorrentHandle::from_inner(unsafe { torrent_alert_handle(self.0) })
    }
}

pub enum TorrentAlert {
    /// The [`TorrentAlert::TorrentRemoved`] is posted whenever a torrent is removed. Since
    /// the torrent handle in its base class will usually be invalid (since the torrent
    /// is already removed) it has the info hash as a member, to identify it.
    /// It's posted when the [`AlertCategory::Status`] bit is set in the alert_mask.
    ///
    /// ## Notes
    /// Note that the ``handle`` remains valid for some time after
    /// [`TorrentAlert::TorrentRemoved`] is posted, as long as some internal libtorrent
    /// task (such as an I/O task) refers to it. Additionally, other alerts like
    /// [`TorrentAlert::SaveResumeData`] may be posted after [`TorrentAlert::TorrentRemoved`].
    /// To synchronize on whether the torrent has been removed or not, call
    /// [`TorrentHandle::in_session()`]. This will return true before
    /// [`TorrentAlert::TorrentRemoved`] is posted, and false afterward.
    ///
    /// Even though the ``handle`` member doesn't point to an existing torrent anymore,
    /// it is still useful for comparing to other handles, which may also no
    /// longer point to existing torrents, but to the same non-existing torrents.
    ///
    /// The [`TorrentHandle`] acts as a weak pointer, even though its object no
    /// longer exists, it can still compare equal to another weak pointer which
    /// points to the same non-existent object.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    TorrentRemoved(TorrentRemovedAlert),
    /// This alert is posted when the asynchronous read operation initiated by a call to [`TorrentHandle::read_piece()`] is completed.
    /// If the read failed, the torrent is paused and an error state is set and the buffer member of the alert is 0.
    /// If successful, buffer points to a buffer containing all the data of the piece. piece is the piece index that was read.
    /// size is the number of bytes that was read.
    ///
    /// If the operation fails, error will indicate what went wrong.
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    ReadPiece(ReadPieceAlert),
    // This is posted whenever an individual file completes its download. i.e.
    // All pieces overlapping this file have passed their hash check.
    //
    // ## Alert Category
    // [`AlertCategory::FileProgress`]
    // ## Alert Priority
    // [`AlertPriority::Normal`]
    FileCompleted(FileCompletedAlert),
    /// This is posted as a response to a [`TorrentHandle::rename_file()`] call, if the rename
    /// operation succeeds.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    FileRenamed(FileRenamedAlert),
    /// This is posted as a response to a [`TorrentHandle::rename_file()`] call, if the rename
    /// operation failed.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    FileRenameFailed(FileRenameFailedAlert),
    /// This alert is generated when a limit is reached that might have a negative impact on
    /// upload or download rate performance.
    ///
    /// ## Alert Category
    /// [`AlertCategory::PerformanceWarning`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    Performance(PerformanceAlert),
    /// Generated whenever a torrent changes its state.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::High`]
    StateChanged(StateChangedAlert),
    /// This alert is generated when a finished piece fails its hash check. You can get the handle
    /// to the torrent which got the failed piece and the index of the piece itself from the alert.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    HashFailed(HashFailedAlert),
    /// This alert is generated when a torrent switches from being a downloader to a seed.
    /// It will only be generated once per torrent.
    /// It contains a [`TorrentHandle`] to the torrent in question.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::High`]
    TorrentFinished(TorrentFinishedAlert),
    /// This alert is posted every time a piece completes downloading
    /// and passes the hash check. This alert derives from [`TorrentAlert`],
    /// which contains the [`TorrentHandle`] to the torrent the piece belongs to.
    /// Note that being downloaded and passing the hash check may happen before
    /// the piece is also fully flushed to disk. So [`TorrentHandle::have_piece()`]
    /// may still return false
    ///
    /// ## Alert Category
    /// [`AlertCategory::PieceProgress`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    PieceFinished(PieceFinishedAlert),
    /// The [`TorrentAlert::StorageMoved`] is generated when all the disk IO has
    /// completed and the files have been moved, as an effect of a call to
    /// [`TorrentHandle::move_storage()`]. This is useful to synchronize with the
    /// actual disk. The ``storage_path()`` member return the new path of the
    /// storage.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    StorageMoved(StorageMovedAlert),
    /// The [`TorrentAlert::StorageMovedFailed`] is generated when an attempt to move the storage,
    /// via [`TorrentHandle::move_storage()`], fails.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    StorageMovedFailed(StorageMovedFailedAlert),
    /// This alert is generated when a request to delete the files of a torrent complete.
    ///
    /// This alert is posted in the [`AlertCategory::Storage`] category, and that bit
    /// needs to be set in the alert_mask.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    TorrentDeleted(TorrentDeletedAlert),
    /// This alert is generated when a request to delete the files of a torrent fails.
    /// Just removing a torrent from the session cannot fail
    ///
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    TorrentDeleteFailed(TorrentDeleteFailedAlert),
    /// This alert is generated as a response to a [`TorrentHandle::save_resume_data`] request.
    /// It is generated once the disk IO thread is done writing the state for this torrent.
    ///
    /// The params struct is populated with the torrent file whose resume data was saved. It is suitable to be:
    ///
    /// * Added to a session with [`Session::add_torrent()`] or [`Session::async_add_torrent()`]
    /// * Saved to disk with write_resume_data()
    /// * Turned into a magnet link with make_magnet_uri()
    /// * Saved as a .torrent file with write_torrent_file()
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    SaveResumeData(SaveResumeDataAlert),
    /// This alert is generated instead of [`TorrentAlert::SaveResumeData`] if there was an error generating
    /// the resume data. error describes what went wrong.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    SaveResumeDataFailed(SaveResumeDataFailedAlert),
    /// This alert is generated as a response to a [`TorrentHandle::pause`] request. It is
    /// generated once all disk IO is complete and the files in the torrent have been closed.
    /// This is useful for synchronizing with the disk.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::High`]
    TorrentPaused(TorrentPausedAlert),
    /// This alert is generated as a response to a [`TorrentHandle::resume()`] request. It is
    /// generated when a torrent goes from a paused state to an active state.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::High`]
    TorrentResumed(TorrentResumedAlert),
    /// This alert is posted when a torrent completes checking. i.e. when it transitions
    /// out of the ``checking files`` state into a state where it is ready to start downloading
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::High`]
    TorrentChecked(TorrentCheckedAlert),
    /// This alert is generated when a HTTP seed name lookup fails.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Peer`] [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    UrlSeed(UrlSeedAlert),
    /// If the storage fails to read or write files that it needs access to, this alert is
    /// generated and the torrent is paused.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::High`]
    FileError(FileErrorAlert),
    /// This alert is generated when the metadata has been completely received and the info-hash
    /// failed to match it. i.e. the metadata that was received was corrupt. libtorrent will
    /// automatically retry to fetch it in this case. This is only relevant when running a
    /// torrent-less download, with the metadata extension provided by libtorrent.
    ///
    /// ## Alert Category
    ///  [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    MetadataFailed(MetadataFailedAlert),

    /// This alert is generated when the metadata has been completely received and the torrent
    /// can start downloading. It is not generated on torrents that are started with metadata, but
    /// only those that needs to download it from peers (when utilizing the libtorrent extension).
    ///
    /// There are no additional data members in this alert.
    ///
    /// Typically, when receiving this alert, you would want to save the torrent file in order
    /// to load it back up again when the session is restarted.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    MetadataReceived(MetadataReceivedAlert),

    /// This alert is generated when a fast resume file has been passed to
    /// add_torrent() but the files on disk did not match the fast resume file.
    /// The error_code explains the reason why the resume file was rejected.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    FastresumeRejected(FastresumeRejectedAlert),

    /// This alert is posted approximately once every second, and it contains
    /// byte counters of most statistics that's tracked for torrents. Each active
    /// torrent posts these alerts regularly.
    /// This alert has been superseded by calling ``post_torrent_updates()``
    /// regularly on the session object. This alert will be removed
    ///
    /// /// ## Alert Category
    /// [`AlertCategory::Stats`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    //Stats(StatsAlert),

    /// This alert is posted when the disk cache has been flushed for a specific
    /// torrent as a result of a call to [`TorrentHandle::flush_cache()`]. This
    /// alert belongs to the [`AlertCategory::Storage`] category, which must be
    /// enabled to let this alert through. The alert is also posted when removing
    /// a torrent from the session, once the outstanding cache flush is complete
    /// and the torrent does no longer have any files open.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    /// ## Alert Priority
    /// [`AlertPriority::High`]
    CacheFlushed(CacheFlushedAlert),

    /// This is posted whenever a torrent is transitioned into the error state.
    /// If the error code is duplicate_torrent (error_code_enum) error, it suggests two magnet
    /// links ended up resolving to the same hybrid torrent. For more details,
    /// see BitTorrent-v2-torrents_.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`] [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::High`]
    TorrentError(TorrentErrorAlert),

    /// This is always posted for SSL torrents. This is a reminder to the client that
    /// the torrent won't work unless [`TorrentHandle::set_ssl_certificate()`] is called with
    /// a valid certificate. Valid certificates MUST be signed by the SSL certificate
    /// in the .torrent file.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    TorrentNeedCert(TorrentNeedCertAlert),
    /// This alert is always posted when a torrent was attempted to be added and contains the return status of the add operation.
    /// The torrent handle of the new torrent can be found as the handle member in the base class.
    /// If adding the torrent failed, error contains the error code.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    AddTorrent(AddTorrentAlert),
    /// This alert is posted by torrent wide events. It's meant to be used for
    /// trouble shooting and debugging. It's not enabled by the default alert
    /// mask and is enabled by the [`AlertCategory::TorrentLog`] bit. By
    /// default it is disabled as a build configuration.
    ///
    /// ## Alert Category
    /// [`AlertCategory::TorrentLog`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    TorrentLog(TorrentLogAlert),
    /// Posted when a prioritize_files() or file_priority() update of the file
    /// priorities complete, which requires a round-trip to the disk thread.
    ///
    /// If the disk operation fails this alert won't be posted, but a
    /// [`Alert::FileError`] is posted instead, and the torrent is stopped.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    FilePrio(FilePrioAlert),
    /// This alert may be posted when the initial checking of resume data and files
    /// on disk (just existence, not piece hashes) completes. If a file belonging
    /// to the torrent is found on disk, but is larger than the file in the
    /// torrent, that's when this alert is posted.
    /// the client may want to call truncate_files() in that case, or perhaps
    /// interpret it as a sign that some other file is in the way, that shouldn't
    /// be overwritten.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Storage`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    OversizedFile(OversizedFileAlert),
    /// This alert is posted when two separate torrents (magnet links) resolve to
    /// the same torrent, thus causing the same torrent being added twice. In
    /// that case, both torrents enter an error state with ``duplicate_torrent``
    /// as the error code. This alert is posted containing the metadata. For more
    /// information, see BitTorrent-v2-torrents_.
    /// The torrent this alert originated from was the one that downloaded the
    /// metadata (i.e. the `handle` member).
    ///
    /// ## Alert Category
    /// [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::High`]
    TorrentConflict(TorrentConflictAlert),
    /// Posted when [`TorrentHandle::post_peer_info()`] is called
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    PeerInfo(PeerInfoAlert),
    /// Posted when [`TorrentHandle::post_file_progress()`] is called
    ///
    /// ## Alert Category
    /// [`AlertCategory::FileProgress`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    FileProgress(FileProgressAlert),
    /// Posted when [`TorrentHandle::post_download_queue()`] is called
    ///
    /// ## Alert Category
    /// [`AlertCategory::PieceProgress`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    PieceInfo(PieceInfoAlert),
    /// Posted when [`TorrentHandle::post_piece_availability()`] is called
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    PieceAvailability(PieceAvailabilityAlert),
    /// Posted when [`TorrentHandle::post_trackers()`] is called
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    TrackerList(TrackerListAlert),

    /// The peer alert is a base variant for alerts that refer to a specific peer.
    /// It includes all the information to identify the peer. i.e. ip and peer-id.
    PeerAlert(PeerAlert),
    /// This is a base variant used for alerts that are associated with a specific tracker.
    /// It is a variant of [`TorrentAlert`] since a tracker is also associated with a specific torrent.
    TrackerAlert(TrackerAlert),
}
