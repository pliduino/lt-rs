/// Types of storage allocation used for AddTorrentParams::storage_mode.
#[derive(Debug, Clone, num_enum::IntoPrimitive)]
#[repr(u8)]
pub enum StorageMode {
    /// All pieces will be written to their final position, all files will be
    /// allocated in full when the torrent is first started. This mode minimizes
    /// fragmentation but could be a costly operation.
    StorageModeAllocate,

    /// All pieces will be written to the place where they belong and sparse files
    /// will be used. This is the recommended, and default mode.
    StorageModeSparse,
}
