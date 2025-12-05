use cxx::UniquePtr;

use crate::alerts::types::PieceIndex;
use crate::ffi::torrent_handle::ffi::{
    torrent_handle, torrent_handle_in_session, torrent_handle_info_hashes,
    torrent_handle_read_piece, torrent_handle_save_resume_data,
};
use crate::info_hash::InfoHash;
use crate::torrent_status::TorrentStatus;

// Torrent handles are just a weak pointer so we can just clone them from C++.
pub struct TorrentHandle(UniquePtr<torrent_handle>);

impl TorrentHandle {
    pub(crate) fn from_inner(inner: UniquePtr<torrent_handle>) -> TorrentHandle {
        TorrentHandle(inner)
    }

    /// Returns true if the torrent is in the session. It returns true before session::remove_torrent() is called, and false afterward.
    /// ### Note
    /// This is a blocking function, unlike [`TorrentHandle::is_valid()`] which returns immediately.
    pub fn in_session(&self) -> bool {
        torrent_handle_in_session(&self.0)
    }

    /// [`TorrentHandle::save_resume_data()`] asks libtorrent to generate fast-resume data for
    /// this torrent. The fast resume data (stored in an [`AddTorrentParams`]
    /// struct) can be used to resume a torrent in the next session without
    /// having to check all files for which pieces have been downloaded. It
    /// can also be used to save a .torrent file for a [`TorrentHandle`].
    ///
    /// This operation is asynchronous, [`TorrentHandle::save_resume_data()`] will return
    /// immediately. The resume data is delivered when it's done through a
    /// [`save_resume_data_alert`].
    ///
    /// The operation will fail, and post a save_resume_data_failed_alert
    /// instead, in the following cases:
    ///
    ///	1. The torrent is in the process of being removed.
    ///	2. No torrent state has changed since the last saving of resume
    ///	   data, and the only_if_modified flag is set.
    ///	   metadata (see libtorrent's metadata-from-peers_ extension)
    ///
    /// Note that some counters may be outdated by the time you receive the fast resume data
    ///
    /// When saving resume data because of shutting down, make sure not to
    /// remove_torrent() before you receive the save_resume_data_alert.
    /// There's no need to pause the session or torrent when saving resume
    /// data.
    ///
    /// The paused state of a torrent is saved in the resume data, so pausing
    /// all torrents before saving resume data will all torrents be restored
    /// in a paused state.
    ///
    ///.. note::
    ///   It is typically a good idea to save resume data whenever a torrent
    ///   is completed or paused. If you save resume data for torrents when they are
    ///   paused, you can accelerate the shutdown process by not saving resume
    ///   data again for those torrents. Completed torrents should have their
    ///   resume data saved when they complete and on exit, since their
    ///   statistics might be updated.
    ///
    /// Example code to pause and save resume data for all torrents and wait
    /// for the alerts:
    ///
    /// .. code:: c++
    ///
    ///	extern int outstanding_resume_data; // global counter of outstanding resume data
    ///	std::vector<torrent_handle> handles = ses.get_torrents();
    ///	for (torrent_handle const& h : handles) try
    ///	{
    ///		h.save_resume_data(torrent_handle::only_if_modified);
    ///		++outstanding_resume_data;
    ///	}
    ///	catch (lt::system_error const& e)
    ///	{
    ///		// the handle was invalid, ignore this one and move to the next
    ///	}
    ///
    ///	while (outstanding_resume_data > 0)
    ///	{
    ///		alert const* a = ses.wait_for_alert(seconds(30));
    ///
    ///		// if we don't get an alert within 30 seconds, abort
    ///		if (a == nullptr) break;
    ///
    ///		std::vector<alert*> alerts;
    ///		ses.pop_alerts(&alerts);
    ///
    ///		for (alert* i : alerts)
    ///		{
    ///			if (alert_cast<save_resume_data_failed_alert>(i))
    ///			{
    ///				process_alert(i);
    ///				--outstanding_resume_data;
    ///				continue;
    ///			}
    ///
    ///			save_resume_data_alert const* rd = alert_cast<save_resume_data_alert>(i);
    ///			if (rd == nullptr)
    ///			{
    ///				process_alert(i);
    ///				continue;
    ///			}
    ///
    ///			std::ofstream out((rd->params.save_path
    ///				+ "/" + rd->params.name + ".fastresume").c_str()
    ///				, std::ios_base::binary);
    ///			std::vector<char> buf = write_resume_data_buf(rd->params);
    ///			out.write(buf.data(), buf.size());
    ///			--outstanding_resume_data;
    ///		}
    ///	}
    ///
    ///.. note::
    ///	Note how ``outstanding_resume_data`` is a global counter in this
    ///	example. This is deliberate, otherwise there is a race condition for
    ///	torrents that was just asked to save their resume data, they posted
    ///	the alert, but it has not been received yet. Those torrents would
    ///	report that they don't need to save resume data again, and skipped by
    ///	the initial loop, and thwart the counter otherwise.
    pub fn save_resume_data(&self, flags: ResumeDataFlags) {
        torrent_handle_save_resume_data(&self.0, flags.bits());
    }

