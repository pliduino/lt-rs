use std::fmt::Display;

/// The missing enums are unused enums from versions of libtorrent before 1.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TorrentState {
    CheckingFiles = 1,
    DownloadingMetadata = 2,
    Downloading = 3,
    Finished = 4,
    Seeding = 5,
    CheckingResumeData = 7,

    /// Shouldn't ever be built, if it does it's a bug, please report it.
    /// This is a fallback type to avoid having a Result in the API due to converting a raw integer
    /// to an enum.
    Unknown = 255,
}

impl From<u8> for TorrentState {
    fn from(value: u8) -> Self {
        match value {
            1 => TorrentState::CheckingFiles,
            2 => TorrentState::DownloadingMetadata,
            3 => TorrentState::Downloading,
            4 => TorrentState::Finished,
            5 => TorrentState::Seeding,
            7 => TorrentState::CheckingResumeData,
            _ => TorrentState::Unknown,
        }
    }
}

impl Display for TorrentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
