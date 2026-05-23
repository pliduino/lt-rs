use cxx::UniquePtr;

use crate::ffi::torrent_handle::ffi::*;
use crate::info_hash::InfoHash;
use crate::torrent_status::TorrentStatus;

pub struct TorrentHandle(UniquePtr<torrent_handle>);

impl TorrentHandle {
    pub(crate) fn from_inner(inner: UniquePtr<torrent_handle>) -> TorrentHandle {
        TorrentHandle(inner)
    }

    pub(crate) fn inner(&self) -> &torrent_handle {
        self.0.as_ref().unwrap()
    }

    pub fn in_session(&self) -> bool {
        torrent_handle_in_session(&self.0)
    }

    pub fn make_magnet_uri(&self) -> Result<String, ()> {
        let magnet = torrent_handle_make_magnet_uri(&self.0);
        if magnet.is_empty() {
            return Err(());
        }
        Ok(magnet)
    }

    pub fn info_hashes(&self) -> InfoHash {
        torrent_handle_info_hashes(&self.0).into()
    }

    pub fn status(&self) -> TorrentStatus {
        TorrentStatus::from_inner(torrent_handle_status(&self.0))
    }

    pub fn save_resume_data(&self, flags: ResumeDataFlags) {
        torrent_handle_save_resume_data(&self.0, flags.bits());
    }

    pub fn pause(&self, flags: PauseFlags) {
        torrent_handle_pause(&self.0, flags.bits());
    }

    pub fn resume(&self) {
        torrent_handle_resume(&self.0);
    }

    pub fn force_recheck(&self) {
        torrent_handle_force_recheck(&self.0);
    }

    pub fn force_reannounce(&self) {
        torrent_handle_force_reannounce(&self.0);
    }

    /// Move all torrent files to `save_path`. Posts `StorageMovedAlert` on
    /// success or `StorageMovedFailedAlert` on error. Seeding continues during the move.
    pub fn move_storage(&self, save_path: &str, flags: MoveFlags) {
        torrent_handle_move_storage(&self.0, save_path, flags.bits());
    }

    pub fn save_path(&self) -> String {
        torrent_handle_save_path(&self.0)
    }

    pub fn name(&self) -> String {
        torrent_handle_name(&self.0)
    }

    /// Returns torrent metadata if available (None for unresolved magnet links).
    pub fn torrent_file_info(&self) -> Option<crate::torrent_info::TorrentInfoView> {
        crate::torrent_info::TorrentInfoView::from_handle(self.inner())
    }

    /// Rate limit in bytes/sec. 0 = unlimited.
    pub fn set_upload_limit(&self, limit: i32) {
        torrent_handle_set_upload_limit(&self.0, limit);
    }

    pub fn set_download_limit(&self, limit: i32) {
        torrent_handle_set_download_limit(&self.0, limit);
    }

    pub fn upload_limit(&self) -> i32 {
        torrent_handle_upload_limit(&self.0)
    }

    pub fn download_limit(&self) -> i32 {
        torrent_handle_download_limit(&self.0)
    }

    pub fn set_flags(&self, flags: TorrentFlags) {
        torrent_handle_set_flags(&self.0, flags.bits());
    }

    pub fn unset_flags(&self, flags: TorrentFlags) {
        torrent_handle_unset_flags(&self.0, flags.bits());
    }

    pub fn flags(&self) -> TorrentFlags {
        TorrentFlags::from_bits_truncate(torrent_handle_get_flags(&self.0))
    }

    pub fn prioritize_files(&self, priorities: &[u8]) {
        torrent_handle_prioritize_files(&self.0, priorities);
    }

    pub fn file_priorities(&self) -> Vec<u8> {
        torrent_handle_file_priorities(&self.0)
    }

    pub fn add_tracker(&self, url: &str, tier: i32) {
        torrent_handle_add_tracker(&self.0, url, tier);
    }