    /// This function starts an asynchronous read operation of the specified piece from this torrent.
    /// You must have completed the download of the specified piece before calling this function.
    ///
    /// When the read operation is completed, it is passed back through an alert, [`TorrentAlert::ReadPiece`].
    /// Since this alert is a response to an explicit call, it will always be posted, regardless of the alert mask.
    ///
    /// ## Notes
    /// If you read multiple pieces, the read operations are not guaranteed to finish in
    /// the same order as you initiated them.
    fn read_piece(&self, piece: PieceIndex) {
        torrent_handle_read_piece(&self.0, piece.to_inner());
    }

    pub fn status(&self) -> TorrentStatus {
        // torrent_handle_status(self.0)
        unimplemented!()
    }
    /// Returns the info-hash(es) of the torrent. If this handle is to a torrent that hasn't loaded
    /// yet (for instance by being added) by a URL, the returned value is undefined.
    pub fn info_hashes(&self) -> InfoHash {
        torrent_handle_info_hashes(&self.0).into()
    }
}

// TODO: Check if this is safe
unsafe impl<'a> Send for TorrentHandle {}

impl<'a> std::fmt::Debug for TorrentHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TorrentHandle").finish()
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StatusFlags: u32 {
        const QueryDistributedCopies = 1 << 0;
        const QueryAccurateDownloadCounters = 1 << 1;
        const QueryLastSeenComplete = 1 << 2;
        const QueryPieces = 1 << 3;
        const QueryVerifiedPieces = 1 << 4;
        const QueryTorrentFile = 1 << 5;
        const QueryName = 1 << 6;
        const QuerySavePath = 1 << 7;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ResumeDataFlags: u8 {
        /// The disk cache will be flushed before creating the resume data.
        /// This avoids a problem with file timestamps in the resume data in
        /// case the cache hasn't been flushed yet.
        const FluskDiskCache = 1 << 0;

        /// The resume data will contain the metadata from the torrent file as
        /// well. This is useful for clients that don't keep .torrent files
        /// around separately, or for torrents that were added via a magnet link.
        const SaveInfoDict = 1 << 1;

        /// This flag has the same behavior as the combination of:
        /// [`ResumeDataFlags::IfCountersChanged`] | [`ResumeDataFlags::IfDownloadProgress`] |
        /// [`ResumeDataFlags::IfConfigChanged`] | [`ResumeDataFlags::IfStateChanged`] |
        /// [`ResumeDataFlags::IfMetadataChanged`].
        const OnlyIfModified = 1 << 2;

        /// Save resume data if any counters has changed since the last time
        /// resume data was saved. This includes upload/download counters, active
        /// time counters and scrape data. A torrent that is not paused will have
        /// its active time counters incremented continuously.
        const IfCountersChanged = 1 << 3;

        /// Save the resume data if any blocks have been downloaded since the
        /// last time resume data was saved. This includes:
        /// * checking existing files on disk
        /// * downloading a block from a peer
        const IfDownloadProgress = 1 << 4;

        /// Save the resume data if configuration options changed since last time
        /// the resume data was saved. This includes:
        /// * file- or piece priorities
        /// * upload- and download rate limits
        /// * change max-uploads (unchoke slots)
        /// * change max connection limit
        /// * enable/disable peer-exchange, local service discovery or DHT
        /// * enable/disable apply IP-filter
        /// * enable/disable auto-managed
        /// * enable/disable share-mode
        /// * enable/disable sequential-mode
        /// * files renamed
        /// * storage moved (save_path changed)
        const IfConfigChanged = 1 << 5;

        /// Save the resume data if torrent state has changed since last time the
        /// resume data was saved. This includes:
        /// * upload mode
        /// * paused state
        /// * super-seeding
        /// * seed-mode
        const IfStateChanged = 1 << 6;

        /// Save the resume data if any *metadata* changed since the last time
        /// resume data was saved. This includes:
        /// * add/remove web seeds
        /// * add/remove trackers
        /// * receiving metadata for a magnet link
        const IfMetadataChanged = 1 << 7;
    }
}
