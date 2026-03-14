pub enum DownloadPriority {
    // Don't download the file or piece. Partial pieces may still be downloaded when
    // setting file priorities.
    DontDownload = 0,
    // The lowest priority for files and pieces.
    LowPriority = 1,
    // The default priority for files and pieces.
    DefaultPriority = 4,
    // The highest priority for files and pieces.
    TopPriority = 7,
}
