use std::fmt::Display;

/// The missing enums are unused enums from versions of libtorrent before 1.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum TorrentState {
    /// The torrent is in the queue for being checked. But there
    /// currently is another torrent that are being checked.
    /// This torrent will wait for its turn.
    // QueuedForChecking = 0,

    /// The torrent has not started its download yet, and is
    /// currently checking existing files.
    CheckingFiles,
    /// The torrent is trying to download metadata from peers.
    /// This implies the ut_metadata extension is in use.
    DownloadingMetadata,
    /// The torrent is being downloaded. This is the state
    /// most torrents will be in most of the time. The progress
    /// meter will tell how much of the files that has been
    /// downloaded.
    Downloading,
    /// In this state the torrent has finished downloading but
    /// still doesn't have the entire torrent. i.e. some pieces
    /// are filtered and won't get downloaded.
    Finished,
    /// In this state the torrent has finished downloading and
    /// is a pure seeder.
    Seeding,
    /// If the torrent was started in full allocation mode, this
    /// indicates that the (disk) storage for the torrent is
    /// allocated.
    // Allocating = 6,

    /// The torrent is currently checking the fast resume data and
    /// comparing it to the files on disk. This is typically
    /// completed in a fraction of a second, but if you add a
    /// large number of torrents at once, they will queue up.
    CheckingResumeData = 7,

    /// Theoretically this state should never be reached, but
    /// just in case libtorrent adds a new state and this enum is not updated
    /// or libtorrent itself somehow reaches an invalid state.
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    Unknown,
}

impl TorrentState {
    pub(crate) fn from_u8(v: u8) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                v.into()
            } else {
                unsafe { std::mem::transmute(v) }
            }
        }
    }
}

impl Display for TorrentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
