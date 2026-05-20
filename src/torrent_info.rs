use crate::ffi::torrent_handle::ffi::*;
use crate::ffi::torrent_handle::ffi::torrent_handle;

pub struct FileEntry {
    pub path:   String,
    pub size:   i64,
}

pub struct TorrentInfoView {
    pub name:         String,
    pub total_size:   i64,
    pub num_files:    i32,
    pub num_pieces:   i32,
    pub piece_length: i32,
    pub files:        Vec<FileEntry>,
}

impl TorrentInfoView {
    /// Returns `None` if the torrent has no metadata yet (magnet not resolved).
    pub(crate) fn from_handle(h: &torrent_handle) -> Option<Self> {
        let name = lt_torrent_handle_name(h);
        if name.is_empty() { return None; }
        let paths = lt_torrent_handle_file_paths(h);
        let sizes = lt_torrent_handle_file_sizes(h);
        let files = paths.into_iter().zip(sizes)
            .map(|(path, size)| FileEntry { path, size })
            .collect();
        Some(TorrentInfoView {
            name,
            total_size:   lt_torrent_handle_total_size(h),
            num_files:    lt_torrent_handle_num_files(h),
            num_pieces:   lt_torrent_handle_num_pieces(h),
            piece_length: lt_torrent_handle_piece_length(h),
            files,
        })
    }
}
