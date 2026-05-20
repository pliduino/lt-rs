use std::fmt::Display;

/// The missing enums are unused enums from versions of libtorrent before 1.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(num_enum::FromPrimitive)]
#[repr(u8)]
pub enum TorrentState {
    /// The torrent has not started its download yet, and is
    /// currently checking existing files.
    CheckingFiles = 1,
    /// The torrent is trying to download metadata from peers.
    DownloadingMetadata = 2,
    /// The torrent is being downloaded.
    Downloading = 3,
    /// The torrent has finished downloading but still doesn't have the entire torrent.
    Finished = 4,
    /// The torrent has finished downloading and is a pure seeder.
    Seeding = 5,
    /// The torrent is currently checking the fast resume data.
    CheckingResumeData = 7,
    /// Returned for any state value not recognized by this version of the library.
    #[num_enum(default)]
    Unknown = 255,
}

impl TorrentState {
    pub(crate) fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::CheckingFiles,
            2 => Self::DownloadingMetadata,
            3 => Self::Downloading,
            4 => Self::Finished,
            5 => Self::Seeding,
            7 => Self::CheckingResumeData,
            _ => Self::Unknown,
        }
    }
}

impl Display for TorrentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