    pub fn trackers(&self) -> Vec<AnnounceEntry> {
        let urls = torrent_handle_trackers_url(&self.0);
        let tiers = torrent_handle_trackers_tier(&self.0);
        let fails = torrent_handle_trackers_fails(&self.0);
        let working = torrent_handle_trackers_working(&self.0);
        let complete = torrent_handle_trackers_scrape_complete(&self.0);
        let incomplete = torrent_handle_trackers_scrape_incomplete(&self.0);
        let downloaded = torrent_handle_trackers_scrape_downloaded(&self.0);
        urls.into_iter()
            .enumerate()
            .map(|(i, url)| AnnounceEntry {
                url,
                tier: tiers.get(i).copied().unwrap_or(0),
                fail_count: fails.get(i).copied().unwrap_or(0),
                is_working: working.get(i).copied().unwrap_or(false),
                scrape_complete: complete.get(i).copied().unwrap_or(-1),
                scrape_incomplete: incomplete.get(i).copied().unwrap_or(-1),
                scrape_downloaded: downloaded.get(i).copied().unwrap_or(-1),
            })
            .collect()
    }

    /// Replace all trackers. Assigned tiers 0..n in slice order.
    pub fn replace_trackers(&self, urls: &[&str]) {
        torrent_handle_replace_trackers(&self.0, urls);
    }

    /// Trigger a manual scrape on tracker at `index` (matches `trackers()` order).
    pub fn scrape_tracker(&self, index: i32) {
        torrent_handle_scrape_tracker(&self.0, index);
    }

    // ─── Rename ───────────────────────────────────────────────────────────────

    /// Rename a file inside the torrent. Async: fires `FileRenamed` or
    /// `FileRenameFailed` alert when done.
    pub fn rename_file(&self, file_index: i32, new_name: &str) {
        torrent_handle_rename_file(&self.0, file_index, new_name);
    }

    // ─── Queue position ───────────────────────────────────────────────────────

    /// Queue position within the active download queue. Returns -1 for
    /// seeding / finished / checking torrents that are not queued.
    pub fn queue_position(&self) -> i32 {
        torrent_handle_queue_position(&self.0)
    }
    pub fn queue_position_up(&self) {
        torrent_handle_queue_position_up(&self.0);
    }
    pub fn queue_position_down(&self) {
        torrent_handle_queue_position_down(&self.0);
    }
    pub fn queue_position_top(&self) {
        torrent_handle_queue_position_top(&self.0);
    }
    pub fn queue_position_bottom(&self) {
        torrent_handle_queue_position_bottom(&self.0);
    }

    // ─── Web seeds ────────────────────────────────────────────────────────────

    pub fn add_url_seed(&self, url: &str) {
        torrent_handle_add_url_seed(&self.0, url);
    }
    pub fn remove_url_seed(&self, url: &str) {
        torrent_handle_remove_url_seed(&self.0, url);
    }
    pub fn url_seeds(&self) -> Vec<String> {
        torrent_handle_url_seeds(&self.0)
    }

    // ─── Piece priorities ─────────────────────────────────────────────────────

    /// Get the download priority of a single piece (0 = ignore, 1-6 normal, 7 = max).
    pub fn piece_priority(&self, piece: i32) -> u8 {
        torrent_handle_piece_priority(&self.0, piece)
    }
    /// All piece priorities as a flat `Vec<u8>`.
    pub fn piece_priorities(&self) -> Vec<u8> {
        torrent_handle_piece_priorities(&self.0)
    }
    /// Set piece priorities for all pieces in one call.
    pub fn prioritize_pieces(&self, priorities: &[u8]) {
        torrent_handle_prioritize_pieces(&self.0, priorities);
    }
    /// Set a deadline (milliseconds from now) on a piece for streaming.
    pub fn set_piece_deadline(&self, piece: i32, deadline_ms: i32) {
        torrent_handle_set_piece_deadline(&self.0, piece, deadline_ms);
    }
    pub fn reset_piece_deadline(&self, piece: i32) {
        torrent_handle_reset_piece_deadline(&self.0, piece);
    }
    pub fn clear_piece_deadlines(&self) {
        torrent_handle_clear_piece_deadlines(&self.0);
    }
    /// Helper: set piece 0 and the last piece as urgent (deadline=0) for streaming.
    pub fn set_first_last_piece_priority(&self) {
        let n = crate::ffi::torrent_handle::ffi::lt_torrent_handle_num_pieces(&self.0);
        if n > 0 {
            torrent_handle_set_piece_deadline(&self.0, 0, 0);
            torrent_handle_set_piece_deadline(&self.0, n - 1, 0);
        }
    }

