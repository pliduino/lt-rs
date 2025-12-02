/// These constants are used to identify the operation that failed, causing a
/// peer to disconnect
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum Operation {
    /// The error was unexpected and it is unknown which operation caused it
    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    Unknown,
    /// This is used when the bittorrent logic
    /// determines to disconnect
    Bittorrent,

    /// A call to iocontrol failed
    Iocontrol,

    /// A call to ``getpeername()`` failed (querying the remote IP of a
    /// connection)
    Getpeername,

    /// A call to getname failed (querying the local IP of a
    /// connection)
    Getname,

    /// An attempt to allocate a receive buffer failed
    AllocRecvbuf,

    /// An attempt to allocate a send buffer failed
    AllocSndbuf,

    /// Writing to a file failed
    FileWrite,

    /// Reading from a file failed
    Fileead,

    /// A non-read and non-write file operation failed
    File,

    /// A socket write operation failed
    SockWrite,

    /// A socket read operation failed
    SockRead,

    /// A call to open(), to create a socket socket failed
    SockOpen,

    /// A call to bind() on a socket failed
    SockBind,

    /// An attempt to query the number of bytes available to read from a socket
    /// failed
    Available,

    /// A call related to bittorrent protocol encryption failed
    Encryption,

    /// An attempt to connect a socket failed
    Connect,

    /// Establishing an SSL connection failed
    SslHandshake,

    /// A connection failed to satisfy the bind interface setting
    GetInterface,

    /// A call to listen() on a socket
    SockListen,

    /// A call to the ioctl to bind a socket to a specific network device or
    /// adapter
    SockBindToDevice,

    /// A call to accept() on a socket
    SockAccept,

    /// Convert a string into a valid network address
    ParseAddress,

    /// Enumeration network devices or adapters
    EnumIf,

    /// Invoking stat() on a file
    FileStat,

    /// Copying a file
    FileCopy,

    /// Allocating storage for a file
    FileFallocate,

    /// Creating a hard link
    FileHardLink,

    /// Removing a file
    FileRemove,

    /// Renaming a file
    FileRename,

    /// Opening a file
    FileOpen,

    /// Creating a directory
    Mkdir,

    /// Check fast resume data against files on disk
    CheckResume,

    /// An unknown exception
    Exception,

    /// Allocate space for a piece in the cache
    AllocCachePiece,

    /// Move a part-file
    PartfileMove,

    /// Read from a part file
    PartfileRead,

    /// Write to a part-file
    PartfileWrite,

    /// A hostname lookup
    HostnameLookup,

    /// Create or read a symlink
    Symlink,

    /// Handshake with a peer or server
    Handshake,

    /// Set socket option
    SockOption,

    /// Enumeration of network routes
    EnumRoute,

    /// Moving read/write position in a file, operation_t::hostname_lookup
    FileSeek,

    /// An async wait operation on a timer
    Timer,

    /// Call to mmap() (or windows counterpart)
    FileMmap,

    /// Call to ftruncate() (or SetEndOfFile() on windows)
    FileTruncate,
}

impl Operation {
    pub unsafe fn from_u8(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
