use cxx::UniquePtr;
use crate::ffi::ffi::{
    FileStorageWrapper, CreateTorrentWrapper,
    lt_file_storage_new, lt_add_files,
    lt_file_storage_num_files, lt_file_storage_total_size,
    lt_create_torrent_new, lt_create_torrent_add_tracker,
    lt_create_torrent_set_comment, lt_create_torrent_set_creator,
    lt_create_torrent_set_priv, lt_set_piece_hashes, lt_create_torrent_generate,
};

pub struct FileStorage(UniquePtr<FileStorageWrapper>);

impl FileStorage {
    pub fn new() -> Self { FileStorage(lt_file_storage_new()) }

    pub fn add_files(&mut self, path: &str) {
        lt_add_files(self.0.pin_mut(), path);
    }

    pub fn num_files(&self) -> i32 { lt_file_storage_num_files(&self.0) }
    pub fn total_size(&self) -> i64 { lt_file_storage_total_size(&self.0) }
}

impl Default for FileStorage { fn default() -> Self { Self::new() } }
unsafe impl Send for FileStorage {}

pub struct CreateTorrent(UniquePtr<CreateTorrentWrapper>);

impl CreateTorrent {
    /// `piece_size` = 0 lets libtorrent choose automatically.
    pub fn new(fs: &mut FileStorage, piece_size: i32) -> Self {
        CreateTorrent(lt_create_torrent_new(fs.0.pin_mut(), piece_size))
    }

    pub fn add_tracker(&mut self, url: &str, tier: i32) {
        lt_create_torrent_add_tracker(self.0.pin_mut(), url, tier);
    }

    pub fn set_comment(&mut self, comment: &str) {
        lt_create_torrent_set_comment(self.0.pin_mut(), comment);
    }

    pub fn set_creator(&mut self, creator: &str) {
        lt_create_torrent_set_creator(self.0.pin_mut(), creator);
    }

    pub fn set_priv(&mut self, private: bool) {
        lt_create_torrent_set_priv(self.0.pin_mut(), private);
    }

    /// Compute piece hashes — **blocking I/O**, call from `tokio::task::spawn_blocking`.
    /// Returns `Err` if the files cannot be read (e.g. permission denied).
    pub fn set_piece_hashes(&mut self, base_path: &str) -> Result<(), String> {
        let err = lt_set_piece_hashes(self.0.pin_mut(), base_path);
        if err.is_empty() { Ok(()) } else { Err(err) }
    }

    /// Returns raw bencoded .torrent bytes (starts with `d`).
    pub fn generate(&mut self) -> Vec<u8> {
        lt_create_torrent_generate(self.0.pin_mut())
    }
}

unsafe impl Send for CreateTorrent {}