    // ─── Peer info (async) ────────────────────────────────────────────────────

    /// Request peer info asynchronously. Fires a `PeerInfo` alert on next poll.
    pub fn post_peer_info(&self) {
        torrent_handle_post_peer_info(&self.0);
    }
}

impl Clone for TorrentHandle {
    fn clone(&self) -> Self {
        Self(torrent_handle_clone(&self.0))
    }
}

unsafe impl Send for TorrentHandle {}

impl std::fmt::Debug for TorrentHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TorrentHandle").finish()
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StatusFlags: u32 {
        const QueryDistributedCopies        = 1 << 0;
        const QueryAccurateDownloadCounters = 1 << 1;
        const QueryLastSeenComplete         = 1 << 2;
        const QueryPieces                   = 1 << 3;
        const QueryVerifiedPieces           = 1 << 4;
        const QueryTorrentFile              = 1 << 5;
        const QueryName                     = 1 << 6;
        const QuerySavePath                 = 1 << 7;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ResumeDataFlags: u8 {
        const FlushDiskCache     = 1 << 0;
        const SaveInfoDict       = 1 << 1;
        const OnlyIfModified     = 1 << 2;
        const IfCountersChanged  = 1 << 3;
        const IfDownloadProgress = 1 << 4;
        const IfConfigChanged    = 1 << 5;
        const IfStateChanged     = 1 << 6;
        const IfMetadataChanged  = 1 << 7;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PauseFlags: u32 {
        const GracefulPause = 1 << 0;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MoveFlags: u32 {
        const AlwaysReplaceFiles = 0;
        const FailIfExist        = 1;
        const DontReplace        = 2;
    }
}

bitflags::bitflags! {
    /// Subset of lt::torrent_flags_t. See libtorrent/torrent_flags.hpp for the full list.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TorrentFlags: u64 {
        const SeedMode         = 1 << 0;
        const UploadMode       = 1 << 1;
        const ShareMode        = 1 << 2;
        const ApplyIPFilter    = 1 << 3;
        const Paused           = 1 << 4;
        const AutoManaged      = 1 << 5;
        const DuplicateIsError = 1 << 6;
        const UpdateSubscribe  = 1 << 7;
        const SuperSeeding     = 1 << 8;
        const Sequential       = 1 << 9;
        const StopWhenReady    = 1 << 10;
        const OverrideTrackers = 1 << 11;
        const OverrideWebSeeds = 1 << 12;
        const NeedSaveResume   = 1 << 13;
        const DisableDownload  = 1 << 19;
    }
}

/// Mirrors `lt::announce_entry`. One tracker URL with its current state.
#[derive(Debug, Clone)]
pub struct AnnounceEntry {
    pub url: String,
    pub tier: u8,
    pub fail_count: u8,
    pub is_working: bool,
    /// Seeders from last scrape. -1 if unknown.
    pub scrape_complete: i32,
    /// Leechers from last scrape. -1 if unknown.
    pub scrape_incomplete: i32,
    /// Total downloads from last scrape. -1 if unknown.
    pub scrape_downloaded: i32,
}

pub mod file_priority {
    pub const DONT_DOWNLOAD: u8 = 0;
    pub const LOW: u8 = 1;
    pub const DEFAULT: u8 = 4;
    pub const HIGH: u8 = 6;
    pub const MAXIMUM: u8 = 7;
}
