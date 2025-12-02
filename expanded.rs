#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
pub mod add_torrent_params {
    use std::fmt::Debug;
    use cxx::UniquePtr;
    use crate::{ffi::ffi, info_hash::InfoHash};
    pub struct AddTorrentParams {
        inner: UniquePtr<ffi::add_torrent_params>,
    }
    impl Debug for AddTorrentParams {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AddTorrentParams").finish()
        }
    }
    impl AddTorrentParams {
        pub(super) fn inner(&self) -> &ffi::add_torrent_params {
            &self.inner
        }
        pub fn parse_magnet_uri(magnet_uri: &str) -> AddTorrentParams {
            AddTorrentParams {
                inner: ffi::lt_parse_magnet_uri(magnet_uri),
            }
        }
        pub fn set_path(&mut self, path: &str) {
            ffi::lt_set_add_torrent_params_path(self.inner.pin_mut(), path);
        }
        pub fn get_info_hash(&self) -> InfoHash {
            ffi::lt_add_torrent_params_info_hash(&self.inner).into()
        }
        pub fn write_resume_data_buf(&self) -> Vec<u8> {
            ffi::lt_write_resume_data_buf(&self.inner)
        }
        pub fn load_resume_data(buf: &[u8]) -> AddTorrentParams {
            AddTorrentParams {
                inner: ffi::lt_read_resume_data(buf),
            }
        }
    }
    impl From<UniquePtr<ffi::add_torrent_params>> for AddTorrentParams {
        fn from(inner: UniquePtr<ffi::add_torrent_params>) -> AddTorrentParams {
            AddTorrentParams { inner }
        }
    }
    unsafe impl Send for AddTorrentParams {}
}
pub mod alerts {
    use crate::{
        alerts::{peer_alert::PeerAlert, tracker_alert::TrackerAlert},
        ffi::ffi::self,
    };
    pub mod implementations {
        pub mod add_torrent_alert {
            use crate::{
                add_torrent_params::AddTorrentParams, alerts::AddTorrentAlert,
                errors::LtrsError, torrent_handle::TorrentHandle,
            };
            impl AddTorrentAlert {
                pub fn handle(&self) -> TorrentHandle {
                    self.as_torrent_alert().handle()
                }
                pub fn torrent_name<'a>(&'a self) -> &'a str {
                    self.as_torrent_alert().torrent_name()
                }
                pub fn message(&self) -> String {
                    self.as_torrent_alert().message()
                }
                /// This contains copies of the most important fields from the original add_torrent_params
                /// object, passed to add_torrent() or async_add_torrent(). Specifically, these fields are copied:
                ///
                /// * version
                /// * ti
                /// * name
                /// * save_path
                /// * userdata
                /// * tracker_id
                /// * flags
                /// * info_hash
                pub fn params(&self) -> AddTorrentParams {
                    ::core::panicking::panic("not implemented")
                }
                pub fn error(&self) -> LtrsError {
                    ::core::panicking::panic("not implemented")
                }
            }
        }
        pub mod file_completed {
            use crate::{alerts::FileCompletedAlert, torrent_handle::TorrentHandle};
            impl FileCompletedAlert {
                pub fn handle(&self) -> TorrentHandle {
                    self.as_torrent_alert().handle()
                }
                pub fn torrent_name<'a>(&'a self) -> &'a str {
                    self.as_torrent_alert().torrent_name()
                }
                pub fn message(&self) -> String {
                    self.as_torrent_alert().message()
                }
                pub fn index(&self) {
                    ::core::panicking::panic("not implemented")
                }
            }
        }
        pub mod file_rename_failed {
            use crate::{
                alerts::FileRenameFailedAlert, errors::LtrsError,
                ffi::alerts::file_rename_failed::ffi::file_rename_failed_alert_get_error,
                torrent_handle::TorrentHandle,
            };
            impl FileRenameFailedAlert {
                pub fn handle(&self) -> TorrentHandle {
                    self.as_torrent_alert().handle()
                }
                pub fn torrent_name<'a>(&'a self) -> &'a str {
                    self.as_torrent_alert().torrent_name()
                }
                pub fn message(&self) -> String {
                    self.as_torrent_alert().message()
                }
                pub fn index(&self) {
                    ::core::panicking::panic("not implemented")
                }
                pub fn error(&self) -> LtrsError {
                    unsafe { file_rename_failed_alert_get_error(self.0) }.into()
                }
            }
        }
        pub mod file_renamed {
            use crate::{
                alerts::FileRenamedAlert,
                ffi::alerts::file_renamed::ffi::{
                    file_renamed_alert_get_new_name, file_renamed_alert_get_old_name,
                },
                torrent_handle::TorrentHandle,
            };
            impl FileRenamedAlert {
                pub fn handle(&self) -> TorrentHandle {
                    self.as_torrent_alert().handle()
                }
                pub fn torrent_name<'a>(&'a self) -> &'a str {
                    self.as_torrent_alert().torrent_name()
                }
                pub fn message(&self) -> String {
                    self.as_torrent_alert().message()
                }
                pub fn index(&self) {
                    ::core::panicking::panic("not implemented")
                }
                pub fn old_name<'a>(&'a self) -> &'a str {
                    unsafe { file_renamed_alert_get_old_name(self.0) }
                }
                pub fn new_name<'a>(&'a self) -> &'a str {
                    unsafe { file_renamed_alert_get_new_name(self.0) }
                }
            }
        }
        pub mod performance {
            use crate::{
                alerts::{PerformanceAlert, performance_warning::PerformanceWarning},
                ffi::alerts::performance::ffi::performance_alert_get_warning_code,
                torrent_handle::TorrentHandle,
            };
            impl PerformanceAlert {
                pub fn handle(&self) -> TorrentHandle {
                    self.as_torrent_alert().handle()
                }
                pub fn torrent_name<'a>(&'a self) -> &'a str {
                    self.as_torrent_alert().torrent_name()
                }
                pub fn message(&self) -> String {
                    self.as_torrent_alert().message()
                }
                pub fn warning_code(&self) -> PerformanceWarning {
                    let warning_code = unsafe {
                        performance_alert_get_warning_code(self.0)
                    };
                    unsafe { std::mem::transmute(warning_code) }
                }
            }
        }
        pub mod read_piece {
            use crate::{
                alerts::ReadPieceAlert, errors::LtrsError,
                ffi::alerts::read_piece::ffi::{
                    read_piece_alert_get_error, read_piece_alert_get_size,
                },
                torrent_handle::TorrentHandle,
            };
            impl ReadPieceAlert {
                pub fn handle(&self) -> TorrentHandle {
                    self.as_torrent_alert().handle()
                }
                pub fn torrent_name<'a>(&'a self) -> &'a str {
                    self.as_torrent_alert().torrent_name()
                }
                pub fn message(&self) -> String {
                    self.as_torrent_alert().message()
                }
                pub fn piece(&self) {
                    ::core::panicking::panic("not implemented")
                }
                pub fn buffer(&self) {
                    ::core::panicking::panic("not implemented")
                }
                pub fn size(&self) -> i32 {
                    unsafe { read_piece_alert_get_size(self.0) }
                }
                pub fn error(&self) -> LtrsError {
                    unsafe { read_piece_alert_get_error(self.0) }.into()
                }
            }
        }
        pub mod state_changed {
            use crate::{
                alerts::{StateChangedAlert, TorrentState},
                ffi::alerts::state_changed::ffi::state_changed_alert_get_prev_state,
                ffi::alerts::state_changed::ffi2::state_changed_alert_get_state,
                torrent_handle::TorrentHandle,
            };
            pub struct StateChangedValues {
                torrent_handle: TorrentHandle,
                state: TorrentState,
                prev_state: TorrentState,
            }
            impl StateChangedValues {
                pub fn torrent_handle(&self) -> &TorrentHandle {
                    &self.torrent_handle
                }
                pub fn state(&self) -> TorrentState {
                    self.state
                }
                pub fn prev_state(&self) -> TorrentState {
                    self.prev_state
                }
            }
            impl StateChangedAlert {
                pub fn null() -> Self {
                    StateChangedAlert(std::ptr::null_mut())
                }
                pub fn handle(&self) -> TorrentHandle {
                    self.as_torrent_alert().handle()
                }
                pub fn torrent_name<'a>(&'a self) -> &'a str {
                    self.as_torrent_alert().torrent_name()
                }
                pub fn message(&self) -> String {
                    self.as_torrent_alert().message()
                }
                #[inline(always)]
                pub fn state(&self) -> TorrentState {
                    let state = unsafe { state_changed_alert_get_state(self.0) };
                    unsafe { std::mem::transmute(state) }
                }
                pub fn prev_state(&self) -> TorrentState {
                    let prev_state = unsafe {
                        state_changed_alert_get_prev_state(self.0)
                    };
                    unsafe { std::mem::transmute(prev_state) }
                }
            }
        }
        pub mod torrent_removed {
            use crate::{
                alerts::TorrentRemovedAlert,
                ffi::alerts::torrent_removed::ffi::torrent_removed_alert_get_info_hashes,
                info_hash::InfoHash, torrent_handle::TorrentHandle,
            };
            impl TorrentRemovedAlert {
                pub fn handle(&self) -> TorrentHandle {
                    self.as_torrent_alert().handle()
                }
                pub fn torrent_name<'a>(&'a self) -> &'a str {
                    self.as_torrent_alert().torrent_name()
                }
                pub fn message(&self) -> String {
                    self.as_torrent_alert().message()
                }
                /// The info-hash of the torrent that was removed.
                pub fn info_hash(&self) -> InfoHash {
                    let hash = unsafe { torrent_removed_alert_get_info_hashes(self.0) };
                    hash.into()
                }
                pub fn userdata(&self) {
                    ::core::panicking::panic("not implemented")
                }
            }
        }
        pub mod tracker_error {
            use crate::{
                alerts::{
                    TrackerErrorAlert, operation::Operation,
                    protocol_version::ProtocolVersion,
                },
                errors::LtrsError,
                ffi::alerts::tracker_error::ffi::{
                    tracker_error_alert_get_error,
                    tracker_error_alert_get_failure_reason, tracker_error_alert_get_op,
                    tracker_error_alert_get_times_in_row, tracker_error_alert_get_version,
                },
                torrent_handle::TorrentHandle,
            };
            impl TrackerErrorAlert {
                pub fn handle(&self) -> TorrentHandle {
                    self.as_torrent_alert().handle()
                }
                pub fn torrent_name<'a>(&'a self) -> &'a str {
                    self.as_torrent_alert().torrent_name()
                }
                pub fn message(&self) -> String {
                    self.as_torrent_alert().message()
                }
                pub fn tracker_url<'a>(&'a self) -> &'a str {
                    self.as_tracker_alert().tracker_url()
                }
                pub fn local_endpoint(&self) {}
                pub fn failure_reason(&self) -> &str {
                    unsafe { tracker_error_alert_get_failure_reason(self.0) }
                }
                /// How many times in a row this tracker has failed.
                pub fn times_in_row(&self) -> i32 {
                    unsafe { tracker_error_alert_get_times_in_row(self.0) }
                }
                pub fn error(&self) -> LtrsError {
                    unsafe { tracker_error_alert_get_error(self.0) }.into()
                }
                pub fn op(&self) -> Operation {
                    let op = unsafe { tracker_error_alert_get_op(self.0) };
                    unsafe { Operation::from_u8(op) }
                }
                /// The bittorrent protocol version that was announced
                pub fn version(&self) -> ProtocolVersion {
                    let version = unsafe { tracker_error_alert_get_version(self.0) };
                    unsafe { std::mem::transmute(version) }
                }
            }
        }
    }
    pub mod operation {
        /// These constants are used to identify the operation that failed, causing a
        /// peer to disconnect
        #[repr(u8)]
        pub enum Operation {
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
    }
    pub mod peer_alert {
        use std::marker::PhantomData;
        use crate::{
            alerts::{
                BlockDownloadingAlert, BlockFinishedAlert, BlockTimeoutAlert,
                BlockUploadedAlert, IncomingRequestAlert, InvalidRequestAlert,
                LsdPeerAlert, PeerBanAlert, PeerBlockedAlert, PeerConnectAlert,
                PeerDisconnectedAlert, PeerErrorAlert, PeerLogAlert, PeerSnubbedAlert,
                PeerUnsnubbedAlert, PickerLogAlert, RequestDroppedAlert,
                UnwantedBlockAlert,
            },
            ffi::ffi,
        };
        pub(super) struct PeerAlertRaw<'a>(*mut ffi::peer_alert, PhantomData<&'a ()>);
        impl<'a> PeerAlertRaw<'a> {
            pub(crate) fn new(alert: *mut ffi::peer_alert) -> PeerAlertRaw<'a> {
                PeerAlertRaw(alert, PhantomData)
            }
        }
        pub enum PeerAlert {
            /// This alert is generated when a peer is banned because it has sent too many corrupt pieces
            /// to us. ``ip`` is the endpoint to the peer that was banned.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Peer`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            PeerBan(PeerBanAlert),
            /// This alert is generated when a peer is un-snubbed. Essentially when it was snubbed for stalling
            /// sending data, and now it started sending data again.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Peer`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            PeerUnsnubbed(PeerUnsnubbedAlert),
            /// This alert is generated when a peer is snubbed, when it stops sending data when we request
            /// it.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Peer`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            PeerSnubbed(PeerSnubbedAlert),
            /// This alert is generated when a peer sends invalid data over the peer-peer protocol. The peer
            /// will be disconnected, but you get its ip address from the alert, to identify it.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Peer`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            PeerError(PeerErrorAlert),
            /// This alert is posted every time an incoming peer connection both
            /// successfully passes the protocol handshake and is associated with a
            /// torrent, or an outgoing peer connection attempt succeeds. For arbitrary
            /// incoming connections, see incoming_connection_alert.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Connect`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            PeerConnect(PeerConnectAlert),
            /// This alert is generated when a peer is disconnected for any reason (other than the ones
            /// covered by [`PeerAlert::PeerError`]).
            ///
            /// ## Alert Category
            /// [`AlertCategory::Connect`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            PeerDisconnected(PeerDisconnectedAlert),
            /// This is a debug alert that is generated by an incoming invalid piece request.
            /// ``ip`` is the address of the peer and the ``request`` is the actual incoming
            /// request from the peer. See peer_request for more info.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Peer`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            InvalidRequest(InvalidRequestAlert),
            /// This alert is generated when a peer rejects or ignores a piece request.
            ///
            /// ## Alert Category
            /// [`AlertCategory::BlockProgress`] [`AlertCategory::Peer`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            RequestDropped(RequestDroppedAlert),
            /// This alert is generated when a block request times out.
            ///
            /// ## Alert Category
            /// [`AlertCategory::BlockProgress`] [`AlertCategory::Peer`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            BlockTimeout(BlockTimeoutAlert),
            /// This alert is generated when a block request receives a response.
            ///
            /// ## Alert Category
            /// [`AlertCategory::BlockProgress`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            BlockFinished(BlockFinishedAlert),
            /// This alert is generated when a block request is sent to a peer.
            ///
            /// ## Alert Category
            /// [`AlertCategory::BlockProgress`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            BlockDownloading(BlockDownloadingAlert),
            /// This alert is generated when a block is received that was not requested or
            /// whose request timed out.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Peer`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            UnwantedBlock(UnwantedBlockAlert),
            /// This alert is posted when an incoming peer connection, or a peer that's about to be added
            /// to our peer list, is blocked for some reason. This could be any of:
            ///
            /// * the IP filter
            /// * i2p mixed mode restrictions (a normal peer is not allowed on an i2p swarm)
            /// * the port filter
            /// * the peer has a low port and ``no_connect_privileged_ports`` is enabled
            /// * the protocol of the peer is blocked (uTP/TCP blocking)
            ///
            /// ## Alert Category
            /// [`AlertCategory::IpBlock`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            PeerBlocked(PeerBlockedAlert),
            /// This alert is generated when we receive a local service discovery message
            /// from a peer for a torrent we're currently participating in.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Peer`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            LsdPeer(LsdPeerAlert),
            /// This alert is posted by events specific to a peer. It's meant to be used
            /// for trouble shooting and debugging. It's not enabled by the default alert
            /// mask and is enabled by the [`AlertCategory::PeerLog`] bit. By
            /// default it is disabled as a build configuration.
            ///
            /// ## Alert Category
            /// [`AlertCategory::PeerLog`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            PeerLog(PeerLogAlert),
            /// Posted every time an incoming request from a peer is accepted and queued
            /// up for being serviced. This alert is only posted if
            /// the AlertCategory::IncomingRequest flag is enabled in the alert
            /// mask.
            ///
            /// ## Alert Category
            /// [`AlertCategory::IncomingRequest`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            IncomingRequest(IncomingRequestAlert),
            /// This is posted when one or more blocks are picked by the piece picker,
            /// assuming the verbose piece picker logging is enabled (see
            /// [`AlertCategory::PickerLog`]).
            ///
            /// ## Alert Category
            /// [`AlertCategory::PickerLog`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            PickerLog(PickerLogAlert),
            /// This alert is posted when a block intended to be sent to a peer is placed in the
            /// send buffer. Note that if the connection is closed before the send buffer is sent,
            /// the alert may be posted without the bytes having been sent to the peer.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Upload`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            BlockUploaded(BlockUploadedAlert),
        }
    }
    pub mod performance_warning {
        #[repr(u8)]
        pub enum PerformanceWarning {
            /// This warning means that the number of bytes queued to be written to disk
            /// exceeds the max disk byte queue setting (``settings_pack::max_queued_disk_bytes``).
            /// This might restrict the download rate, by not queuing up enough write jobs
            /// to the disk I/O thread. When this alert is posted, peer connections are
            /// temporarily stopped from downloading, until the queued disk bytes have fallen
            /// below the limit again. Unless your ``max_queued_disk_bytes`` setting is already
            /// high, you might want to increase it to get better performance.
            OutstandingDiskBufferLimitReached,
            /// This is posted when libtorrent would like to send more requests to a peer,
            /// but it's limited by ``settings_pack::max_out_request_queue``. The queue length
            /// libtorrent is trying to achieve is determined by the download rate and the
            /// assumed round-trip-time (``settings_pack::request_queue_time``). The assumed
            /// round-trip-time is not limited to just the network RTT, but also the remote disk
            /// access time and message handling time. It defaults to 3 seconds. The target number
            /// of outstanding requests is set to fill the bandwidth-delay product (assumed RTT
            /// times download rate divided by number of bytes per request). When this alert
            /// is posted, there is a risk that the number of outstanding requests is too low
            /// and limits the download rate. You might want to increase the ``max_out_request_queue``
            /// setting.
            OutstandingRequestLimitReached,
            /// This warning is posted when the amount of TCP/IP overhead is greater than the
            /// upload rate limit. When this happens, the TCP/IP overhead is caused by a much
            /// faster download rate, triggering TCP ACK packets. These packets eat into the
            /// rate limit specified to libtorrent. When the overhead traffic is greater than
            /// the rate limit, libtorrent will not be able to send any actual payload, such
            /// as piece requests. This means the download rate will suffer, and new requests
            /// can be sent again. There will be an equilibrium where the download rate, on
            /// average, is about 20 times the upload rate limit. If you want to maximize the
            /// download rate, increase the upload rate limit above 5% of your download capacity.
            UploadLimitTooLow,
            /// This is the same warning as ``upload_limit_too_low`` but referring to the download
            /// limit instead of upload. This suggests that your download rate limit is much lower
            /// than your upload capacity. Your upload rate will suffer. To maximize upload rate,
            /// make sure your download rate limit is above 5% of your upload capacity.
            DownloadLimitTooLow,
            /// We're stalled on the disk. We want to write to the socket, and we can write
            /// but our send buffer is empty, waiting to be refilled from the disk.
            /// This either means the disk is slower than the network connection
            /// or that our send buffer watermark is too small, because we can
            /// send it all before the disk gets back to us.
            /// The number of bytes that we keep outstanding, requested from the disk, is calculated
            /// as follows:
            ///
            /// .. code:: C++
            ///
            ///    min(512, max(upload_rate * send_buffer_watermark_factor / 100, send_buffer_watermark))
            ///
            /// If you receive this alert, you might want to either increase your ``send_buffer_watermark``
            /// or ``send_buffer_watermark_factor``.
            SendBufferWatermarkTooLow,
            /// If the half (or more) of all upload slots are set as optimistic unchoke slots, this
            /// warning is issued. You probably want more regular (rate based) unchoke slots.
            TooManyOptimisticUnchokeSlots,
            /// If the disk write queue ever grows larger than half of the cache size, this warning
            /// is posted. The disk write queue eats into the total disk cache and leaves very little
            /// left for the actual cache. This causes the disk cache to oscillate in evicting large
            /// portions of the cache before allowing peers to download any more, onto the disk write
            /// queue. Either lower ``max_queued_disk_bytes`` or increase ``cache_size``.
            TooHighDiskQueueLimit,
            AioLimitReached,
            #[deprecated]
            BittyrantWithNoUplimit,
            /// This is generated if outgoing peer connections are failing because of *address in use*
            /// errors, indicating that ``settings_pack::outgoing_ports`` is set and is too small of
            /// a range. Consider not using the ``outgoing_ports`` setting at all, or widen the range to
            /// include more ports.
            TooFewOutgoingPorts,
            TooFewFileDescriptors,
        }
    }
    pub mod protocol_version {
        /// BitTorrent version enumerator
        #[repr(u8)]
        pub enum ProtocolVersion {
            /// The original BitTorrent version, using SHA-1 hashes
            V1,
            /// Version 2 of the BitTorrent protocol, using SHA-256 hashes
            V2,
        }
    }
    pub mod torrent_alert {
        use std::marker::PhantomData;
        use crate::{
            alerts::{
                AddTorrentAlert, CacheFlushedAlert, FastresumeRejectedAlert,
                FileCompletedAlert, FileErrorAlert, FilePrioAlert, FileProgressAlert,
                FileRenameFailedAlert, FileRenamedAlert, HashFailedAlert,
                MetadataFailedAlert, MetadataReceivedAlert, OversizedFileAlert,
                PeerInfoAlert, PerformanceAlert, PieceAvailabilityAlert,
                PieceFinishedAlert, PieceInfoAlert, ReadPieceAlert, SaveResumeDataAlert,
                SaveResumeDataFailedAlert, StateChangedAlert, StorageMovedAlert,
                StorageMovedFailedAlert, TorrentCheckedAlert, TorrentConflictAlert,
                TorrentDeleteFailedAlert, TorrentDeletedAlert, TorrentErrorAlert,
                TorrentFinishedAlert, TorrentLogAlert, TorrentNeedCertAlert,
                TorrentPausedAlert, TorrentRemovedAlert, TorrentResumedAlert,
                TrackerListAlert, UrlSeedAlert, peer_alert::PeerAlert,
                tracker_alert::TrackerAlert,
            },
            ffi::{
                alerts::torrent_alert::ffi::{
                    torrent_alert_handle, torrent_alert_message,
                    torrent_alert_torrent_name,
                },
                ffi::torrent_alert,
            },
            torrent_handle::TorrentHandle,
        };
        pub(crate) struct TorrentAlertRaw<'a>(*mut torrent_alert, PhantomData<&'a ()>);
        impl<'a> TorrentAlertRaw<'a> {
            pub(crate) fn new(alert: *mut torrent_alert) -> TorrentAlertRaw<'a> {
                TorrentAlertRaw(alert, PhantomData)
            }
            pub(crate) fn message(&self) -> String {
                unsafe { torrent_alert_message(self.0) }
            }
            pub(crate) fn torrent_name(&self) -> &'a str {
                unsafe { torrent_alert_torrent_name::<'a>(self.0) }
            }
            pub fn handle(&self) -> TorrentHandle {
                TorrentHandle::from_inner(unsafe { torrent_alert_handle(self.0) })
            }
        }
        pub enum TorrentAlert {
            /// The [`TorrentAlert::TorrentRemoved`] is posted whenever a torrent is removed. Since
            /// the torrent handle in its base class will usually be invalid (since the torrent
            /// is already removed) it has the info hash as a member, to identify it.
            /// It's posted when the [`AlertCategory::Status`] bit is set in the alert_mask.
            ///
            /// ## Notes
            /// Note that the ``handle`` remains valid for some time after
            /// [`TorrentAlert::TorrentRemoved`] is posted, as long as some internal libtorrent
            /// task (such as an I/O task) refers to it. Additionally, other alerts like
            /// [`TorrentAlert::SaveResumeData`] may be posted after [`TorrentAlert::TorrentRemoved`].
            /// To synchronize on whether the torrent has been removed or not, call
            /// [`TorrentHandle::in_session()`]. This will return true before
            /// [`TorrentAlert::TorrentRemoved`] is posted, and false afterward.
            ///
            /// Even though the ``handle`` member doesn't point to an existing torrent anymore,
            /// it is still useful for comparing to other handles, which may also no
            /// longer point to existing torrents, but to the same non-existing torrents.
            ///
            /// The [`TorrentHandle`] acts as a weak pointer, even though its object no
            /// longer exists, it can still compare equal to another weak pointer which
            /// points to the same non-existent object.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            TorrentRemoved(TorrentRemovedAlert),
            /// This alert is posted when the asynchronous read operation initiated by a call to [`TorrentHandle::read_piece()`] is completed.
            /// If the read failed, the torrent is paused and an error state is set and the buffer member of the alert is 0.
            /// If successful, buffer points to a buffer containing all the data of the piece. piece is the piece index that was read.
            /// size is the number of bytes that was read.
            ///
            /// If the operation fails, error will indicate what went wrong.
            /// ## Alert Category
            /// [`AlertCategory::Storage`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            ReadPiece(ReadPieceAlert),
            FileCompleted(FileCompletedAlert),
            /// This is posted as a response to a [`TorrentHandle::rename_file()`] call, if the rename
            /// operation succeeds.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Storage`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            FileRenamed(FileRenamedAlert),
            /// This is posted as a response to a [`TorrentHandle::rename_file()`] call, if the rename
            /// operation failed.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Storage`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            FileRenameFailed(FileRenameFailedAlert),
            /// This alert is generated when a limit is reached that might have a negative impact on
            /// upload or download rate performance.
            ///
            /// ## Alert Category
            /// [`AlertCategory::PerformanceWarning`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            Performance(PerformanceAlert),
            /// Generated whenever a torrent changes its state.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::High`]
            StateChanged(StateChangedAlert),
            /// This alert is generated when a finished piece fails its hash check. You can get the handle
            /// to the torrent which got the failed piece and the index of the piece itself from the alert.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            HashFailed(HashFailedAlert),
            /// This alert is generated when a torrent switches from being a downloader to a seed.
            /// It will only be generated once per torrent.
            /// It contains a [`TorrentHandle`] to the torrent in question.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::High`]
            TorrentFinished(TorrentFinishedAlert),
            /// This alert is posted every time a piece completes downloading
            /// and passes the hash check. This alert derives from [`TorrentAlert`],
            /// which contains the [`TorrentHandle`] to the torrent the piece belongs to.
            /// Note that being downloaded and passing the hash check may happen before
            /// the piece is also fully flushed to disk. So [`TorrentHandle::have_piece()`]
            /// may still return false
            ///
            /// ## Alert Category
            /// [`AlertCategory::PieceProgress`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            PieceFinished(PieceFinishedAlert),
            /// The [`TorrentAlert::StorageMoved`] is generated when all the disk IO has
            /// completed and the files have been moved, as an effect of a call to
            /// [`TorrentHandle::move_storage()`]. This is useful to synchronize with the
            /// actual disk. The ``storage_path()`` member return the new path of the
            /// storage.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Storage`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            StorageMoved(StorageMovedAlert),
            /// The [`TorrentAlert::StorageMovedFailed`] is generated when an attempt to move the storage,
            /// via [`TorrentHandle::move_storage()`], fails.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Storage`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            StorageMovedFailed(StorageMovedFailedAlert),
            /// This alert is generated when a request to delete the files of a torrent complete.
            ///
            /// This alert is posted in the [`AlertCategory::Storage`] category, and that bit
            /// needs to be set in the alert_mask.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Storage`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            TorrentDeleted(TorrentDeletedAlert),
            /// This alert is generated when a request to delete the files of a torrent fails.
            /// Just removing a torrent from the session cannot fail
            ///
            /// ## Alert Category
            /// [`AlertCategory::Storage`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            TorrentDeleteFailed(TorrentDeleteFailedAlert),
            /// This alert is generated as a response to a [`TorrentHandle::save_resume_data`] request.
            /// It is generated once the disk IO thread is done writing the state for this torrent.
            ///
            /// The params struct is populated with the torrent file whose resume data was saved. It is suitable to be:
            ///
            /// * Added to a session with [`Session::add_torrent()`] or [`Session::async_add_torrent()`]
            /// * Saved to disk with write_resume_data()
            /// * Turned into a magnet link with make_magnet_uri()
            /// * Saved as a .torrent file with write_torrent_file()
            /// ## Alert Category
            /// [`AlertCategory::Storage`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            SaveResumeData(SaveResumeDataAlert),
            /// This alert is generated instead of [`TorrentAlert::SaveResumeData`] if there was an error generating
            /// the resume data. error describes what went wrong.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Storage`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            SaveResumeDataFailed(SaveResumeDataFailedAlert),
            /// This alert is generated as a response to a [`TorrentHandle::pause`] request. It is
            /// generated once all disk IO is complete and the files in the torrent have been closed.
            /// This is useful for synchronizing with the disk.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::High`]
            TorrentPaused(TorrentPausedAlert),
            /// This alert is generated as a response to a [`TorrentHandle::resume()`] request. It is
            /// generated when a torrent goes from a paused state to an active state.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::High`]
            TorrentResumed(TorrentResumedAlert),
            /// This alert is posted when a torrent completes checking. i.e. when it transitions
            /// out of the ``checking files`` state into a state where it is ready to start downloading
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::High`]
            TorrentChecked(TorrentCheckedAlert),
            /// This alert is generated when a HTTP seed name lookup fails.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Peer`] [`AlertCategory::Error`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            UrlSeed(UrlSeedAlert),
            /// If the storage fails to read or write files that it needs access to, this alert is
            /// generated and the torrent is paused.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::High`]
            FileError(FileErrorAlert),
            /// This alert is generated when the metadata has been completely received and the info-hash
            /// failed to match it. i.e. the metadata that was received was corrupt. libtorrent will
            /// automatically retry to fetch it in this case. This is only relevant when running a
            /// torrent-less download, with the metadata extension provided by libtorrent.
            ///
            /// ## Alert Category
            ///  [`AlertCategory::Error`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            MetadataFailed(MetadataFailedAlert),
            /// This alert is generated when the metadata has been completely received and the torrent
            /// can start downloading. It is not generated on torrents that are started with metadata, but
            /// only those that needs to download it from peers (when utilizing the libtorrent extension).
            ///
            /// There are no additional data members in this alert.
            ///
            /// Typically, when receiving this alert, you would want to save the torrent file in order
            /// to load it back up again when the session is restarted.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            MetadataReceived(MetadataReceivedAlert),
            /// This alert is generated when a fast resume file has been passed to
            /// add_torrent() but the files on disk did not match the fast resume file.
            /// The error_code explains the reason why the resume file was rejected.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            FastresumeRejected(FastresumeRejectedAlert),
            /// This alert is posted approximately once every second, and it contains
            /// byte counters of most statistics that's tracked for torrents. Each active
            /// torrent posts these alerts regularly.
            /// This alert has been superseded by calling ``post_torrent_updates()``
            /// regularly on the session object. This alert will be removed
            ///
            /// /// ## Alert Category
            /// [`AlertCategory::Stats`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            /// This alert is posted when the disk cache has been flushed for a specific
            /// torrent as a result of a call to [`TorrentHandle::flush_cache()`]. This
            /// alert belongs to the [`AlertCategory::Storage`] category, which must be
            /// enabled to let this alert through. The alert is also posted when removing
            /// a torrent from the session, once the outstanding cache flush is complete
            /// and the torrent does no longer have any files open.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Storage`]
            /// ## Alert Priority
            /// [`AlertPriority::High`]
            CacheFlushed(CacheFlushedAlert),
            /// This is posted whenever a torrent is transitioned into the error state.
            /// If the error code is duplicate_torrent (error_code_enum) error, it suggests two magnet
            /// links ended up resolving to the same hybrid torrent. For more details,
            /// see BitTorrent-v2-torrents_.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`] [`AlertCategory::Error`]
            /// ## Alert Priority
            /// [`AlertPriority::High`]
            TorrentError(TorrentErrorAlert),
            /// This is always posted for SSL torrents. This is a reminder to the client that
            /// the torrent won't work unless [`TorrentHandle::set_ssl_certificate()`] is called with
            /// a valid certificate. Valid certificates MUST be signed by the SSL certificate
            /// in the .torrent file.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            TorrentNeedCert(TorrentNeedCertAlert),
            /// This alert is always posted when a torrent was attempted to be added and contains the return status of the add operation.
            /// The torrent handle of the new torrent can be found as the handle member in the base class.
            /// If adding the torrent failed, error contains the error code.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            AddTorrent(AddTorrentAlert),
            /// This alert is posted by torrent wide events. It's meant to be used for
            /// trouble shooting and debugging. It's not enabled by the default alert
            /// mask and is enabled by the [`AlertCategory::TorrentLog`] bit. By
            /// default it is disabled as a build configuration.
            ///
            /// ## Alert Category
            /// [`AlertCategory::TorrentLog`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            TorrentLog(TorrentLogAlert),
            /// Posted when a prioritize_files() or file_priority() update of the file
            /// priorities complete, which requires a round-trip to the disk thread.
            ///
            /// If the disk operation fails this alert won't be posted, but a
            /// [`Alert::FileError`] is posted instead, and the torrent is stopped.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Storage`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            FilePrio(FilePrioAlert),
            /// This alert may be posted when the initial checking of resume data and files
            /// on disk (just existence, not piece hashes) completes. If a file belonging
            /// to the torrent is found on disk, but is larger than the file in the
            /// torrent, that's when this alert is posted.
            /// the client may want to call truncate_files() in that case, or perhaps
            /// interpret it as a sign that some other file is in the way, that shouldn't
            /// be overwritten.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Storage`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            OversizedFile(OversizedFileAlert),
            /// This alert is posted when two separate torrents (magnet links) resolve to
            /// the same torrent, thus causing the same torrent being added twice. In
            /// that case, both torrents enter an error state with ``duplicate_torrent``
            /// as the error code. This alert is posted containing the metadata. For more
            /// information, see BitTorrent-v2-torrents_.
            /// The torrent this alert originated from was the one that downloaded the
            /// metadata (i.e. the `handle` member).
            ///
            /// ## Alert Category
            /// [`AlertCategory::Error`]
            /// ## Alert Priority
            /// [`AlertPriority::High`]
            TorrentConflict(TorrentConflictAlert),
            /// Posted when [`TorrentHandle::post_peer_info()`] is called
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            PeerInfo(PeerInfoAlert),
            /// Posted when [`TorrentHandle::post_file_progress()`] is called
            ///
            /// ## Alert Category
            /// [`AlertCategory::FileProgress`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            FileProgress(FileProgressAlert),
            /// Posted when [`TorrentHandle::post_download_queue()`] is called
            ///
            /// ## Alert Category
            /// [`AlertCategory::PieceProgress`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            PieceInfo(PieceInfoAlert),
            /// Posted when [`TorrentHandle::post_piece_availability()`] is called
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            PieceAvailability(PieceAvailabilityAlert),
            /// Posted when [`TorrentHandle::post_trackers()`] is called
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            TrackerList(TrackerListAlert),
            /// The peer alert is a base variant for alerts that refer to a specific peer.
            /// It includes all the information to identify the peer. i.e. ip and peer-id.
            PeerAlert(PeerAlert),
            /// This is a base variant used for alerts that are associated with a specific tracker.
            /// It is a variant of [`TorrentAlert`] since a tracker is also associated with a specific torrent.
            TrackerAlert(TrackerAlert),
        }
    }
    pub mod torrent_state {
        use std::fmt::Display;
        /// The missing enums are unused enums from versions of libtorrent before 1.2
        #[repr(u8)]
        pub enum TorrentState {
            /// The torrent is in the queue for being checked. But there
            /// currently is another torrent that are being checked.
            /// This torrent will wait for its turn.
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
            /// The torrent is currently checking the fast resume data and
            /// comparing it to the files on disk. This is typically
            /// completed in a fraction of a second, but if you add a
            /// large number of torrents at once, they will queue up.
            CheckingResumeData = 7,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TorrentState {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        TorrentState::CheckingFiles => "CheckingFiles",
                        TorrentState::DownloadingMetadata => "DownloadingMetadata",
                        TorrentState::Downloading => "Downloading",
                        TorrentState::Finished => "Finished",
                        TorrentState::Seeding => "Seeding",
                        TorrentState::CheckingResumeData => "CheckingResumeData",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TorrentState {
            #[inline]
            fn clone(&self) -> TorrentState {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for TorrentState {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TorrentState {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TorrentState {
            #[inline]
            fn eq(&self, other: &TorrentState) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for TorrentState {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for TorrentState {
            #[inline]
            fn partial_cmp(
                &self,
                other: &TorrentState,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for TorrentState {
            #[inline]
            fn cmp(&self, other: &TorrentState) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for TorrentState {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state)
            }
        }
        impl Display for TorrentState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{0:?}", self))
            }
        }
    }
    pub mod tracker_alert {
        use std::marker::PhantomData;
        use crate::{
            alerts::{
                DhtReplyAlert, ScrapeFailedAlert, ScrapeReplyAlert, TrackerAnnounceAlert,
                TrackerErrorAlert, TrackerReplyAlert, TrackerWarningAlert, TrackeridAlert,
            },
            ffi::{alerts::tracker_alert::ffi::tracker_alert_get_tracker_url, ffi},
        };
        pub struct TrackerAlertRaw<'a>(*mut ffi::tracker_alert, PhantomData<&'a ()>);
        impl<'a> TrackerAlertRaw<'a> {
            pub(crate) fn new(alert: *mut ffi::tracker_alert) -> TrackerAlertRaw<'a> {
                TrackerAlertRaw(alert, PhantomData)
            }
            pub(crate) fn tracker_url(&self) -> &'a str {
                unsafe { tracker_alert_get_tracker_url(self.0) }
            }
            pub(crate) fn local_endpoint() {
                ::core::panicking::panic("not implemented")
            }
        }
        pub enum TrackerAlert {
            /// This alert is generated on tracker time outs, premature disconnects,
            /// invalid response or a HTTP response other than "200 OK". From the alert
            /// you can get the handle to the torrent the tracker belongs to.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Tracker`] [`AlertCategory::Error`]
            /// ## Alert Priority
            /// [`AlertPriority::High`]
            TrackerError(TrackerErrorAlert),
            /// This alert is triggered if the tracker reply contains a warning field.
            /// Usually this means that the tracker announce was successful, but the
            /// tracker has a message to the client.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Tracker`] [`AlertCategory::Error`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            TrackerWarning(TrackerWarningAlert),
            /// This alert is generated when a scrape request succeeds.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Tracker`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            ScrapeReply(ScrapeReplyAlert),
            /// If a scrape request fails, this alert is generated. This might be due
            /// to the tracker timing out, refusing connection or returning an http response
            /// code indicating an error.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Tracker`] [`AlertCategory::Error`]
            /// ## Alert Priority
            /// [`AlertPriority::Critical`]
            ScrapeFailed(ScrapeFailedAlert),
            /// This alert is only for informational purpose. It is generated when a tracker announce
            /// succeeds. It is generated regardless what kind of tracker was used, be it UDP, HTTP or
            /// the DHT.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Tracker`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            TrackerReply(TrackerReplyAlert),
            /// This alert is generated each time the DHT receives peers from a node. ``num_peers``
            /// is the number of peers we received in this packet. Typically these packets are
            /// received from multiple DHT nodes, and so the alerts are typically generated
            /// a few at a time.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Dht`] [`AlertCategory::Tracker`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            DhtReply(DhtReplyAlert),
            /// This alert is generated each time a tracker announce is sent (or attempted to be sent).
            /// There are no extra data members in this alert. The url can be found in the base class
            /// however.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Tracker`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            TrackerAnnounce(TrackerAnnounceAlert),
            /// This alert is posted whenever a tracker responds with a ``trackerid``.
            /// The tracker ID is like a cookie. libtorrent will store the tracker ID
            /// for this tracker and repeat it in subsequent announces.
            ///
            /// ## Alert Category
            /// [`AlertCategory::Status`]
            /// ## Alert Priority
            /// [`AlertPriority::Normal`]
            Trackerid(TrackeridAlert),
        }
    }
    pub use torrent_alert::TorrentAlert;
    pub use torrent_state::TorrentState;
    use peer_alert::PeerAlertRaw;
    use torrent_alert::TorrentAlertRaw;
    use tracker_alert::TrackerAlertRaw;
    pub struct TorrentRemovedAlert(pub(super) *mut ffi::torrent_removed_alert);
    impl TorrentRemovedAlert {}
    pub struct ReadPieceAlert(pub(super) *mut ffi::read_piece_alert);
    impl ReadPieceAlert {}
    pub struct FileCompletedAlert(pub(super) *mut ffi::file_completed_alert);
    impl FileCompletedAlert {}
    pub struct FileRenamedAlert(pub(super) *mut ffi::file_renamed_alert);
    impl FileRenamedAlert {}
    pub struct FileRenameFailedAlert(pub(super) *mut ffi::file_rename_failed_alert);
    impl FileRenameFailedAlert {}
    pub struct PerformanceAlert(pub(super) *mut ffi::performance_alert);
    impl PerformanceAlert {}
    pub struct StateChangedAlert(pub(super) *mut ffi::state_changed_alert);
    impl StateChangedAlert {}
    pub struct TrackerErrorAlert(pub(super) *mut ffi::tracker_error_alert);
    impl TrackerErrorAlert {}
    pub struct TrackerWarningAlert(pub(super) *mut ffi::tracker_warning_alert);
    impl TrackerWarningAlert {}
    pub struct ScrapeReplyAlert(pub(super) *mut ffi::scrape_reply_alert);
    impl ScrapeReplyAlert {}
    pub struct ScrapeFailedAlert(pub(super) *mut ffi::scrape_failed_alert);
    impl ScrapeFailedAlert {}
    pub struct TrackerReplyAlert(pub(super) *mut ffi::tracker_reply_alert);
    impl TrackerReplyAlert {}
    pub struct DhtReplyAlert(pub(super) *mut ffi::dht_reply_alert);
    impl DhtReplyAlert {}
    pub struct TrackerAnnounceAlert(pub(super) *mut ffi::tracker_announce_alert);
    impl TrackerAnnounceAlert {}
    pub struct HashFailedAlert(pub(super) *mut ffi::hash_failed_alert);
    impl HashFailedAlert {}
    pub struct PeerBanAlert(pub(super) *mut ffi::peer_ban_alert);
    impl PeerBanAlert {}
    pub struct PeerUnsnubbedAlert(pub(super) *mut ffi::peer_unsnubbed_alert);
    impl PeerUnsnubbedAlert {}
    pub struct PeerSnubbedAlert(pub(super) *mut ffi::peer_snubbed_alert);
    impl PeerSnubbedAlert {}
    pub struct PeerErrorAlert(pub(super) *mut ffi::peer_error_alert);
    impl PeerErrorAlert {}
    pub struct PeerConnectAlert(pub(super) *mut ffi::peer_connect_alert);
    impl PeerConnectAlert {}
    pub struct PeerDisconnectedAlert(pub(super) *mut ffi::peer_disconnected_alert);
    impl PeerDisconnectedAlert {}
    pub struct InvalidRequestAlert(pub(super) *mut ffi::invalid_request_alert);
    impl InvalidRequestAlert {}
    pub struct TorrentFinishedAlert(pub(super) *mut ffi::torrent_finished_alert);
    impl TorrentFinishedAlert {}
    pub struct PieceFinishedAlert(pub(super) *mut ffi::piece_finished_alert);
    impl PieceFinishedAlert {}
    pub struct RequestDroppedAlert(pub(super) *mut ffi::request_dropped_alert);
    impl RequestDroppedAlert {}
    pub struct BlockTimeoutAlert(pub(super) *mut ffi::block_timeout_alert);
    impl BlockTimeoutAlert {}
    pub struct BlockFinishedAlert(pub(super) *mut ffi::block_finished_alert);
    impl BlockFinishedAlert {}
    pub struct BlockDownloadingAlert(pub(super) *mut ffi::block_downloading_alert);
    impl BlockDownloadingAlert {}
    pub struct UnwantedBlockAlert(pub(super) *mut ffi::unwanted_block_alert);
    impl UnwantedBlockAlert {}
    pub struct StorageMovedAlert(pub(super) *mut ffi::storage_moved_alert);
    impl StorageMovedAlert {}
    pub struct StorageMovedFailedAlert(pub(super) *mut ffi::storage_moved_failed_alert);
    impl StorageMovedFailedAlert {}
    pub struct TorrentDeletedAlert(pub(super) *mut ffi::torrent_deleted_alert);
    impl TorrentDeletedAlert {}
    pub struct TorrentDeleteFailedAlert(
        pub(super) *mut ffi::torrent_delete_failed_alert,
    );
    impl TorrentDeleteFailedAlert {}
    pub struct SaveResumeDataAlert(pub(super) *mut ffi::save_resume_data_alert);
    impl SaveResumeDataAlert {}
    pub struct SaveResumeDataFailedAlert(
        pub(super) *mut ffi::save_resume_data_failed_alert,
    );
    impl SaveResumeDataFailedAlert {}
    pub struct TorrentPausedAlert(pub(super) *mut ffi::torrent_paused_alert);
    impl TorrentPausedAlert {}
    pub struct TorrentResumedAlert(pub(super) *mut ffi::torrent_resumed_alert);
    impl TorrentResumedAlert {}
    pub struct TorrentCheckedAlert(pub(super) *mut ffi::torrent_checked_alert);
    impl TorrentCheckedAlert {}
    pub struct UrlSeedAlert(pub(super) *mut ffi::url_seed_alert);
    impl UrlSeedAlert {}
    pub struct FileErrorAlert(pub(super) *mut ffi::file_error_alert);
    impl FileErrorAlert {}
    pub struct MetadataFailedAlert(pub(super) *mut ffi::metadata_failed_alert);
    impl MetadataFailedAlert {}
    pub struct MetadataReceivedAlert(pub(super) *mut ffi::metadata_received_alert);
    impl MetadataReceivedAlert {}
    pub struct UdpErrorAlert(pub(super) *mut ffi::udp_error_alert);
    impl UdpErrorAlert {}
    pub struct ExternalIpAlert(pub(super) *mut ffi::external_ip_alert);
    impl ExternalIpAlert {}
    pub struct ListenFailedAlert(pub(super) *mut ffi::listen_failed_alert);
    impl ListenFailedAlert {}
    pub struct ListenSucceededAlert(pub(super) *mut ffi::listen_succeeded_alert);
    impl ListenSucceededAlert {}
    pub struct PortmapErrorAlert(pub(super) *mut ffi::portmap_error_alert);
    impl PortmapErrorAlert {}
    pub struct PortmapAlert(pub(super) *mut ffi::portmap_alert);
    impl PortmapAlert {}
    pub struct PortmapLogAlert(pub(super) *mut ffi::portmap_log_alert);
    impl PortmapLogAlert {}
    pub struct FastresumeRejectedAlert(pub(super) *mut ffi::fastresume_rejected_alert);
    impl FastresumeRejectedAlert {}
    pub struct PeerBlockedAlert(pub(super) *mut ffi::peer_blocked_alert);
    impl PeerBlockedAlert {}
    pub struct DhtAnnounceAlert(pub(super) *mut ffi::dht_announce_alert);
    impl DhtAnnounceAlert {}
    pub struct DhtGetPeersAlert(pub(super) *mut ffi::dht_get_peers_alert);
    impl DhtGetPeersAlert {}
    pub struct CacheFlushedAlert(pub(super) *mut ffi::cache_flushed_alert);
    impl CacheFlushedAlert {}
    pub struct LsdPeerAlert(pub(super) *mut ffi::lsd_peer_alert);
    impl LsdPeerAlert {}
    pub struct TrackeridAlert(pub(super) *mut ffi::trackerid_alert);
    impl TrackeridAlert {}
    pub struct DhtBootstrapAlert(pub(super) *mut ffi::dht_bootstrap_alert);
    impl DhtBootstrapAlert {}
    pub struct TorrentErrorAlert(pub(super) *mut ffi::torrent_error_alert);
    impl TorrentErrorAlert {}
    pub struct TorrentNeedCertAlert(pub(super) *mut ffi::torrent_need_cert_alert);
    impl TorrentNeedCertAlert {}
    pub struct IncomingConnectionAlert(pub(super) *mut ffi::incoming_connection_alert);
    impl IncomingConnectionAlert {}
    pub struct AddTorrentAlert(pub(super) *mut ffi::add_torrent_alert);
    impl AddTorrentAlert {}
    pub struct StateUpdateAlert(pub(super) *mut ffi::state_update_alert);
    impl StateUpdateAlert {}
    pub struct SessionStatsAlert(pub(super) *mut ffi::session_stats_alert);
    impl SessionStatsAlert {}
    pub struct DhtErrorAlert(pub(super) *mut ffi::dht_error_alert);
    impl DhtErrorAlert {}
    pub struct DhtImmutableItemAlert(pub(super) *mut ffi::dht_immutable_item_alert);
    impl DhtImmutableItemAlert {}
    pub struct DhtMutableItemAlert(pub(super) *mut ffi::dht_mutable_item_alert);
    impl DhtMutableItemAlert {}
    pub struct DhtPutAlert(pub(super) *mut ffi::dht_put_alert);
    impl DhtPutAlert {}
    pub struct I2pAlert(pub(super) *mut ffi::i2p_alert);
    impl I2pAlert {}
    pub struct DhtOutgoingGetPeersAlert(
        pub(super) *mut ffi::dht_outgoing_get_peers_alert,
    );
    impl DhtOutgoingGetPeersAlert {}
    pub struct LogAlert(pub(super) *mut ffi::log_alert);
    impl LogAlert {}
    pub struct TorrentLogAlert(pub(super) *mut ffi::torrent_log_alert);
    impl TorrentLogAlert {}
    pub struct PeerLogAlert(pub(super) *mut ffi::peer_log_alert);
    impl PeerLogAlert {}
    pub struct LsdErrorAlert(pub(super) *mut ffi::lsd_error_alert);
    impl LsdErrorAlert {}
    pub struct DhtStatsAlert(pub(super) *mut ffi::dht_stats_alert);
    impl DhtStatsAlert {}
    pub struct IncomingRequestAlert(pub(super) *mut ffi::incoming_request_alert);
    impl IncomingRequestAlert {}
    pub struct DhtLogAlert(pub(super) *mut ffi::dht_log_alert);
    impl DhtLogAlert {}
    pub struct DhtPktAlert(pub(super) *mut ffi::dht_pkt_alert);
    impl DhtPktAlert {}
    pub struct DhtGetPeersReplyAlert(pub(super) *mut ffi::dht_get_peers_reply_alert);
    impl DhtGetPeersReplyAlert {}
    pub struct DhtDirectResponseAlert(pub(super) *mut ffi::dht_direct_response_alert);
    impl DhtDirectResponseAlert {}
    pub struct PickerLogAlert(pub(super) *mut ffi::picker_log_alert);
    impl PickerLogAlert {}
    pub struct SessionErrorAlert(pub(super) *mut ffi::session_error_alert);
    impl SessionErrorAlert {}
    pub struct DhtLiveNodesAlert(pub(super) *mut ffi::dht_live_nodes_alert);
    impl DhtLiveNodesAlert {}
    pub struct SessionStatsHeaderAlert(pub(super) *mut ffi::session_stats_header_alert);
    impl SessionStatsHeaderAlert {}
    pub struct DhtSampleInfohashesAlert(
        pub(super) *mut ffi::dht_sample_infohashes_alert,
    );
    impl DhtSampleInfohashesAlert {}
    pub struct BlockUploadedAlert(pub(super) *mut ffi::block_uploaded_alert);
    impl BlockUploadedAlert {}
    pub struct AlertsDroppedAlert(pub(super) *mut ffi::alerts_dropped_alert);
    impl AlertsDroppedAlert {}
    pub struct Socks5Alert(pub(super) *mut ffi::socks5_alert);
    impl Socks5Alert {}
    pub struct FilePrioAlert(pub(super) *mut ffi::file_prio_alert);
    impl FilePrioAlert {}
    pub struct OversizedFileAlert(pub(super) *mut ffi::oversized_file_alert);
    impl OversizedFileAlert {}
    pub struct TorrentConflictAlert(pub(super) *mut ffi::torrent_conflict_alert);
    impl TorrentConflictAlert {}
    pub struct PeerInfoAlert(pub(super) *mut ffi::peer_info_alert);
    impl PeerInfoAlert {}
    pub struct FileProgressAlert(pub(super) *mut ffi::file_progress_alert);
    impl FileProgressAlert {}
    pub struct PieceInfoAlert(pub(super) *mut ffi::piece_info_alert);
    impl PieceInfoAlert {}
    pub struct PieceAvailabilityAlert(pub(super) *mut ffi::piece_availability_alert);
    impl PieceAvailabilityAlert {}
    pub struct TrackerListAlert(pub(super) *mut ffi::tracker_list_alert);
    impl TrackerListAlert {}
    type TcpEndpoint = String;
    type PeerId = String;
    type UserData = String;
    pub type ErrorCode = i32;
    pub type PieceIndex = i32;
    /// Struct to hold information about a single DHT routing table bucket
    struct DhtRoutingBucket {
        /// Total number of nodes in the routing table
        num_nodes: i32,
        /// Total number of replacement nodes in the routing table
        num_replacements: i32,
        /// Number of seconds since last activity
        last_active: i32,
    }
    /// The [`LtSession::pop_alerts()`] function on session is the main interface for retrieving alerts
    /// (warnings, messages and errors from libtorrent).
    /// If no alerts have been posted by libtorrent [`LtSession::pop_alerts()`] will return an empty list.
    ///
    /// By default, only errors are reported. [`SettingsPack::set_alert_mask()`] can be used to specify
    /// which kinds of events should be reported. The alert mask is a combination of the [`AlertCategory`] flags.
    ///
    /// Every alert belongs to one or more category. There is a cost associated with posting alerts.
    /// Only alerts that belong to an enabled category are posted. Setting the alert bitmask to 0 will disable all alerts
    /// (except those that are non-discardable).
    /// Alerts that are responses to API calls such as save_resume_data() and post_session_stats() are //TODO function reference
    /// non-discardable and will be posted even if their category is disabled.
    ///
    /// There are other alert base classes that some alerts derive from, all the alerts that are
    /// generated for a specific torrent are derived from [`TorrentAlert`],
    /// and tracker events derive from [`TrackerAlert`].
    ///
    /// Alerts returned by [`LtSession::pop_alerts()`] are only valid until the next call to [`LtSession::pop_alerts()`].
    /// You may not copy an alert object to access it after the next call to [`LtSession::pop_alerts()`].
    /// Internal members of alerts also become invalid once [`LtSession::pop_alerts()`] is called again.
    pub enum Alert {
        NotImplemented,
        /// This alert is posted when there is an error on a UDP socket. The
        /// UDP sockets are used for all uTP, DHT and UDP tracker traffic. They are
        /// global to the session.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Error`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        UdpError(UdpErrorAlert),
        /// Whenever libtorrent learns about the machines external IP, this alert is
        /// generated. The external IP address can be acquired from the tracker (if it
        /// supports that) or from peers that supports the extension protocol.
        /// The address can be accessed through the ``external_address`` member.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Status`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        ExternalIp(ExternalIpAlert),
        /// This alert is generated when none of the ports, given in the port range, to
        /// session can be opened for listening. The ``listen_interface`` member is the
        /// interface that failed, ``error`` is the error code describing the failure.
        ///
        /// In the case an endpoint was created before generating the alert, it is
        /// represented by ``address`` and ``port``. The combinations of socket type
        /// and operation in which such address and port are not valid are:
        /// accept  - i2p
        /// accept  - socks5
        /// enum_if - tcp
        ///
        /// libtorrent may sometimes try to listen on port 0, if all other ports failed.
        /// Port 0 asks the operating system to pick a port that's free). If that fails
        /// you may see a [`Alert::ListenFailed`] with port 0 even if you didn't ask to
        /// listen on it.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Status`] | [`AlertCategory::Error`]
        /// ## Alert Priority
        /// [`AlertPriority::Critical`]
        ListenFailed(ListenFailedAlert),
        /// This alert is posted when the listen port succeeds to be opened on a
        /// particular interface. ``address`` and ``port`` is the endpoint that
        /// successfully was opened for listening.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Status`]
        /// ## Alert Priority
        /// [`AlertPriority::Critical`]
        ListenSucceeded(ListenSucceededAlert),
        /// This alert is generated when a NAT router was successfully found but some
        /// part of the port mapping request failed. It contains a text message that
        /// may help the user figure out what is wrong. This alert is not generated in
        /// case it appears the client is not running on a NAT:ed network or if it
        /// appears there is no NAT router that can be remote controlled to add port
        /// mappings.
        ///
        /// ## Alert Category
        /// [`AlertCategory::PortMapping`] | [`AlertCategory::Error`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        PortmapError(PortmapErrorAlert),
        /// This alert is generated when a NAT router was successfully found and
        /// a port was successfully mapped on it. On a NAT:ed network with a NAT-PMP
        /// capable router, this is typically generated once when mapping the TCP
        /// port and, if DHT is enabled, when the UDP port is mapped.
        ///
        /// ## Alert Category
        /// [`AlertCategory::PortMapping`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        Portmap(PortmapAlert),
        /// This alert is generated to log informational events related to either
        /// UPnP or NAT-PMP. They contain a log line and the type (0 = NAT-PMP
        /// and 1 = UPnP). Displaying these messages to an end user is only useful
        /// for debugging the UPnP or NAT-PMP implementation. This alert is only
        /// posted if the alert_category::port_mapping_log flag is enabled in
        /// the alert mask.
        ///
        /// ## Alert Category
        /// [`AlertCategory::PortMappingLog`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        PortmapLog(PortmapLogAlert),
        /// This alert is generated when a DHT node announces to an info-hash on our
        /// DHT node.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Dht`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtAnnounce(DhtAnnounceAlert),
        /// This alert is generated when a DHT node sends a ``get_peers`` message to
        /// our DHT node.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Dht`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtGetPeers(DhtGetPeersAlert),
        /// This alert is posted when the initial DHT bootstrap is done.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Dht`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtBootstrap(DhtBootstrapAlert),
        /// The incoming connection alert is posted every time we successfully accept
        /// an incoming connection, through any mean. The most straight-forward ways
        /// of accepting incoming connections are through the TCP listen socket and
        /// the UDP listen socket for uTP sockets. However, connections may also be
        /// accepted through a Socks5 or i2p listen socket, or via an SSL listen
        /// socket.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Status`]
        /// ## Alert Priority
        /// [`AlertPriority::Peer`]
        IncomingConnection(IncomingConnectionAlert),
        /// This alert is only posted when requested by the user, by calling [`LtSession::post_torrent_updates()`] on the session.
        /// It contains the torrent status of all torrents that changed since last time this message was posted.
        /// Its category is [`AlertCategory::Status`], but it's not subject to filtering, since it's only manually posted anyway.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Status`]
        /// ## Alert Priority
        /// [`AlertPriority::High`]
        StateUpdate(StateUpdateAlert),
        /// The [`Alert::SessionStats`] is posted when the user requests session statistics by
        /// calling post_session_stats() on the session object. This alert does not
        /// have a category, since it's only posted in response to an API call. It
        /// is not subject to the alert_mask filter.
        ///
        /// the ``message()`` member function returns a string representation of the values that
        /// properly match the line returned in ``session_stats_header_alert::message()``.
        ///
        /// this specific output is parsed by tools/parse_session_stats.py
        /// if this is changed, that parser should also be changed
        ///
        /// ## Alert Category
        /// ## Alert Priority
        /// [`AlertPriority::Critical`]
        SessionStats(SessionStatsAlert),
        /// Posted when something fails in the DHT. This is not necessarily a fatal
        /// error, but it could prevent proper operation
        ///
        /// ## Alert Category
        /// [`AlertCategory::Dht`] [`AlertCategory::Error`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtError(DhtErrorAlert),
        /// This alert is posted as a response to a call to [`LtSession::get_item()`],
        /// specifically the overload for looking up immutable items in the DHT.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Dht`]
        /// ## Alert Priority
        /// [`AlertPriority::Critical`]
        DhtImmutableItem(DhtImmutableItemAlert),
        /// This alert is posted as a response to a call to [`LtSession::get_item()`],
        /// specifically the overload for looking up mutable items in the DHT.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Dht`]
        /// ## Alert Priority
        /// [`AlertPriority::Critical`]
        DhtMutableItem(DhtMutableItemAlert),
        /// This is posted when a DHT put operation completes. This is useful if the
        /// client is waiting for a put to complete before shutting down for instance.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Dht`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtPut(DhtPutAlert),
        /// This alert is used to report errors in the i2p SAM connection
        ///
        /// ## Alert Category
        /// [`AlertCategory::Error`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        I2p(I2pAlert),
        /// This alert is generated when we send a get_peers request
        ///
        /// ## Alert Category
        /// [`AlertCategory::Dht`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtOutgoingGetPeers(DhtOutgoingGetPeersAlert),
        /// This alert is posted by some session wide event. Its main purpose is
        /// trouble shooting and debugging. It's not enabled by the default alert
        /// mask and is enabled by the [`AlertCategory::SessionLog`] bit.
        /// Furthermore, it's by default disabled as a build configuration.
        ///
        /// ## Alert Category
        /// [`AlertCategory::SessionLog`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        Log(LogAlert),
        /// Posted if the local service discovery socket fails to start properly.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Error`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        LsdError(LsdErrorAlert),
        /// Contains current DHT state. Posted in response to [`LtSession::post_dht_stats()`].
        ///
        /// ## Alert Category
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtStats(DhtStatsAlert),
        /// Debug logging of the DHT when [`AlertCategory::DhtLog`] is set in the alert
        /// mask.
        ///
        /// ## Alert Category
        /// [`AlertCategory::DhtLog`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtLog(DhtLogAlert),
        /// This alert is posted every time a DHT message is sent or received. It is
        /// only posted if the [`AlertCategory::DhtLog`] alert category is
        /// enabled. It contains a verbatim copy of the message.
        ///
        /// ## Alert Category
        /// [`AlertCategory::DhtLog`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtPkt(DhtPktAlert),
        /// Posted when we receive a response to a DHT get_peers request.
        ///
        /// ## Alert Category
        /// [`AlertCategory::DhtOperation`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtGetPeersReply(DhtGetPeersReplyAlert),
        /// This is posted exactly once for every call to session_handle::dht_direct_request.
        /// If the request failed, response() will return a default constructed bdecode_node.
        ///
        /// ## Alert Category
        /// [`AlertCategory::DhtOperation`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtDirectResponse(DhtDirectResponseAlert),
        /// This alert is posted when the session encounters a serious error,
        /// potentially fatal
        ///
        /// ## Alert Category
        /// [`AlertCategory::Error`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        SessionError(SessionErrorAlert),
        /// Posted in response to a call to [`LtSession::dht_live_nodes()`]. It contains the
        /// live nodes from the DHT routing table of one of the DHT nodes running
        /// locally.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Dht`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtLiveNodes(DhtLiveNodesAlert),
        /// The session_stats_header alert is posted the first time
        /// post_session_stats() is called
        ///
        /// the ``message()`` member function returns a string representation of the
        /// header that properly match the stats values string returned in
        /// ``session_stats_alert::message()``.
        ///
        /// ## Alert Category
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        SessionStatsHeader(SessionStatsHeaderAlert),
        /// Posted as a response to a call to [`LtSession::dht_sample_infohashes()`] with
        /// the information from the DHT response message.
        ///
        /// ## Alert Category
        /// [`AlertCategory::DhtOperation`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        DhtSampleInfohashes(DhtSampleInfohashesAlert),
        /// This alert is posted to indicate to the client that some alerts were
        /// dropped. Dropped meaning that the alert failed to be delivered to the
        /// client. The most common cause of such failure is that the internal alert
        /// queue grew too big (controlled by alert_queue_size).
        ///
        /// ## Alert Category
        /// [`AlertCategory::Error`]
        /// ## Alert Priority
        /// [`AlertPriority::Meta`]
        AlertsDropped(AlertsDroppedAlert),
        /// This alert is posted with SOCKS5 related errors, when a SOCKS5 proxy is
        /// configured. It's enabled with the AlertCategory::Error alert category.
        ///
        /// ## Alert Category
        /// [`AlertCategory::Error`]
        /// ## Alert Priority
        /// [`AlertPriority::Normal`]
        Socks5(Socks5Alert),
        /// This is a base variant for alerts that are associated with a specific torrent. It contains a handle to the torrent.
        ///
        /// Note that by the time the client receives a TorrentAlert, its handle member may be invalid.
        TorrentAlert(TorrentAlert),
    }
    impl From<ffi::CastAlertRaw> for Alert {
        fn from(value: ffi::CastAlertRaw) -> Self {
            match value.type_ {
                ffi::AlertType::TorrentRemoved => {
                    Alert::TorrentAlert(
                        TorrentAlert::TorrentRemoved(
                            TorrentRemovedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::ReadPiece => {
                    Alert::TorrentAlert(
                        TorrentAlert::ReadPiece(ReadPieceAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::FileCompleted => {
                    Alert::TorrentAlert(
                        TorrentAlert::FileCompleted(
                            FileCompletedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::FileRenamed => {
                    Alert::TorrentAlert(
                        TorrentAlert::FileRenamed(FileRenamedAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::FileRenameFailed => {
                    Alert::TorrentAlert(
                        TorrentAlert::FileRenameFailed(
                            FileRenameFailedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::Performance => {
                    Alert::TorrentAlert(
                        TorrentAlert::Performance(PerformanceAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::StateChanged => {
                    Alert::TorrentAlert(
                        TorrentAlert::StateChanged(StateChangedAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::TrackerError => {
                    Alert::TorrentAlert(
                        TorrentAlert::TrackerAlert(
                            TrackerAlert::TrackerError(
                                TrackerErrorAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::TrackerWarning => {
                    Alert::TorrentAlert(
                        TorrentAlert::TrackerAlert(
                            TrackerAlert::TrackerWarning(
                                TrackerWarningAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::ScrapeReply => {
                    Alert::TorrentAlert(
                        TorrentAlert::TrackerAlert(
                            TrackerAlert::ScrapeReply(
                                ScrapeReplyAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::ScrapeFailed => {
                    Alert::TorrentAlert(
                        TorrentAlert::TrackerAlert(
                            TrackerAlert::ScrapeFailed(
                                ScrapeFailedAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::TrackerReply => {
                    Alert::TorrentAlert(
                        TorrentAlert::TrackerAlert(
                            TrackerAlert::TrackerReply(
                                TrackerReplyAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::DhtReply => {
                    Alert::TorrentAlert(
                        TorrentAlert::TrackerAlert(
                            TrackerAlert::DhtReply(DhtReplyAlert(value.alert.cast())),
                        ),
                    )
                }
                ffi::AlertType::TrackerAnnounce => {
                    Alert::TorrentAlert(
                        TorrentAlert::TrackerAlert(
                            TrackerAlert::TrackerAnnounce(
                                TrackerAnnounceAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::HashFailed => {
                    Alert::TorrentAlert(
                        TorrentAlert::HashFailed(HashFailedAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::PeerBan => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::PeerBan(PeerBanAlert(value.alert.cast())),
                        ),
                    )
                }
                ffi::AlertType::PeerUnsnubbed => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::PeerUnsnubbed(
                                PeerUnsnubbedAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::PeerSnubbed => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::PeerSnubbed(PeerSnubbedAlert(value.alert.cast())),
                        ),
                    )
                }
                ffi::AlertType::PeerError => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::PeerError(PeerErrorAlert(value.alert.cast())),
                        ),
                    )
                }
                ffi::AlertType::PeerConnect => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::PeerConnect(PeerConnectAlert(value.alert.cast())),
                        ),
                    )
                }
                ffi::AlertType::PeerDisconnected => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::PeerDisconnected(
                                PeerDisconnectedAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::InvalidRequest => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::InvalidRequest(
                                InvalidRequestAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::TorrentFinished => {
                    Alert::TorrentAlert(
                        TorrentAlert::TorrentFinished(
                            TorrentFinishedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::PieceFinished => {
                    Alert::TorrentAlert(
                        TorrentAlert::PieceFinished(
                            PieceFinishedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::RequestDropped => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::RequestDropped(
                                RequestDroppedAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::BlockTimeout => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::BlockTimeout(
                                BlockTimeoutAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::BlockFinished => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::BlockFinished(
                                BlockFinishedAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::BlockDownloading => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::BlockDownloading(
                                BlockDownloadingAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::UnwantedBlock => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::UnwantedBlock(
                                UnwantedBlockAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::StorageMoved => {
                    Alert::TorrentAlert(
                        TorrentAlert::StorageMoved(StorageMovedAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::StorageMovedFailed => {
                    Alert::TorrentAlert(
                        TorrentAlert::StorageMovedFailed(
                            StorageMovedFailedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::TorrentDeleted => {
                    Alert::TorrentAlert(
                        TorrentAlert::TorrentDeleted(
                            TorrentDeletedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::TorrentDeleteFailed => {
                    Alert::TorrentAlert(
                        TorrentAlert::TorrentDeleteFailed(
                            TorrentDeleteFailedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::SaveResumeData => {
                    Alert::TorrentAlert(
                        TorrentAlert::SaveResumeData(
                            SaveResumeDataAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::SaveResumeDataFailed => {
                    Alert::TorrentAlert(
                        TorrentAlert::SaveResumeDataFailed(
                            SaveResumeDataFailedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::TorrentPaused => {
                    Alert::TorrentAlert(
                        TorrentAlert::TorrentPaused(
                            TorrentPausedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::TorrentResumed => {
                    Alert::TorrentAlert(
                        TorrentAlert::TorrentResumed(
                            TorrentResumedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::TorrentChecked => {
                    Alert::TorrentAlert(
                        TorrentAlert::TorrentChecked(
                            TorrentCheckedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::UrlSeed => {
                    Alert::TorrentAlert(
                        TorrentAlert::UrlSeed(UrlSeedAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::FileError => {
                    Alert::TorrentAlert(
                        TorrentAlert::FileError(FileErrorAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::MetadataFailed => {
                    Alert::TorrentAlert(
                        TorrentAlert::MetadataFailed(
                            MetadataFailedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::MetadataReceived => {
                    Alert::TorrentAlert(
                        TorrentAlert::MetadataReceived(
                            MetadataReceivedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::UdpError => {
                    Alert::UdpError(UdpErrorAlert(value.alert.cast()))
                }
                ffi::AlertType::ExternalIp => {
                    Alert::ExternalIp(ExternalIpAlert(value.alert.cast()))
                }
                ffi::AlertType::ListenFailed => {
                    Alert::ListenFailed(ListenFailedAlert(value.alert.cast()))
                }
                ffi::AlertType::ListenSucceeded => {
                    Alert::ListenSucceeded(ListenSucceededAlert(value.alert.cast()))
                }
                ffi::AlertType::PortmapError => {
                    Alert::PortmapError(PortmapErrorAlert(value.alert.cast()))
                }
                ffi::AlertType::Portmap => {
                    Alert::Portmap(PortmapAlert(value.alert.cast()))
                }
                ffi::AlertType::PortmapLog => {
                    Alert::PortmapLog(PortmapLogAlert(value.alert.cast()))
                }
                ffi::AlertType::FastresumeRejected => {
                    Alert::TorrentAlert(
                        TorrentAlert::FastresumeRejected(
                            FastresumeRejectedAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::PeerBlocked => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::PeerBlocked(PeerBlockedAlert(value.alert.cast())),
                        ),
                    )
                }
                ffi::AlertType::DhtAnnounce => {
                    Alert::DhtAnnounce(DhtAnnounceAlert(value.alert.cast()))
                }
                ffi::AlertType::DhtGetPeers => {
                    Alert::DhtGetPeers(DhtGetPeersAlert(value.alert.cast()))
                }
                ffi::AlertType::CacheFlushed => {
                    Alert::TorrentAlert(
                        TorrentAlert::CacheFlushed(CacheFlushedAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::LsdPeer => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::LsdPeer(LsdPeerAlert(value.alert.cast())),
                        ),
                    )
                }
                ffi::AlertType::Trackerid => {
                    Alert::TorrentAlert(
                        TorrentAlert::TrackerAlert(
                            TrackerAlert::Trackerid(TrackeridAlert(value.alert.cast())),
                        ),
                    )
                }
                ffi::AlertType::DhtBootstrap => {
                    Alert::DhtBootstrap(DhtBootstrapAlert(value.alert.cast()))
                }
                ffi::AlertType::TorrentError => {
                    Alert::TorrentAlert(
                        TorrentAlert::TorrentError(TorrentErrorAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::TorrentNeedCert => {
                    Alert::TorrentAlert(
                        TorrentAlert::TorrentNeedCert(
                            TorrentNeedCertAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::IncomingConnection => {
                    Alert::IncomingConnection(
                        IncomingConnectionAlert(value.alert.cast()),
                    )
                }
                ffi::AlertType::AddTorrent => {
                    Alert::TorrentAlert(
                        TorrentAlert::AddTorrent(AddTorrentAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::StateUpdate => {
                    Alert::StateUpdate(StateUpdateAlert(value.alert.cast()))
                }
                ffi::AlertType::SessionStats => {
                    Alert::SessionStats(SessionStatsAlert(value.alert.cast()))
                }
                ffi::AlertType::DhtError => {
                    Alert::DhtError(DhtErrorAlert(value.alert.cast()))
                }
                ffi::AlertType::DhtImmutableItem => {
                    Alert::DhtImmutableItem(DhtImmutableItemAlert(value.alert.cast()))
                }
                ffi::AlertType::DhtMutableItem => {
                    Alert::DhtMutableItem(DhtMutableItemAlert(value.alert.cast()))
                }
                ffi::AlertType::DhtPut => Alert::DhtPut(DhtPutAlert(value.alert.cast())),
                ffi::AlertType::I2p => Alert::I2p(I2pAlert(value.alert.cast())),
                ffi::AlertType::DhtOutgoingGetPeers => {
                    Alert::DhtOutgoingGetPeers(
                        DhtOutgoingGetPeersAlert(value.alert.cast()),
                    )
                }
                ffi::AlertType::Log => Alert::Log(LogAlert(value.alert.cast())),
                ffi::AlertType::TorrentLog => {
                    Alert::TorrentAlert(
                        TorrentAlert::TorrentLog(TorrentLogAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::PeerLog => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::PeerLog(PeerLogAlert(value.alert.cast())),
                        ),
                    )
                }
                ffi::AlertType::LsdError => {
                    Alert::LsdError(LsdErrorAlert(value.alert.cast()))
                }
                ffi::AlertType::DhtStats => {
                    Alert::DhtStats(DhtStatsAlert(value.alert.cast()))
                }
                ffi::AlertType::IncomingRequest => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::IncomingRequest(
                                IncomingRequestAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::DhtLog => Alert::DhtLog(DhtLogAlert(value.alert.cast())),
                ffi::AlertType::DhtPkt => Alert::DhtPkt(DhtPktAlert(value.alert.cast())),
                ffi::AlertType::DhtGetPeersReply => {
                    Alert::DhtGetPeersReply(DhtGetPeersReplyAlert(value.alert.cast()))
                }
                ffi::AlertType::DhtDirectResponse => {
                    Alert::DhtDirectResponse(DhtDirectResponseAlert(value.alert.cast()))
                }
                ffi::AlertType::PickerLog => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::PickerLog(PickerLogAlert(value.alert.cast())),
                        ),
                    )
                }
                ffi::AlertType::SessionError => {
                    Alert::SessionError(SessionErrorAlert(value.alert.cast()))
                }
                ffi::AlertType::DhtLiveNodes => {
                    Alert::DhtLiveNodes(DhtLiveNodesAlert(value.alert.cast()))
                }
                ffi::AlertType::SessionStatsHeader => {
                    Alert::SessionStatsHeader(
                        SessionStatsHeaderAlert(value.alert.cast()),
                    )
                }
                ffi::AlertType::DhtSampleInfohashes => {
                    Alert::DhtSampleInfohashes(
                        DhtSampleInfohashesAlert(value.alert.cast()),
                    )
                }
                ffi::AlertType::BlockUploaded => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerAlert(
                            PeerAlert::BlockUploaded(
                                BlockUploadedAlert(value.alert.cast()),
                            ),
                        ),
                    )
                }
                ffi::AlertType::AlertsDropped => {
                    Alert::AlertsDropped(AlertsDroppedAlert(value.alert.cast()))
                }
                ffi::AlertType::Socks5 => Alert::Socks5(Socks5Alert(value.alert.cast())),
                ffi::AlertType::FilePrio => {
                    Alert::TorrentAlert(
                        TorrentAlert::FilePrio(FilePrioAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::OversizedFile => {
                    Alert::TorrentAlert(
                        TorrentAlert::OversizedFile(
                            OversizedFileAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::TorrentConflict => {
                    Alert::TorrentAlert(
                        TorrentAlert::TorrentConflict(
                            TorrentConflictAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::PeerInfo => {
                    Alert::TorrentAlert(
                        TorrentAlert::PeerInfo(PeerInfoAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::FileProgress => {
                    Alert::TorrentAlert(
                        TorrentAlert::FileProgress(FileProgressAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::PieceInfo => {
                    Alert::TorrentAlert(
                        TorrentAlert::PieceInfo(PieceInfoAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::PieceAvailability => {
                    Alert::TorrentAlert(
                        TorrentAlert::PieceAvailability(
                            PieceAvailabilityAlert(value.alert.cast()),
                        ),
                    )
                }
                ffi::AlertType::TrackerList => {
                    Alert::TorrentAlert(
                        TorrentAlert::TrackerList(TrackerListAlert(value.alert.cast())),
                    )
                }
                ffi::AlertType::Unknown => Alert::NotImplemented,
                _ => Alert::NotImplemented,
            }
        }
    }
    impl TorrentRemovedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl ReadPieceAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl FileCompletedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl FileRenamedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl FileRenameFailedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl PerformanceAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl StateChangedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl TrackerErrorAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_tracker_alert<'a>(&'a self) -> TrackerAlertRaw<'a> {
            TrackerAlertRaw::new(self.0.cast())
        }
    }
    impl TrackerWarningAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_tracker_alert<'a>(&'a self) -> TrackerAlertRaw<'a> {
            TrackerAlertRaw::new(self.0.cast())
        }
    }
    impl ScrapeReplyAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_tracker_alert<'a>(&'a self) -> TrackerAlertRaw<'a> {
            TrackerAlertRaw::new(self.0.cast())
        }
    }
    impl ScrapeFailedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_tracker_alert<'a>(&'a self) -> TrackerAlertRaw<'a> {
            TrackerAlertRaw::new(self.0.cast())
        }
    }
    impl TrackerReplyAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_tracker_alert<'a>(&'a self) -> TrackerAlertRaw<'a> {
            TrackerAlertRaw::new(self.0.cast())
        }
    }
    impl DhtReplyAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_tracker_alert<'a>(&'a self) -> TrackerAlertRaw<'a> {
            TrackerAlertRaw::new(self.0.cast())
        }
    }
    impl TrackerAnnounceAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_tracker_alert<'a>(&'a self) -> TrackerAlertRaw<'a> {
            TrackerAlertRaw::new(self.0.cast())
        }
    }
    impl HashFailedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl PeerBanAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl PeerUnsnubbedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl PeerSnubbedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl PeerErrorAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl PeerConnectAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl PeerDisconnectedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl InvalidRequestAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl TorrentFinishedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl PieceFinishedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl RequestDroppedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl BlockTimeoutAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl BlockFinishedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl BlockDownloadingAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl UnwantedBlockAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl StorageMovedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl StorageMovedFailedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl TorrentDeletedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl TorrentDeleteFailedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl SaveResumeDataAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl SaveResumeDataFailedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl TorrentPausedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl TorrentResumedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl TorrentCheckedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl UrlSeedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl FileErrorAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl MetadataFailedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl MetadataReceivedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl UdpErrorAlert {}
    impl ExternalIpAlert {}
    impl ListenFailedAlert {}
    impl ListenSucceededAlert {}
    impl PortmapErrorAlert {}
    impl PortmapAlert {}
    impl PortmapLogAlert {}
    impl FastresumeRejectedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl PeerBlockedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl DhtAnnounceAlert {}
    impl DhtGetPeersAlert {}
    impl CacheFlushedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl LsdPeerAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl TrackeridAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_tracker_alert<'a>(&'a self) -> TrackerAlertRaw<'a> {
            TrackerAlertRaw::new(self.0.cast())
        }
    }
    impl DhtBootstrapAlert {}
    impl TorrentErrorAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl TorrentNeedCertAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl IncomingConnectionAlert {}
    impl AddTorrentAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl StateUpdateAlert {}
    impl SessionStatsAlert {}
    impl DhtErrorAlert {}
    impl DhtImmutableItemAlert {}
    impl DhtMutableItemAlert {}
    impl DhtPutAlert {}
    impl I2pAlert {}
    impl DhtOutgoingGetPeersAlert {}
    impl LogAlert {}
    impl TorrentLogAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl PeerLogAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl LsdErrorAlert {}
    impl DhtStatsAlert {}
    impl IncomingRequestAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl DhtLogAlert {}
    impl DhtPktAlert {}
    impl DhtGetPeersReplyAlert {}
    impl DhtDirectResponseAlert {}
    impl PickerLogAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl SessionErrorAlert {}
    impl DhtLiveNodesAlert {}
    impl SessionStatsHeaderAlert {}
    impl DhtSampleInfohashesAlert {}
    impl BlockUploadedAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
        fn as_peer_alert<'a>(&'a self) -> PeerAlertRaw<'a> {
            PeerAlertRaw::new(self.0.cast())
        }
    }
    impl AlertsDroppedAlert {}
    impl Socks5Alert {}
    impl FilePrioAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl OversizedFileAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl TorrentConflictAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl PeerInfoAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl FileProgressAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl PieceInfoAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl PieceAvailabilityAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl TrackerListAlert {
        fn as_torrent_alert<'a>(&'a self) -> TorrentAlertRaw<'a> {
            TorrentAlertRaw::new(self.0.cast())
        }
    }
    impl Alert {}
    pub enum AlertPriority {
        Normal = 0,
        High,
        Critical,
        Meta,
    }
    pub struct AlertCategory(
        <AlertCategory as ::bitflags::__private::PublicFlags>::Internal,
    );
    #[automatically_derived]
    impl ::core::fmt::Debug for AlertCategory {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "AlertCategory",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AlertCategory {
        #[inline]
        fn clone(&self) -> AlertCategory {
            let _: ::core::clone::AssertParamIsClone<
                <AlertCategory as ::bitflags::__private::PublicFlags>::Internal,
            >;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AlertCategory {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AlertCategory {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AlertCategory {
        #[inline]
        fn eq(&self, other: &AlertCategory) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for AlertCategory {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <AlertCategory as ::bitflags::__private::PublicFlags>::Internal,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for AlertCategory {
        #[inline]
        fn partial_cmp(
            &self,
            other: &AlertCategory,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for AlertCategory {
        #[inline]
        fn cmp(&self, other: &AlertCategory) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for AlertCategory {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    impl AlertCategory {
        #[allow(deprecated, non_upper_case_globals)]
        pub const Error: Self = Self::from_bits_retain(1 << 0);
        #[allow(deprecated, non_upper_case_globals)]
        pub const Peer: Self = Self::from_bits_retain(1 << 1);
        #[allow(deprecated, non_upper_case_globals)]
        pub const PortMapping: Self = Self::from_bits_retain(1 << 2);
        #[allow(deprecated, non_upper_case_globals)]
        pub const Storage: Self = Self::from_bits_retain(1 << 3);
        #[allow(deprecated, non_upper_case_globals)]
        pub const Tracker: Self = Self::from_bits_retain(1 << 4);
        #[allow(deprecated, non_upper_case_globals)]
        pub const Connect: Self = Self::from_bits_retain(1 << 5);
        #[allow(deprecated, non_upper_case_globals)]
        pub const Status: Self = Self::from_bits_retain(1 << 6);
        #[deprecated(note = "used only for libtorrent < 1.2")]
        #[allow(deprecated, non_upper_case_globals)]
        pub const Progress: Self = Self::from_bits_retain(1 << 7);
        #[allow(deprecated, non_upper_case_globals)]
        pub const IpBlock: Self = Self::from_bits_retain(1 << 8);
        #[allow(deprecated, non_upper_case_globals)]
        pub const PerformanceWarning: Self = Self::from_bits_retain(1 << 9);
        #[allow(deprecated, non_upper_case_globals)]
        pub const Dht: Self = Self::from_bits_retain(1 << 10);
        #[deprecated(note = "used only for libtorrent < 2.0")]
        #[allow(deprecated, non_upper_case_globals)]
        pub const Stats: Self = Self::from_bits_retain(1 << 11);
        #[allow(deprecated, non_upper_case_globals)]
        pub const SessionLog: Self = Self::from_bits_retain(1 << 13);
        #[allow(deprecated, non_upper_case_globals)]
        pub const TorrentLog: Self = Self::from_bits_retain(1 << 14);
        #[allow(deprecated, non_upper_case_globals)]
        pub const PeerLog: Self = Self::from_bits_retain(1 << 15);
        #[allow(deprecated, non_upper_case_globals)]
        pub const IncomingRequest: Self = Self::from_bits_retain(1 << 16);
        #[allow(deprecated, non_upper_case_globals)]
        pub const DhtLog: Self = Self::from_bits_retain(1 << 17);
        #[allow(deprecated, non_upper_case_globals)]
        pub const DhtOperation: Self = Self::from_bits_retain(1 << 18);
        #[allow(deprecated, non_upper_case_globals)]
        pub const PortMappingLog: Self = Self::from_bits_retain(1 << 19);
        #[allow(deprecated, non_upper_case_globals)]
        pub const PickerLog: Self = Self::from_bits_retain(1 << 20);
        #[allow(deprecated, non_upper_case_globals)]
        pub const FileProgress: Self = Self::from_bits_retain(1 << 21);
        #[allow(deprecated, non_upper_case_globals)]
        pub const PieceProgress: Self = Self::from_bits_retain(1 << 22);
        #[allow(deprecated, non_upper_case_globals)]
        pub const Upload: Self = Self::from_bits_retain(1 << 23);
        #[allow(deprecated, non_upper_case_globals)]
        pub const BlockProgress: Self = Self::from_bits_retain(1 << 24);
    }
    impl ::bitflags::Flags for AlertCategory {
        const FLAGS: &'static [::bitflags::Flag<AlertCategory>] = &[
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("Error", AlertCategory::Error)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("Peer", AlertCategory::Peer)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("PortMapping", AlertCategory::PortMapping)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("Storage", AlertCategory::Storage)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("Tracker", AlertCategory::Tracker)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("Connect", AlertCategory::Connect)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("Status", AlertCategory::Status)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("Progress", AlertCategory::Progress)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("IpBlock", AlertCategory::IpBlock)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new(
                    "PerformanceWarning",
                    AlertCategory::PerformanceWarning,
                )
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("Dht", AlertCategory::Dht)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("Stats", AlertCategory::Stats)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("SessionLog", AlertCategory::SessionLog)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("TorrentLog", AlertCategory::TorrentLog)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("PeerLog", AlertCategory::PeerLog)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("IncomingRequest", AlertCategory::IncomingRequest)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("DhtLog", AlertCategory::DhtLog)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("DhtOperation", AlertCategory::DhtOperation)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("PortMappingLog", AlertCategory::PortMappingLog)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("PickerLog", AlertCategory::PickerLog)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("FileProgress", AlertCategory::FileProgress)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("PieceProgress", AlertCategory::PieceProgress)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("Upload", AlertCategory::Upload)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("BlockProgress", AlertCategory::BlockProgress)
            },
        ];
        type Bits = u32;
        fn bits(&self) -> u32 {
            AlertCategory::bits(self)
        }
        fn from_bits_retain(bits: u32) -> AlertCategory {
            AlertCategory::from_bits_retain(bits)
        }
    }
    #[allow(
        dead_code,
        deprecated,
        unused_doc_comments,
        unused_attributes,
        unused_mut,
        unused_imports,
        non_upper_case_globals,
        clippy::assign_op_pattern,
        clippy::indexing_slicing,
        clippy::same_name_method,
        clippy::iter_without_into_iter,
    )]
    const _: () = {
        #[repr(transparent)]
        pub struct InternalBitFlags(u32);
        #[automatically_derived]
        impl ::core::clone::Clone for InternalBitFlags {
            #[inline]
            fn clone(&self) -> InternalBitFlags {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for InternalBitFlags {
            #[inline]
            fn eq(&self, other: &InternalBitFlags) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for InternalBitFlags {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for InternalBitFlags {
            #[inline]
            fn partial_cmp(
                &self,
                other: &InternalBitFlags,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for InternalBitFlags {
            #[inline]
            fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for InternalBitFlags {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        impl ::bitflags::__private::PublicFlags for AlertCategory {
            type Primitive = u32;
            type Internal = InternalBitFlags;
        }
        impl ::bitflags::__private::core::default::Default for InternalBitFlags {
            #[inline]
            fn default() -> Self {
                InternalBitFlags::empty()
            }
        }
        impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
            ) -> ::bitflags::__private::core::fmt::Result {
                if self.is_empty() {
                    f.write_fmt(format_args!("{0:#x}", <u32 as ::bitflags::Bits>::EMPTY))
                } else {
                    ::bitflags::__private::core::fmt::Display::fmt(self, f)
                }
            }
        }
        impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::parser::to_writer(&AlertCategory(*self), f)
            }
        }
        impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
            type Err = ::bitflags::parser::ParseError;
            fn from_str(
                s: &str,
            ) -> ::bitflags::__private::core::result::Result<Self, Self::Err> {
                ::bitflags::parser::from_str::<AlertCategory>(s).map(|flags| flags.0)
            }
        }
        impl ::bitflags::__private::core::convert::AsRef<u32> for InternalBitFlags {
            fn as_ref(&self) -> &u32 {
                &self.0
            }
        }
        impl ::bitflags::__private::core::convert::From<u32> for InternalBitFlags {
            fn from(bits: u32) -> Self {
                Self::from_bits_retain(bits)
            }
        }
        #[allow(dead_code, deprecated, unused_attributes)]
        impl InternalBitFlags {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                Self(<u32 as ::bitflags::Bits>::EMPTY)
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                let mut truncated = <u32 as ::bitflags::Bits>::EMPTY;
                let mut i = 0;
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <AlertCategory as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                let _ = i;
                Self(truncated)
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> u32 {
                self.0
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(
                bits: u32,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let truncated = Self::from_bits_truncate(bits).0;
                if truncated == bits {
                    ::bitflags::__private::core::option::Option::Some(Self(bits))
                } else {
                    ::bitflags::__private::core::option::Option::None
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: u32) -> Self {
                Self(bits & Self::all().0)
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: u32) -> Self {
                Self(bits)
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(
                name: &str,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                {
                    if name == "Error" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::Error.bits()),
                        );
                    }
                };
                {
                    if name == "Peer" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::Peer.bits()),
                        );
                    }
                };
                {
                    if name == "PortMapping" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::PortMapping.bits()),
                        );
                    }
                };
                {
                    if name == "Storage" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::Storage.bits()),
                        );
                    }
                };
                {
                    if name == "Tracker" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::Tracker.bits()),
                        );
                    }
                };
                {
                    if name == "Connect" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::Connect.bits()),
                        );
                    }
                };
                {
                    if name == "Status" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::Status.bits()),
                        );
                    }
                };
                {
                    if name == "Progress" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::Progress.bits()),
                        );
                    }
                };
                {
                    if name == "IpBlock" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::IpBlock.bits()),
                        );
                    }
                };
                {
                    if name == "PerformanceWarning" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::PerformanceWarning.bits()),
                        );
                    }
                };
                {
                    if name == "Dht" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::Dht.bits()),
                        );
                    }
                };
                {
                    if name == "Stats" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::Stats.bits()),
                        );
                    }
                };
                {
                    if name == "SessionLog" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::SessionLog.bits()),
                        );
                    }
                };
                {
                    if name == "TorrentLog" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::TorrentLog.bits()),
                        );
                    }
                };
                {
                    if name == "PeerLog" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::PeerLog.bits()),
                        );
                    }
                };
                {
                    if name == "IncomingRequest" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::IncomingRequest.bits()),
                        );
                    }
                };
                {
                    if name == "DhtLog" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::DhtLog.bits()),
                        );
                    }
                };
                {
                    if name == "DhtOperation" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::DhtOperation.bits()),
                        );
                    }
                };
                {
                    if name == "PortMappingLog" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::PortMappingLog.bits()),
                        );
                    }
                };
                {
                    if name == "PickerLog" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::PickerLog.bits()),
                        );
                    }
                };
                {
                    if name == "FileProgress" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::FileProgress.bits()),
                        );
                    }
                };
                {
                    if name == "PieceProgress" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::PieceProgress.bits()),
                        );
                    }
                };
                {
                    if name == "Upload" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::Upload.bits()),
                        );
                    }
                };
                {
                    if name == "BlockProgress" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(AlertCategory::BlockProgress.bits()),
                        );
                    }
                };
                let _ = name;
                ::bitflags::__private::core::option::Option::None
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.0 == <u32 as ::bitflags::Bits>::EMPTY
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                Self::all().0 | self.0 == self.0
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                self.0 & other.0 != <u32 as ::bitflags::Bits>::EMPTY
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                self.0 & other.0 == other.0
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                *self = Self(self.0).union(other);
            }
            /// The intersection of a source flags value with the complement of a target flags
            /// value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                *self = Self(self.0).difference(other);
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                *self = Self(self.0).symmetric_difference(other);
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                if value {
                    self.insert(other);
                } else {
                    self.remove(other);
                }
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
            /// The intersection of a source flags value with the complement of a target flags
            /// value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                Self(self.0 & !other.0)
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                Self(self.0 ^ other.0)
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                Self::from_bits_truncate(!self.0)
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: InternalBitFlags) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for InternalBitFlags {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for InternalBitFlags {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<InternalBitFlags>
        for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags>
        for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<
                T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
            >(iterator: T) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl InternalBitFlags {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<AlertCategory> {
                ::bitflags::iter::Iter::__private_const_new(
                    <AlertCategory as ::bitflags::Flags>::FLAGS,
                    AlertCategory::from_bits_retain(self.bits()),
                    AlertCategory::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(
                &self,
            ) -> ::bitflags::iter::IterNames<AlertCategory> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <AlertCategory as ::bitflags::Flags>::FLAGS,
                    AlertCategory::from_bits_retain(self.bits()),
                    AlertCategory::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for InternalBitFlags {
            type Item = AlertCategory;
            type IntoIter = ::bitflags::iter::Iter<AlertCategory>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
        impl InternalBitFlags {
            /// Returns a mutable reference to the raw value of the flags currently stored.
            #[inline]
            pub fn bits_mut(&mut self) -> &mut u32 {
                &mut self.0
            }
        }
        #[allow(dead_code, deprecated, unused_attributes)]
        impl AlertCategory {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                Self(InternalBitFlags::empty())
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                Self(InternalBitFlags::all())
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> u32 {
                self.0.bits()
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(
                bits: u32,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                match InternalBitFlags::from_bits(bits) {
                    ::bitflags::__private::core::option::Option::Some(bits) => {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    }
                    ::bitflags::__private::core::option::Option::None => {
                        ::bitflags::__private::core::option::Option::None
                    }
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: u32) -> Self {
                Self(InternalBitFlags::from_bits_truncate(bits))
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: u32) -> Self {
                Self(InternalBitFlags::from_bits_retain(bits))
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(
                name: &str,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                match InternalBitFlags::from_name(name) {
                    ::bitflags::__private::core::option::Option::Some(bits) => {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    }
                    ::bitflags::__private::core::option::Option::None => {
                        ::bitflags::__private::core::option::Option::None
                    }
                }
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                self.0.is_all()
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                self.0.intersects(other.0)
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                self.0.contains(other.0)
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                self.0.insert(other.0)
            }
            /// The intersection of a source flags value with the complement of a target flags
            /// value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                self.0.remove(other.0)
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                self.0.toggle(other.0)
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                self.0.set(other.0, value)
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                Self(self.0.intersection(other.0))
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                Self(self.0.union(other.0))
            }
            /// The intersection of a source flags value with the complement of a target flags
            /// value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                Self(self.0.difference(other.0))
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                Self(self.0.symmetric_difference(other.0))
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                Self(self.0.complement())
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for AlertCategory {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for AlertCategory {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for AlertCategory {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for AlertCategory {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for AlertCategory {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: AlertCategory) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for AlertCategory {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for AlertCategory {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for AlertCategory {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for AlertCategory {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for AlertCategory {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for AlertCategory {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for AlertCategory {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for AlertCategory {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<AlertCategory> for AlertCategory {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<AlertCategory>
        for AlertCategory {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<
                T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
            >(iterator: T) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl AlertCategory {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<AlertCategory> {
                ::bitflags::iter::Iter::__private_const_new(
                    <AlertCategory as ::bitflags::Flags>::FLAGS,
                    AlertCategory::from_bits_retain(self.bits()),
                    AlertCategory::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(
                &self,
            ) -> ::bitflags::iter::IterNames<AlertCategory> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <AlertCategory as ::bitflags::Flags>::FLAGS,
                    AlertCategory::from_bits_retain(self.bits()),
                    AlertCategory::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for AlertCategory {
            type Item = AlertCategory;
            type IntoIter = ::bitflags::iter::Iter<AlertCategory>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
    };
}
pub mod errors {
    use crate::ffi::error::ffi::self;
    pub enum LtrsError {
        LibtorrentError(LibtorrentError),
        HttpError(HttpError),
        GzipError(GzipError),
        I2pError(I2pError),
        PcpError(PcpError),
        BdecodeError(BdecodeError),
        SocksError(SocksError),
        UpnpError(UpnpError),
        Unknown(i32),
    }
    #[repr(u8)]
    pub enum LibtorrentError {
        /// Not an error
        NoError = 0,
        /// Two torrents has files which end up overwriting each other
        FileCollision,
        /// A piece did not match its piece hash
        FailedHashCheck,
        /// The .torrent file does not contain a bencoded dictionary at
        /// its top level
        TorrentIsNoDict,
        /// The .torrent file does not have an ``info`` dictionary
        TorrentMissingInfo,
        /// The .torrent file's ``info`` entry is not a dictionary
        TorrentInfoNoDict,
        /// The .torrent file does not have a ``piece length`` entry
        TorrentMissingPieceLength,
        /// The .torrent file does not have a ``name`` entry
        TorrentMissingName,
        /// The .torrent file's name entry is invalid
        TorrentInvalidName,
        /// The length of a file, or of the whole .torrent file is invalid.
        /// Either negative or not an integer
        TorrentInvalidLength,
        /// Failed to parse a file entry in the .torrent
        TorrentFileParseFailed,
        /// The ``pieces`` field is missing or invalid in the .torrent file
        TorrentMissingPieces,
        /// The ``pieces`` string has incorrect length
        TorrentInvalidHashes,
        /// The .torrent file has more pieces than is supported by libtorrent
        TooManyPiecesInTorrent,
        /// The metadata (.torrent file) that was received from the swarm
        /// matched the info-hash, but failed to be parsed
        InvalidSwarmMetadata,
        /// The file or buffer is not correctly bencoded
        InvalidBencoding,
        /// The .torrent file does not contain any files
        NoFilesInTorrent,
        /// The string was not properly url-encoded as expected
        InvalidEscapedString,
        /// Operation is not permitted since the session is shutting down
        SessionIsClosing,
        /// There's already a torrent with that info-hash added to the
        /// session
        DuplicateTorrent,
        /// The supplied torrent_handle is not referring to a valid torrent
        InvalidTorrentHandle,
        /// The type requested from the entry did not match its type
        InvalidEntryType,
        /// The specified URI does not contain a valid info-hash
        MissingInfoHashInUri,
        /// One of the files in the torrent was unexpectedly small. This
        /// might be caused by files being changed by an external process
        FileTooShort,
        /// The URL used an unknown protocol. Currently ``http`` and
        /// ``https`` (if built with openssl support) are recognized. For
        /// trackers ``udp`` is recognized as well.
        UnsupportedUrlProtocol,
        /// The URL did not conform to URL syntax and failed to be parsed
        UrlParseError,
        /// The peer sent a piece message of length 0
        PeerSentEmptyPiece,
        /// A bencoded structure was corrupt and failed to be parsed
        ParseFailed,
        /// The fast resume file was missing or had an invalid file version
        /// tag
        InvalidFileTag,
        /// The fast resume file was missing or had an invalid info-hash
        MissingInfoHash,
        /// The info-hash did not match the torrent
        MismatchingInfoHash,
        /// The URL contained an invalid hostname
        InvalidHostname,
        /// The URL had an invalid port
        InvalidPort,
        /// The port is blocked by the port-filter, and prevented the
        /// connection
        PortBlocked,
        /// The IPv6 address was expected to end with "]"
        ExpectedCloseBracketInAddress,
        /// The torrent is being destructed, preventing the operation to
        /// succeed
        DestructingTorrent,
        /// The connection timed out
        TimedOut,
        /// The peer is upload only, and we are upload only. There's no point
        /// in keeping the connection
        UploadUploadConnection,
        /// The peer is upload only, and we're not interested in it. There's
        /// no point in keeping the connection
        UninterestingUploadPeer,
        /// The peer sent an unknown info-hash
        InvalidInfoHash,
        /// The torrent is paused, preventing the operation from succeeding
        TorrentPaused,
        /// The peer sent an invalid have message, either wrong size or
        /// referring to a piece that doesn't exist in the torrent
        InvalidHave,
        /// The bitfield message had the incorrect size
        InvalidBitfieldSize,
        /// The peer kept requesting pieces after it was choked, possible
        /// abuse attempt.
        TooManyRequestsWhenChoked,
        /// The peer sent a piece message that does not correspond to a
        /// piece request sent by the client
        InvalidPiece,
        /// memory allocation failed
        NoMemory,
        /// The torrent is aborted, preventing the operation to succeed
        TorrentAborted,
        /// The peer is a connection to ourself, no point in keeping it
        SelfConnection,
        /// The peer sent a piece message with invalid size, either negative
        /// or greater than one block
        InvalidPieceSize,
        /// The peer has not been interesting or interested in us for too
        /// long, no point in keeping it around
        TimedOutNoInterest,
        /// The peer has not said anything in a long time, possibly dead
        TimedOutInactivity,
        /// The peer did not send a handshake within a reasonable amount of
        /// time, it might not be a bittorrent peer
        TimedOutNoHandshake,
        /// The peer has been unchoked for too long without requesting any
        /// data. It might be lying about its interest in us
        TimedOutNoRequest,
        /// The peer sent an invalid choke message
        InvalidChoke,
        /// The peer send an invalid unchoke message
        InvalidUnchoke,
        /// The peer sent an invalid interested message
        InvalidInterested,
        /// The peer sent an invalid not-interested message
        InvalidNotInterested,
        /// The peer sent an invalid piece request message
        InvalidRequest,
        /// The peer sent an invalid hash-list message (this is part of the
        /// merkle-torrent extension)
        InvalidHashList,
        /// The peer sent an invalid hash-piece message (this is part of the
        /// merkle-torrent extension)
        InvalidHashPiece,
        /// The peer sent an invalid cancel message
        InvalidCancel,
        /// The peer sent an invalid DHT port-message
        InvalidDhtPort,
        /// The peer sent an invalid suggest piece-message
        InvalidSuggest,
        /// The peer sent an invalid have all-message
        InvalidHaveAll,
        /// The peer sent an invalid have none-message
        InvalidHaveNone,
        /// The peer sent an invalid reject message
        InvalidReject,
        /// The peer sent an invalid allow fast-message
        InvalidAllowFast,
        /// The peer sent an invalid extension message ID
        InvalidExtended,
        /// The peer sent an invalid message ID
        InvalidMessage,
        /// The synchronization hash was not found in the encrypted handshake
        SyncHashNotFound,
        /// The encryption constant in the handshake is invalid
        InvalidEncryptionConstant,
        /// The peer does not support plain text, which is the selected mode
        NoPlaintextMode,
        /// The peer does not support RC4, which is the selected mode
        NoRc4Mode,
        /// The peer does not support any of the encryption modes that the
        /// client supports
        UnsupportedEncryptionMode,
        /// The peer selected an encryption mode that the client did not
        /// advertise and does not support
        UnsupportedEncryptionModeSelected,
        /// The pad size used in the encryption handshake is of invalid size
        InvalidPadSize,
        /// The encryption handshake is invalid
        InvalidEncryptHandshake,
        /// The client is set to not support incoming encrypted connections
        /// and this is an encrypted connection
        NoIncomingEncrypted,
        /// The client is set to not support incoming regular bittorrent
        /// connections, and this is a regular connection
        NoIncomingRegular,
        /// The client is already connected to this peer-ID
        DuplicatePeerId,
        /// Torrent was removed
        TorrentRemoved,
        /// The packet size exceeded the upper sanity check-limit
        PacketTooLarge,
        Reserved,
        /// The web server responded with an error
        HttpError,
        /// The web server response is missing a location header
        MissingLocation,
        /// The web seed redirected to a path that no longer matches the
        /// .torrent directory structure
        InvalidRedirection,
        /// The connection was closed because it redirected to a different
        /// URL
        Redirecting,
        /// The HTTP range header is invalid
        InvalidRange,
        /// The HTTP response did not have a content length
        NoContentLength,
        /// The IP is blocked by the IP filter
        BannedByIpFilter,
        /// At the connection limit
        TooManyConnections,
        /// The peer is marked as banned
        PeerBanned,
        /// The torrent is stopping, causing the operation to fail
        StoppingTorrent,
        /// The peer has sent too many corrupt pieces and is banned
        TooManyCorruptPieces,
        /// The torrent is not ready to receive peers
        TorrentNotReady,
        /// The peer is not completely constructed yet
        PeerNotConstructed,
        /// The session is closing, causing the operation to fail
        SessionClosing,
        /// The peer was disconnected in order to leave room for a
        /// potentially better peer
        OptimisticDisconnect,
        /// The torrent is finished
        TorrentFinished,
        /// No UPnP router found
        NoRouter,
        /// The metadata message says the metadata exceeds the limit
        MetadataTooLarge,
        /// The peer sent an invalid metadata request message
        InvalidMetadataRequest,
        /// The peer advertised an invalid metadata size
        InvalidMetadataSize,
        /// The peer sent a message with an invalid metadata offset
        InvalidMetadataOffset,
        /// The peer sent an invalid metadata message
        InvalidMetadataMessage,
        /// The peer sent a peer exchange message that was too large
        PexMessageTooLarge,
        /// The peer sent an invalid peer exchange message
        InvalidPexMessage,
        /// The peer sent an invalid tracker exchange message
        InvalidLtTrackerMessage,
        /// The peer sent pex messages too often. This is a possible
        /// attempt of and attack
        TooFrequentPex,
        /// The operation failed because it requires the torrent to have
        /// the metadata (.torrent file) and it doesn't have it yet.
        /// This happens for magnet links before they have downloaded the
        /// metadata, and also torrents added by URL.
        NoMetadata,
        /// The peer sent an invalid ``dont_have`` message. The don't have
        /// message is an extension to allow peers to advertise that the
        /// no longer has a piece they previously had.
        InvalidDontHave,
        /// The peer tried to connect to an SSL torrent without connecting
        /// over SSL.
        RequiresSslConnection,
        /// The peer tried to connect to a torrent with a certificate
        /// for a different torrent.
        InvalidSslCert,
        /// the torrent is not an SSL torrent, and the operation requires
        /// an SSL torrent
        NotAnSslTorrent,
        /// peer was banned because its listen port is within a banned port
        /// range, as specified by the port_filter.
        BannedByPortFilter,
        /// The session_handle is not referring to a valid session_impl
        InvalidSessionHandle,
        /// the listen socket associated with this request was closed
        InvalidListenSocket,
        InvalidHashRequest,
        InvalidHashes,
        InvalidHashReject,
        /// these error codes are deprecated, NAT-PMP/PCP error codes have
        /// been moved to their own category
        /// The resume data file is missing the ``file sizes`` entry
        MissingFileSizes = 130,
        /// The resume data file ``file sizes`` entry is empty
        NoFilesInResumeData,
        /// The resume data file is missing the ``pieces`` and ``slots`` entry
        MissingPieces,
        /// The number of files in the resume data does not match the number
        /// of files in the torrent
        MismatchingNumberOfFiles,
        /// One of the files on disk has a different size than in the fast
        /// resume file
        MismatchingFileSize,
        /// One of the files on disk has a different timestamp than in the
        /// fast resume file
        MismatchingFileTimestamp,
        /// The resume data file is not a dictionary
        NotADictionary,
        /// The ``blocks per piece`` entry is invalid in the resume data file
        InvalidBlocksPerPiece,
        /// The resume file is missing the ``slots`` entry, which is required
        /// for torrents with compact allocation. *DEPRECATED*
        MissingSlots,
        /// The resume file contains more slots than the torrent
        TooManySlots,
        /// The ``slot`` entry is invalid in the resume data
        InvalidSlotList,
        /// One index in the ``slot`` list is invalid
        InvalidPieceIndex,
        /// The pieces on disk needs to be re-ordered for the specified
        /// allocation mode. This happens if you specify sparse allocation
        /// and the files on disk are using compact storage. The pieces needs
        /// to be moved to their right position. *DEPRECATED*
        PiecesNeedReorder,
        /// this error is returned when asking to save resume data and
        /// specifying the flag to only save when there's anything new to save
        /// (torrent_handle::only_if_modified) and there wasn't anything changed.
        ResumeDataNotModified,
        /// the save_path in add_torrent_params is not valid
        InvalidSavePath,
        /// The HTTP header was not correctly formatted
        HttpParseError = 150,
        /// The HTTP response was in the 300-399 range but lacked a location
        /// header
        HttpMissingLocation,
        /// The HTTP response was encoded with gzip or deflate but
        /// decompressing it failed
        HttpFailedDecompress,
        /// The URL specified an i2p address, but no i2p router is configured
        NoI2pRouter = 160,
        /// i2p acceptor is not available yet, can't announce without endpoint
        NoI2pEndpoint,
        /// The tracker URL doesn't support transforming it into a scrape
        /// URL. i.e. it doesn't contain "announce.
        ScrapeNotAvailable = 170,
        /// invalid tracker response
        InvalidTrackerResponse,
        /// invalid peer dictionary entry. Not a dictionary
        InvalidPeerDict,
        /// tracker sent a failure message
        TrackerFailure,
        /// missing or invalid ``files`` entry
        InvalidFilesEntry,
        /// missing or invalid ``hash`` entry
        InvalidHashEntry,
        /// missing or invalid ``peers`` and ``peers6`` entry
        InvalidPeersEntry,
        /// UDP tracker response packet has invalid size
        InvalidTrackerResponseLength,
        /// invalid transaction id in UDP tracker response
        InvalidTrackerTransactionId,
        /// invalid action field in UDP tracker response
        InvalidTrackerAction,
        /// skipped announce (because it's assumed to be unreachable over the
        /// given source network interface)
        AnnounceSkipped,
        /// expected string in bencoded string
        /// random number generation failed
        NoEntropy = 200,
        /// blocked by SSRF mitigation
        SsrvMitigation,
        /// blocked because IDNA host names are banned
        BlockedByIdna,
        /// the torrent file has an unknown meta version
        TorrentUnknownVersion = 210,
        /// the v2 torrent file has no file tree
        TorrentMissingFileTree,
        /// the torrent contains v2 keys but does not specify meta version 2
        TorrentMissingMetaVersion,
        /// the v1 and v2 file metadata does not match
        TorrentInconsistentFiles,
        /// one or more files are missing piece layer hashes
        TorrentMissingPieceLayer,
        /// a piece layer has the wrong size or failed hash check
        TorrentInvalidPieceLayer,
        /// a v2 file entry has no root hash
        TorrentMissingPiecesRoot,
        /// the v1 and v2 hashes do not describe the same data
        TorrentInconsistentHashes,
        /// a file in the v2 metadata has the pad attribute set
        TorrentInvalidPadFile,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LibtorrentError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    LibtorrentError::NoError => "NoError",
                    LibtorrentError::FileCollision => "FileCollision",
                    LibtorrentError::FailedHashCheck => "FailedHashCheck",
                    LibtorrentError::TorrentIsNoDict => "TorrentIsNoDict",
                    LibtorrentError::TorrentMissingInfo => "TorrentMissingInfo",
                    LibtorrentError::TorrentInfoNoDict => "TorrentInfoNoDict",
                    LibtorrentError::TorrentMissingPieceLength => {
                        "TorrentMissingPieceLength"
                    }
                    LibtorrentError::TorrentMissingName => "TorrentMissingName",
                    LibtorrentError::TorrentInvalidName => "TorrentInvalidName",
                    LibtorrentError::TorrentInvalidLength => "TorrentInvalidLength",
                    LibtorrentError::TorrentFileParseFailed => "TorrentFileParseFailed",
                    LibtorrentError::TorrentMissingPieces => "TorrentMissingPieces",
                    LibtorrentError::TorrentInvalidHashes => "TorrentInvalidHashes",
                    LibtorrentError::TooManyPiecesInTorrent => "TooManyPiecesInTorrent",
                    LibtorrentError::InvalidSwarmMetadata => "InvalidSwarmMetadata",
                    LibtorrentError::InvalidBencoding => "InvalidBencoding",
                    LibtorrentError::NoFilesInTorrent => "NoFilesInTorrent",
                    LibtorrentError::InvalidEscapedString => "InvalidEscapedString",
                    LibtorrentError::SessionIsClosing => "SessionIsClosing",
                    LibtorrentError::DuplicateTorrent => "DuplicateTorrent",
                    LibtorrentError::InvalidTorrentHandle => "InvalidTorrentHandle",
                    LibtorrentError::InvalidEntryType => "InvalidEntryType",
                    LibtorrentError::MissingInfoHashInUri => "MissingInfoHashInUri",
                    LibtorrentError::FileTooShort => "FileTooShort",
                    LibtorrentError::UnsupportedUrlProtocol => "UnsupportedUrlProtocol",
                    LibtorrentError::UrlParseError => "UrlParseError",
                    LibtorrentError::PeerSentEmptyPiece => "PeerSentEmptyPiece",
                    LibtorrentError::ParseFailed => "ParseFailed",
                    LibtorrentError::InvalidFileTag => "InvalidFileTag",
                    LibtorrentError::MissingInfoHash => "MissingInfoHash",
                    LibtorrentError::MismatchingInfoHash => "MismatchingInfoHash",
                    LibtorrentError::InvalidHostname => "InvalidHostname",
                    LibtorrentError::InvalidPort => "InvalidPort",
                    LibtorrentError::PortBlocked => "PortBlocked",
                    LibtorrentError::ExpectedCloseBracketInAddress => {
                        "ExpectedCloseBracketInAddress"
                    }
                    LibtorrentError::DestructingTorrent => "DestructingTorrent",
                    LibtorrentError::TimedOut => "TimedOut",
                    LibtorrentError::UploadUploadConnection => "UploadUploadConnection",
                    LibtorrentError::UninterestingUploadPeer => "UninterestingUploadPeer",
                    LibtorrentError::InvalidInfoHash => "InvalidInfoHash",
                    LibtorrentError::TorrentPaused => "TorrentPaused",
                    LibtorrentError::InvalidHave => "InvalidHave",
                    LibtorrentError::InvalidBitfieldSize => "InvalidBitfieldSize",
                    LibtorrentError::TooManyRequestsWhenChoked => {
                        "TooManyRequestsWhenChoked"
                    }
                    LibtorrentError::InvalidPiece => "InvalidPiece",
                    LibtorrentError::NoMemory => "NoMemory",
                    LibtorrentError::TorrentAborted => "TorrentAborted",
                    LibtorrentError::SelfConnection => "SelfConnection",
                    LibtorrentError::InvalidPieceSize => "InvalidPieceSize",
                    LibtorrentError::TimedOutNoInterest => "TimedOutNoInterest",
                    LibtorrentError::TimedOutInactivity => "TimedOutInactivity",
                    LibtorrentError::TimedOutNoHandshake => "TimedOutNoHandshake",
                    LibtorrentError::TimedOutNoRequest => "TimedOutNoRequest",
                    LibtorrentError::InvalidChoke => "InvalidChoke",
                    LibtorrentError::InvalidUnchoke => "InvalidUnchoke",
                    LibtorrentError::InvalidInterested => "InvalidInterested",
                    LibtorrentError::InvalidNotInterested => "InvalidNotInterested",
                    LibtorrentError::InvalidRequest => "InvalidRequest",
                    LibtorrentError::InvalidHashList => "InvalidHashList",
                    LibtorrentError::InvalidHashPiece => "InvalidHashPiece",
                    LibtorrentError::InvalidCancel => "InvalidCancel",
                    LibtorrentError::InvalidDhtPort => "InvalidDhtPort",
                    LibtorrentError::InvalidSuggest => "InvalidSuggest",
                    LibtorrentError::InvalidHaveAll => "InvalidHaveAll",
                    LibtorrentError::InvalidHaveNone => "InvalidHaveNone",
                    LibtorrentError::InvalidReject => "InvalidReject",
                    LibtorrentError::InvalidAllowFast => "InvalidAllowFast",
                    LibtorrentError::InvalidExtended => "InvalidExtended",
                    LibtorrentError::InvalidMessage => "InvalidMessage",
                    LibtorrentError::SyncHashNotFound => "SyncHashNotFound",
                    LibtorrentError::InvalidEncryptionConstant => {
                        "InvalidEncryptionConstant"
                    }
                    LibtorrentError::NoPlaintextMode => "NoPlaintextMode",
                    LibtorrentError::NoRc4Mode => "NoRc4Mode",
                    LibtorrentError::UnsupportedEncryptionMode => {
                        "UnsupportedEncryptionMode"
                    }
                    LibtorrentError::UnsupportedEncryptionModeSelected => {
                        "UnsupportedEncryptionModeSelected"
                    }
                    LibtorrentError::InvalidPadSize => "InvalidPadSize",
                    LibtorrentError::InvalidEncryptHandshake => "InvalidEncryptHandshake",
                    LibtorrentError::NoIncomingEncrypted => "NoIncomingEncrypted",
                    LibtorrentError::NoIncomingRegular => "NoIncomingRegular",
                    LibtorrentError::DuplicatePeerId => "DuplicatePeerId",
                    LibtorrentError::TorrentRemoved => "TorrentRemoved",
                    LibtorrentError::PacketTooLarge => "PacketTooLarge",
                    LibtorrentError::Reserved => "Reserved",
                    LibtorrentError::HttpError => "HttpError",
                    LibtorrentError::MissingLocation => "MissingLocation",
                    LibtorrentError::InvalidRedirection => "InvalidRedirection",
                    LibtorrentError::Redirecting => "Redirecting",
                    LibtorrentError::InvalidRange => "InvalidRange",
                    LibtorrentError::NoContentLength => "NoContentLength",
                    LibtorrentError::BannedByIpFilter => "BannedByIpFilter",
                    LibtorrentError::TooManyConnections => "TooManyConnections",
                    LibtorrentError::PeerBanned => "PeerBanned",
                    LibtorrentError::StoppingTorrent => "StoppingTorrent",
                    LibtorrentError::TooManyCorruptPieces => "TooManyCorruptPieces",
                    LibtorrentError::TorrentNotReady => "TorrentNotReady",
                    LibtorrentError::PeerNotConstructed => "PeerNotConstructed",
                    LibtorrentError::SessionClosing => "SessionClosing",
                    LibtorrentError::OptimisticDisconnect => "OptimisticDisconnect",
                    LibtorrentError::TorrentFinished => "TorrentFinished",
                    LibtorrentError::NoRouter => "NoRouter",
                    LibtorrentError::MetadataTooLarge => "MetadataTooLarge",
                    LibtorrentError::InvalidMetadataRequest => "InvalidMetadataRequest",
                    LibtorrentError::InvalidMetadataSize => "InvalidMetadataSize",
                    LibtorrentError::InvalidMetadataOffset => "InvalidMetadataOffset",
                    LibtorrentError::InvalidMetadataMessage => "InvalidMetadataMessage",
                    LibtorrentError::PexMessageTooLarge => "PexMessageTooLarge",
                    LibtorrentError::InvalidPexMessage => "InvalidPexMessage",
                    LibtorrentError::InvalidLtTrackerMessage => "InvalidLtTrackerMessage",
                    LibtorrentError::TooFrequentPex => "TooFrequentPex",
                    LibtorrentError::NoMetadata => "NoMetadata",
                    LibtorrentError::InvalidDontHave => "InvalidDontHave",
                    LibtorrentError::RequiresSslConnection => "RequiresSslConnection",
                    LibtorrentError::InvalidSslCert => "InvalidSslCert",
                    LibtorrentError::NotAnSslTorrent => "NotAnSslTorrent",
                    LibtorrentError::BannedByPortFilter => "BannedByPortFilter",
                    LibtorrentError::InvalidSessionHandle => "InvalidSessionHandle",
                    LibtorrentError::InvalidListenSocket => "InvalidListenSocket",
                    LibtorrentError::InvalidHashRequest => "InvalidHashRequest",
                    LibtorrentError::InvalidHashes => "InvalidHashes",
                    LibtorrentError::InvalidHashReject => "InvalidHashReject",
                    LibtorrentError::MissingFileSizes => "MissingFileSizes",
                    LibtorrentError::NoFilesInResumeData => "NoFilesInResumeData",
                    LibtorrentError::MissingPieces => "MissingPieces",
                    LibtorrentError::MismatchingNumberOfFiles => {
                        "MismatchingNumberOfFiles"
                    }
                    LibtorrentError::MismatchingFileSize => "MismatchingFileSize",
                    LibtorrentError::MismatchingFileTimestamp => {
                        "MismatchingFileTimestamp"
                    }
                    LibtorrentError::NotADictionary => "NotADictionary",
                    LibtorrentError::InvalidBlocksPerPiece => "InvalidBlocksPerPiece",
                    LibtorrentError::MissingSlots => "MissingSlots",
                    LibtorrentError::TooManySlots => "TooManySlots",
                    LibtorrentError::InvalidSlotList => "InvalidSlotList",
                    LibtorrentError::InvalidPieceIndex => "InvalidPieceIndex",
                    LibtorrentError::PiecesNeedReorder => "PiecesNeedReorder",
                    LibtorrentError::ResumeDataNotModified => "ResumeDataNotModified",
                    LibtorrentError::InvalidSavePath => "InvalidSavePath",
                    LibtorrentError::HttpParseError => "HttpParseError",
                    LibtorrentError::HttpMissingLocation => "HttpMissingLocation",
                    LibtorrentError::HttpFailedDecompress => "HttpFailedDecompress",
                    LibtorrentError::NoI2pRouter => "NoI2pRouter",
                    LibtorrentError::NoI2pEndpoint => "NoI2pEndpoint",
                    LibtorrentError::ScrapeNotAvailable => "ScrapeNotAvailable",
                    LibtorrentError::InvalidTrackerResponse => "InvalidTrackerResponse",
                    LibtorrentError::InvalidPeerDict => "InvalidPeerDict",
                    LibtorrentError::TrackerFailure => "TrackerFailure",
                    LibtorrentError::InvalidFilesEntry => "InvalidFilesEntry",
                    LibtorrentError::InvalidHashEntry => "InvalidHashEntry",
                    LibtorrentError::InvalidPeersEntry => "InvalidPeersEntry",
                    LibtorrentError::InvalidTrackerResponseLength => {
                        "InvalidTrackerResponseLength"
                    }
                    LibtorrentError::InvalidTrackerTransactionId => {
                        "InvalidTrackerTransactionId"
                    }
                    LibtorrentError::InvalidTrackerAction => "InvalidTrackerAction",
                    LibtorrentError::AnnounceSkipped => "AnnounceSkipped",
                    LibtorrentError::NoEntropy => "NoEntropy",
                    LibtorrentError::SsrvMitigation => "SsrvMitigation",
                    LibtorrentError::BlockedByIdna => "BlockedByIdna",
                    LibtorrentError::TorrentUnknownVersion => "TorrentUnknownVersion",
                    LibtorrentError::TorrentMissingFileTree => "TorrentMissingFileTree",
                    LibtorrentError::TorrentMissingMetaVersion => {
                        "TorrentMissingMetaVersion"
                    }
                    LibtorrentError::TorrentInconsistentFiles => {
                        "TorrentInconsistentFiles"
                    }
                    LibtorrentError::TorrentMissingPieceLayer => {
                        "TorrentMissingPieceLayer"
                    }
                    LibtorrentError::TorrentInvalidPieceLayer => {
                        "TorrentInvalidPieceLayer"
                    }
                    LibtorrentError::TorrentMissingPiecesRoot => {
                        "TorrentMissingPiecesRoot"
                    }
                    LibtorrentError::TorrentInconsistentHashes => {
                        "TorrentInconsistentHashes"
                    }
                    LibtorrentError::TorrentInvalidPadFile => "TorrentInvalidPadFile",
                },
            )
        }
    }
    #[repr(u16)]
    pub enum HttpError {
        Cont = 100,
        Ok = 200,
        Created = 201,
        Accepted = 202,
        NoContent = 204,
        MultipleChoices = 300,
        MovedPermanently = 301,
        MovedTemporarily = 302,
        NotModified = 304,
        BadRequest = 400,
        Unauthorized = 401,
        Forbidden = 403,
        NotFound = 404,
        InternalServerError = 500,
        NotImplemented = 501,
        BadGateway = 502,
        ServiceUnavailable = 503,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for HttpError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    HttpError::Cont => "Cont",
                    HttpError::Ok => "Ok",
                    HttpError::Created => "Created",
                    HttpError::Accepted => "Accepted",
                    HttpError::NoContent => "NoContent",
                    HttpError::MultipleChoices => "MultipleChoices",
                    HttpError::MovedPermanently => "MovedPermanently",
                    HttpError::MovedTemporarily => "MovedTemporarily",
                    HttpError::NotModified => "NotModified",
                    HttpError::BadRequest => "BadRequest",
                    HttpError::Unauthorized => "Unauthorized",
                    HttpError::Forbidden => "Forbidden",
                    HttpError::NotFound => "NotFound",
                    HttpError::InternalServerError => "InternalServerError",
                    HttpError::NotImplemented => "NotImplemented",
                    HttpError::BadGateway => "BadGateway",
                    HttpError::ServiceUnavailable => "ServiceUnavailable",
                },
            )
        }
    }
    #[repr(u8)]
    pub enum I2pError {
        NoError = 0,
        ParseFailed,
        CantReachPeer,
        I2pError,
        InvalidKey,
        InvalidId,
        Timeout,
        KeyNotFound,
        DuplicatedId,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for I2pError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    I2pError::NoError => "NoError",
                    I2pError::ParseFailed => "ParseFailed",
                    I2pError::CantReachPeer => "CantReachPeer",
                    I2pError::I2pError => "I2pError",
                    I2pError::InvalidKey => "InvalidKey",
                    I2pError::InvalidId => "InvalidId",
                    I2pError::Timeout => "Timeout",
                    I2pError::KeyNotFound => "KeyNotFound",
                    I2pError::DuplicatedId => "DuplicatedId",
                },
            )
        }
    }
    #[repr(u8)]
    pub enum GzipError {
        NoError = 0,
        InvalidGzipHeader,
        InflatedDataTooLarge,
        DataDidNotTerminate,
        SpaceExhausted,
        InvalidBlockType,
        InvalidStoredBlockLength,
        TooManyLengthOrDistanceCodes,
        CodeLengthsCodesIncomplete,
        RepeatLengthsWithNoFirstLength,
        RepeatMoreThanSpecifiedLengths,
        InvalidLiteralLengthCodeLengths,
        InvalidDistanceCodeLengths,
        InvalidLiteralCodeInBlock,
        DistanceTooFarBackInBlock,
        UnknownGzipError,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GzipError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    GzipError::NoError => "NoError",
                    GzipError::InvalidGzipHeader => "InvalidGzipHeader",
                    GzipError::InflatedDataTooLarge => "InflatedDataTooLarge",
                    GzipError::DataDidNotTerminate => "DataDidNotTerminate",
                    GzipError::SpaceExhausted => "SpaceExhausted",
                    GzipError::InvalidBlockType => "InvalidBlockType",
                    GzipError::InvalidStoredBlockLength => "InvalidStoredBlockLength",
                    GzipError::TooManyLengthOrDistanceCodes => {
                        "TooManyLengthOrDistanceCodes"
                    }
                    GzipError::CodeLengthsCodesIncomplete => "CodeLengthsCodesIncomplete",
                    GzipError::RepeatLengthsWithNoFirstLength => {
                        "RepeatLengthsWithNoFirstLength"
                    }
                    GzipError::RepeatMoreThanSpecifiedLengths => {
                        "RepeatMoreThanSpecifiedLengths"
                    }
                    GzipError::InvalidLiteralLengthCodeLengths => {
                        "InvalidLiteralLengthCodeLengths"
                    }
                    GzipError::InvalidDistanceCodeLengths => "InvalidDistanceCodeLengths",
                    GzipError::InvalidLiteralCodeInBlock => "InvalidLiteralCodeInBlock",
                    GzipError::DistanceTooFarBackInBlock => "DistanceTooFarBackInBlock",
                    GzipError::UnknownGzipError => "UnknownGzipError",
                },
            )
        }
    }
    #[repr(u8)]
    pub enum PcpError {
        Success = 0,
        UnsupportedVersion,
        NotAuthorized,
        MalformedRequest,
        UnsupportedOpcode,
        UnsupportedOption,
        MalformedOption,
        NetworkFailure,
        NoResources,
        UnsupportedProtocol,
        UserExQuota,
        CannotProvideExternal,
        AddressMismatch,
        ExcessiveRemotePeers,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PcpError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    PcpError::Success => "Success",
                    PcpError::UnsupportedVersion => "UnsupportedVersion",
                    PcpError::NotAuthorized => "NotAuthorized",
                    PcpError::MalformedRequest => "MalformedRequest",
                    PcpError::UnsupportedOpcode => "UnsupportedOpcode",
                    PcpError::UnsupportedOption => "UnsupportedOption",
                    PcpError::MalformedOption => "MalformedOption",
                    PcpError::NetworkFailure => "NetworkFailure",
                    PcpError::NoResources => "NoResources",
                    PcpError::UnsupportedProtocol => "UnsupportedProtocol",
                    PcpError::UserExQuota => "UserExQuota",
                    PcpError::CannotProvideExternal => "CannotProvideExternal",
                    PcpError::AddressMismatch => "AddressMismatch",
                    PcpError::ExcessiveRemotePeers => "ExcessiveRemotePeers",
                },
            )
        }
    }
    #[repr(u8)]
    pub enum BdecodeError {
        NoError = 0,
        ExpectedDigit,
        ExpectedColon,
        UnexpectedEof,
        ExpectedValue,
        DepthExceeded,
        LimitExceeded,
        Overflow,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BdecodeError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    BdecodeError::NoError => "NoError",
                    BdecodeError::ExpectedDigit => "ExpectedDigit",
                    BdecodeError::ExpectedColon => "ExpectedColon",
                    BdecodeError::UnexpectedEof => "UnexpectedEof",
                    BdecodeError::ExpectedValue => "ExpectedValue",
                    BdecodeError::DepthExceeded => "DepthExceeded",
                    BdecodeError::LimitExceeded => "LimitExceeded",
                    BdecodeError::Overflow => "Overflow",
                },
            )
        }
    }
    #[repr(u8)]
    pub enum SocksError {
        NoError = 0,
        UnsupportedVersion,
        UnsupportedAuthenticationMethod,
        UnsupportedAuthenticationVersion,
        AuthenticationError,
        UsernameRequired,
        GeneralFailure,
        CommandNotSupported,
        NoIdentd,
        IdentdError,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SocksError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    SocksError::NoError => "NoError",
                    SocksError::UnsupportedVersion => "UnsupportedVersion",
                    SocksError::UnsupportedAuthenticationMethod => {
                        "UnsupportedAuthenticationMethod"
                    }
                    SocksError::UnsupportedAuthenticationVersion => {
                        "UnsupportedAuthenticationVersion"
                    }
                    SocksError::AuthenticationError => "AuthenticationError",
                    SocksError::UsernameRequired => "UsernameRequired",
                    SocksError::GeneralFailure => "GeneralFailure",
                    SocksError::CommandNotSupported => "CommandNotSupported",
                    SocksError::NoIdentd => "NoIdentd",
                    SocksError::IdentdError => "IdentdError",
                },
            )
        }
    }
    #[repr(u16)]
    pub enum UpnpError {
        NoError = 0,
        InvalidArgument = 402,
        ActionFailed = 501,
        ValueNotInArray = 714,
        SourceIpCannotBeWildcarded = 715,
        ExternalPortCannotBeWildcarded = 716,
        PortMappingConflict = 718,
        InternalPortMustMatchExternal = 724,
        OnlyPermanentLeasesSupported = 725,
        RemoteHostMustBeWildcarded = 726,
        ExternalPortMustBeWildcarded = 727,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UpnpError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    UpnpError::NoError => "NoError",
                    UpnpError::InvalidArgument => "InvalidArgument",
                    UpnpError::ActionFailed => "ActionFailed",
                    UpnpError::ValueNotInArray => "ValueNotInArray",
                    UpnpError::SourceIpCannotBeWildcarded => "SourceIpCannotBeWildcarded",
                    UpnpError::ExternalPortCannotBeWildcarded => {
                        "ExternalPortCannotBeWildcarded"
                    }
                    UpnpError::PortMappingConflict => "PortMappingConflict",
                    UpnpError::InternalPortMustMatchExternal => {
                        "InternalPortMustMatchExternal"
                    }
                    UpnpError::OnlyPermanentLeasesSupported => {
                        "OnlyPermanentLeasesSupported"
                    }
                    UpnpError::RemoteHostMustBeWildcarded => "RemoteHostMustBeWildcarded",
                    UpnpError::ExternalPortMustBeWildcarded => {
                        "ExternalPortMustBeWildcarded"
                    }
                },
            )
        }
    }
    impl From<ffi::Error> for LtrsError {
        fn from(err: ffi::Error) -> Self {
            unsafe {
                match err.category {
                    ffi::ErrorCategory::LibtorrentError => {
                        LtrsError::LibtorrentError(std::mem::transmute(err.code as u8))
                    }
                    ffi::ErrorCategory::HttpError => {
                        LtrsError::HttpError(std::mem::transmute(err.code as u16))
                    }
                    ffi::ErrorCategory::GzipError => {
                        LtrsError::GzipError(std::mem::transmute(err.code as u8))
                    }
                    ffi::ErrorCategory::PcpError => {
                        LtrsError::PcpError(std::mem::transmute(err.code as u8))
                    }
                    ffi::ErrorCategory::BdecodeError => {
                        LtrsError::BdecodeError(std::mem::transmute(err.code as u8))
                    }
                    ffi::ErrorCategory::SocksError => {
                        LtrsError::SocksError(std::mem::transmute(err.code as u8))
                    }
                    ffi::ErrorCategory::UpnpError => {
                        LtrsError::UpnpError(std::mem::transmute(err.code as u16))
                    }
                    ffi::ErrorCategory::I2pError => {
                        LtrsError::I2pError(std::mem::transmute(err.code as u8))
                    }
                    _ => LtrsError::Unknown(err.code),
                }
            }
        }
    }
}
pub mod info_hash {
    use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
    use crate::ffi::ffi;
    pub enum InfoHash {
        V1([u8; 20]),
        V2([u8; 32]),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InfoHash {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                InfoHash::V1(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V1", &__self_0)
                }
                InfoHash::V2(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V2", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InfoHash {
        #[inline]
        fn clone(&self) -> InfoHash {
            let _: ::core::clone::AssertParamIsClone<[u8; 20]>;
            let _: ::core::clone::AssertParamIsClone<[u8; 32]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for InfoHash {}
    impl InfoHash {
        pub fn as_base64(&self) -> String {
            match self {
                InfoHash::V1(v1) => BASE64_URL_SAFE_NO_PAD.encode(v1),
                InfoHash::V2(v2) => BASE64_URL_SAFE_NO_PAD.encode(v2),
            }
        }
        fn from_hex<const N: usize>(hex: &str) -> Result<[u8; N], ()> {
            if hex.len() != N * 2 {
                return Err(());
            }
            let mut bytes = [0u8; N];
            for i in 0..N {
                let hi = hex_char_to_val(hex.as_bytes()[i * 2])?;
                let lo = hex_char_to_val(hex.as_bytes()[i * 2 + 1])?;
                bytes[i] = (hi << 4) | lo;
            }
            Ok(bytes)
        }
        fn from_base32(input: &str) -> Result<[u8; 20], ()> {
            let mut bits = 0u64;
            let mut bit_count = 0;
            let mut out = [0u8; 20];
            let mut byte_index = 0;
            for c in input.chars() {
                let val = base32_char_to_val(c)?;
                bits = (bits << 5) | (val as u64);
                bit_count += 5;
                while bit_count >= 8 {
                    bit_count -= 8;
                    if byte_index >= 20 {
                        return Err(());
                    }
                    out[byte_index] = ((bits >> bit_count) & 0xFF) as u8;
                    byte_index += 1;
                }
            }
            if byte_index != 20 {
                return Err(());
            }
            Ok(out)
        }
        pub fn from_magnet(magnet_uri: &str) -> Result<Self, ()> {
            let xt_start = magnet_uri.find("xt=urn:").ok_or(())?;
            let xt = &magnet_uri[xt_start + 7..];
            let end = xt.find('&').unwrap_or(xt.len());
            let urn = &xt[..end];
            if let Some(rest) = urn.strip_prefix("btih:") {
                if rest.len() == 40 {
                    let bytes = Self::from_hex::<20>(rest)?;
                    return Ok(InfoHash::V1(bytes));
                } else if rest.len() == 32 {
                    let bytes = Self::from_base32(rest)?;
                    return Ok(InfoHash::V1(bytes));
                } else {
                    return Err(());
                }
            } else if let Some(rest) = urn.strip_prefix("btmh:") {
                if let Some(rest) = rest.strip_prefix("1220") {
                    let bytes = Self::from_hex::<32>(rest)?;
                    return Ok(InfoHash::V2(bytes));
                } else {
                    return Err(());
                }
            }
            Err(())
        }
    }
    fn hex_char_to_val(c: u8) -> Result<u8, ()> {
        match c {
            b'0'..=b'9' => Ok(c - b'0'),
            b'a'..=b'f' => Ok(c - b'a' + 10),
            b'A'..=b'F' => Ok(c - b'A' + 10),
            _ => Err(()),
        }
    }
    fn base32_char_to_val(c: char) -> Result<u8, ()> {
        match c {
            'A'..='Z' => Ok((c as u8) - b'A'),
            '2'..='7' => Ok((c as u8) - b'2' + 26),
            _ => Err(()),
        }
    }
    impl std::hash::Hash for InfoHash {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            match self {
                InfoHash::V1(v1) => v1.hash(state),
                InfoHash::V2(v2) => v2.hash(state),
            }
        }
    }
    impl PartialEq for InfoHash {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (InfoHash::V1(lhs), InfoHash::V1(rhs)) => lhs == rhs,
                (InfoHash::V2(lhs), InfoHash::V2(rhs)) => lhs == rhs,
                _ => false,
            }
        }
    }
    impl Eq for InfoHash {}
    impl From<ffi::InfoHashCpp> for InfoHash {
        fn from(value: ffi::InfoHashCpp) -> Self {
            match value.version {
                1 => InfoHash::V1(value.inner[..20].try_into().unwrap()),
                2 => InfoHash::V2(value.inner),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
}
pub mod session {
    use std::mem;
    use cxx::UniquePtr;
    use crate::{
        add_torrent_params::AddTorrentParams, alerts::Alert, ffi::ffi,
        settings_pack::SettingsPack, torrent_handle::{StatusFlags, TorrentHandle},
    };
    pub struct LtSession {
        inner: UniquePtr<ffi::session>,
        alerts: Vec<Alert>,
    }
    impl LtSession {
        pub fn new() -> LtSession {
            LtSession {
                inner: ffi::lt_create_session(),
                alerts: Vec::new(),
            }
        }
        pub fn new_with_settings(settings: &SettingsPack) -> LtSession {
            LtSession {
                inner: ffi::lt_create_session_with_settings(settings.inner()),
                alerts: Vec::new(),
            }
        }
        pub fn add_torrent<'a>(
            &'a mut self,
            _params: &AddTorrentParams,
        ) -> TorrentHandle {
            ::core::panicking::panic("not implemented")
        }
        pub fn async_add_torrent(&mut self, params: &AddTorrentParams) {
            ffi::lt_session_async_add_torrent(self.inner.pin_mut(), params.inner());
        }
        pub fn pop_alerts(&mut self) {
            let alerts = ffi::lt_session_pop_alerts(self.inner.pin_mut());
            self.alerts.clear();
            for alert in alerts {
                self.alerts.push(alert.into());
            }
        }
        pub fn alerts(&self) -> &Vec<Alert> {
            &self.alerts
        }
        /// This functions instructs the session to post the state_update_alert, containing the status of
        /// all torrents whose state changed since the last time this function was called.
        ///
        /// Only torrents who has the state subscription flag set will be included. This flag is on by default.
        pub fn post_torrent_updates(&mut self, flags: StatusFlags) {
            ffi::lt_session_post_torrent_updates(self.inner.pin_mut(), flags.bits());
        }
        /// Marked as unsafe because it takes ownership of the alerts. If the session pops alerts again
        /// the alerts will become invalid.
        ///
        /// As long [`LtSession::pop_alerts()`] is not called again the alerts are valid
        pub unsafe fn take_alerts(&mut self) -> Vec<Alert> {
            let alerts = mem::replace(&mut self.alerts, Vec::new());
            alerts
        }
    }
    unsafe impl Send for LtSession {}
}
pub mod settings_pack {
    use cxx::UniquePtr;
    use crate::{alerts::AlertCategory, ffi::ffi};
    pub struct SettingsPack {
        inner: UniquePtr<ffi::settings_pack>,
    }
    impl SettingsPack {
        pub fn new() -> SettingsPack {
            SettingsPack {
                inner: ffi::lt_create_settings_pack(),
            }
        }
        pub(crate) fn inner(&self) -> &ffi::settings_pack {
            &self.inner
        }
        pub fn set_alert_mask(&mut self, mask: AlertCategory) {
            ffi::lt_set_alert_mask(self.inner.pin_mut(), mask.bits());
        }
    }
    impl From<UniquePtr<ffi::settings_pack>> for SettingsPack {
        fn from(inner: UniquePtr<ffi::settings_pack>) -> SettingsPack {
            SettingsPack { inner }
        }
    }
    unsafe impl Send for SettingsPack {}
}
pub mod torrent_handle {
    use cxx::UniquePtr;
    use crate::alerts::PieceIndex;
    use crate::ffi::torrent_handle::ffi::{
        torrent_handle, torrent_handle_in_session, torrent_handle_info_hashes,
        torrent_handle_read_piece, torrent_handle_save_resume_data,
    };
    use crate::info_hash::InfoHash;
    use crate::torrent_status::TorrentStatus;
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
            torrent_handle_read_piece(&self.0, piece);
        }
        pub fn status(&self) -> TorrentStatus {
            ::core::panicking::panic("not implemented")
        }
        /// Returns the info-hash(es) of the torrent. If this handle is to a torrent that hasn't loaded
        /// yet (for instance by being added) by a URL, the returned value is undefined.
        pub fn info_hashes(&self) -> InfoHash {
            torrent_handle_info_hashes(&self.0).into()
        }
    }
    unsafe impl<'a> Send for TorrentHandle {}
    impl<'a> std::fmt::Debug for TorrentHandle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TorrentHandle").finish()
        }
    }
    pub struct StatusFlags(
        <StatusFlags as ::bitflags::__private::PublicFlags>::Internal,
    );
    #[automatically_derived]
    impl ::core::fmt::Debug for StatusFlags {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "StatusFlags", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StatusFlags {
        #[inline]
        fn clone(&self) -> StatusFlags {
            let _: ::core::clone::AssertParamIsClone<
                <StatusFlags as ::bitflags::__private::PublicFlags>::Internal,
            >;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for StatusFlags {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StatusFlags {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StatusFlags {
        #[inline]
        fn eq(&self, other: &StatusFlags) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StatusFlags {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <StatusFlags as ::bitflags::__private::PublicFlags>::Internal,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StatusFlags {
        #[inline]
        fn partial_cmp(
            &self,
            other: &StatusFlags,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StatusFlags {
        #[inline]
        fn cmp(&self, other: &StatusFlags) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for StatusFlags {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    impl StatusFlags {
        #[allow(deprecated, non_upper_case_globals)]
        pub const QueryDistributedCopies: Self = Self::from_bits_retain(1 << 0);
        #[allow(deprecated, non_upper_case_globals)]
        pub const QueryAccurateDownloadCounters: Self = Self::from_bits_retain(1 << 1);
        #[allow(deprecated, non_upper_case_globals)]
        pub const QueryLastSeenComplete: Self = Self::from_bits_retain(1 << 2);
        #[allow(deprecated, non_upper_case_globals)]
        pub const QueryPieces: Self = Self::from_bits_retain(1 << 3);
        #[allow(deprecated, non_upper_case_globals)]
        pub const QueryVerifiedPieces: Self = Self::from_bits_retain(1 << 4);
        #[allow(deprecated, non_upper_case_globals)]
        pub const QueryTorrentFile: Self = Self::from_bits_retain(1 << 5);
        #[allow(deprecated, non_upper_case_globals)]
        pub const QueryName: Self = Self::from_bits_retain(1 << 6);
        #[allow(deprecated, non_upper_case_globals)]
        pub const QuerySavePath: Self = Self::from_bits_retain(1 << 7);
    }
    impl ::bitflags::Flags for StatusFlags {
        const FLAGS: &'static [::bitflags::Flag<StatusFlags>] = &[
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new(
                    "QueryDistributedCopies",
                    StatusFlags::QueryDistributedCopies,
                )
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new(
                    "QueryAccurateDownloadCounters",
                    StatusFlags::QueryAccurateDownloadCounters,
                )
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new(
                    "QueryLastSeenComplete",
                    StatusFlags::QueryLastSeenComplete,
                )
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("QueryPieces", StatusFlags::QueryPieces)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new(
                    "QueryVerifiedPieces",
                    StatusFlags::QueryVerifiedPieces,
                )
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("QueryTorrentFile", StatusFlags::QueryTorrentFile)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("QueryName", StatusFlags::QueryName)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("QuerySavePath", StatusFlags::QuerySavePath)
            },
        ];
        type Bits = u32;
        fn bits(&self) -> u32 {
            StatusFlags::bits(self)
        }
        fn from_bits_retain(bits: u32) -> StatusFlags {
            StatusFlags::from_bits_retain(bits)
        }
    }
    #[allow(
        dead_code,
        deprecated,
        unused_doc_comments,
        unused_attributes,
        unused_mut,
        unused_imports,
        non_upper_case_globals,
        clippy::assign_op_pattern,
        clippy::indexing_slicing,
        clippy::same_name_method,
        clippy::iter_without_into_iter,
    )]
    const _: () = {
        #[repr(transparent)]
        pub struct InternalBitFlags(u32);
        #[automatically_derived]
        impl ::core::clone::Clone for InternalBitFlags {
            #[inline]
            fn clone(&self) -> InternalBitFlags {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for InternalBitFlags {
            #[inline]
            fn eq(&self, other: &InternalBitFlags) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for InternalBitFlags {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for InternalBitFlags {
            #[inline]
            fn partial_cmp(
                &self,
                other: &InternalBitFlags,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for InternalBitFlags {
            #[inline]
            fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for InternalBitFlags {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        impl ::bitflags::__private::PublicFlags for StatusFlags {
            type Primitive = u32;
            type Internal = InternalBitFlags;
        }
        impl ::bitflags::__private::core::default::Default for InternalBitFlags {
            #[inline]
            fn default() -> Self {
                InternalBitFlags::empty()
            }
        }
        impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
            ) -> ::bitflags::__private::core::fmt::Result {
                if self.is_empty() {
                    f.write_fmt(format_args!("{0:#x}", <u32 as ::bitflags::Bits>::EMPTY))
                } else {
                    ::bitflags::__private::core::fmt::Display::fmt(self, f)
                }
            }
        }
        impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::parser::to_writer(&StatusFlags(*self), f)
            }
        }
        impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
            type Err = ::bitflags::parser::ParseError;
            fn from_str(
                s: &str,
            ) -> ::bitflags::__private::core::result::Result<Self, Self::Err> {
                ::bitflags::parser::from_str::<StatusFlags>(s).map(|flags| flags.0)
            }
        }
        impl ::bitflags::__private::core::convert::AsRef<u32> for InternalBitFlags {
            fn as_ref(&self) -> &u32 {
                &self.0
            }
        }
        impl ::bitflags::__private::core::convert::From<u32> for InternalBitFlags {
            fn from(bits: u32) -> Self {
                Self::from_bits_retain(bits)
            }
        }
        #[allow(dead_code, deprecated, unused_attributes)]
        impl InternalBitFlags {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                Self(<u32 as ::bitflags::Bits>::EMPTY)
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                let mut truncated = <u32 as ::bitflags::Bits>::EMPTY;
                let mut i = 0;
                {
                    {
                        let flag = <StatusFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <StatusFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <StatusFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <StatusFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <StatusFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <StatusFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <StatusFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <StatusFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                let _ = i;
                Self(truncated)
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> u32 {
                self.0
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(
                bits: u32,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let truncated = Self::from_bits_truncate(bits).0;
                if truncated == bits {
                    ::bitflags::__private::core::option::Option::Some(Self(bits))
                } else {
                    ::bitflags::__private::core::option::Option::None
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: u32) -> Self {
                Self(bits & Self::all().0)
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: u32) -> Self {
                Self(bits)
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(
                name: &str,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                {
                    if name == "QueryDistributedCopies" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(StatusFlags::QueryDistributedCopies.bits()),
                        );
                    }
                };
                {
                    if name == "QueryAccurateDownloadCounters" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(StatusFlags::QueryAccurateDownloadCounters.bits()),
                        );
                    }
                };
                {
                    if name == "QueryLastSeenComplete" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(StatusFlags::QueryLastSeenComplete.bits()),
                        );
                    }
                };
                {
                    if name == "QueryPieces" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(StatusFlags::QueryPieces.bits()),
                        );
                    }
                };
                {
                    if name == "QueryVerifiedPieces" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(StatusFlags::QueryVerifiedPieces.bits()),
                        );
                    }
                };
                {
                    if name == "QueryTorrentFile" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(StatusFlags::QueryTorrentFile.bits()),
                        );
                    }
                };
                {
                    if name == "QueryName" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(StatusFlags::QueryName.bits()),
                        );
                    }
                };
                {
                    if name == "QuerySavePath" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(StatusFlags::QuerySavePath.bits()),
                        );
                    }
                };
                let _ = name;
                ::bitflags::__private::core::option::Option::None
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.0 == <u32 as ::bitflags::Bits>::EMPTY
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                Self::all().0 | self.0 == self.0
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                self.0 & other.0 != <u32 as ::bitflags::Bits>::EMPTY
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                self.0 & other.0 == other.0
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                *self = Self(self.0).union(other);
            }
            /// The intersection of a source flags value with the complement of a target flags
            /// value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                *self = Self(self.0).difference(other);
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                *self = Self(self.0).symmetric_difference(other);
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                if value {
                    self.insert(other);
                } else {
                    self.remove(other);
                }
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
            /// The intersection of a source flags value with the complement of a target flags
            /// value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                Self(self.0 & !other.0)
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                Self(self.0 ^ other.0)
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                Self::from_bits_truncate(!self.0)
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: InternalBitFlags) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for InternalBitFlags {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for InternalBitFlags {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<InternalBitFlags>
        for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags>
        for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<
                T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
            >(iterator: T) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl InternalBitFlags {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<StatusFlags> {
                ::bitflags::iter::Iter::__private_const_new(
                    <StatusFlags as ::bitflags::Flags>::FLAGS,
                    StatusFlags::from_bits_retain(self.bits()),
                    StatusFlags::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<StatusFlags> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <StatusFlags as ::bitflags::Flags>::FLAGS,
                    StatusFlags::from_bits_retain(self.bits()),
                    StatusFlags::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for InternalBitFlags {
            type Item = StatusFlags;
            type IntoIter = ::bitflags::iter::Iter<StatusFlags>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
        impl InternalBitFlags {
            /// Returns a mutable reference to the raw value of the flags currently stored.
            #[inline]
            pub fn bits_mut(&mut self) -> &mut u32 {
                &mut self.0
            }
        }
        #[allow(dead_code, deprecated, unused_attributes)]
        impl StatusFlags {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                Self(InternalBitFlags::empty())
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                Self(InternalBitFlags::all())
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> u32 {
                self.0.bits()
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(
                bits: u32,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                match InternalBitFlags::from_bits(bits) {
                    ::bitflags::__private::core::option::Option::Some(bits) => {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    }
                    ::bitflags::__private::core::option::Option::None => {
                        ::bitflags::__private::core::option::Option::None
                    }
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: u32) -> Self {
                Self(InternalBitFlags::from_bits_truncate(bits))
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: u32) -> Self {
                Self(InternalBitFlags::from_bits_retain(bits))
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(
                name: &str,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                match InternalBitFlags::from_name(name) {
                    ::bitflags::__private::core::option::Option::Some(bits) => {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    }
                    ::bitflags::__private::core::option::Option::None => {
                        ::bitflags::__private::core::option::Option::None
                    }
                }
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                self.0.is_all()
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                self.0.intersects(other.0)
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                self.0.contains(other.0)
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                self.0.insert(other.0)
            }
            /// The intersection of a source flags value with the complement of a target flags
            /// value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                self.0.remove(other.0)
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                self.0.toggle(other.0)
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                self.0.set(other.0, value)
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                Self(self.0.intersection(other.0))
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                Self(self.0.union(other.0))
            }
            /// The intersection of a source flags value with the complement of a target flags
            /// value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                Self(self.0.difference(other.0))
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                Self(self.0.symmetric_difference(other.0))
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                Self(self.0.complement())
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for StatusFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for StatusFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for StatusFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for StatusFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for StatusFlags {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: StatusFlags) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for StatusFlags {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for StatusFlags {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for StatusFlags {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for StatusFlags {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for StatusFlags {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for StatusFlags {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for StatusFlags {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for StatusFlags {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<StatusFlags> for StatusFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<StatusFlags>
        for StatusFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<
                T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
            >(iterator: T) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl StatusFlags {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<StatusFlags> {
                ::bitflags::iter::Iter::__private_const_new(
                    <StatusFlags as ::bitflags::Flags>::FLAGS,
                    StatusFlags::from_bits_retain(self.bits()),
                    StatusFlags::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<StatusFlags> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <StatusFlags as ::bitflags::Flags>::FLAGS,
                    StatusFlags::from_bits_retain(self.bits()),
                    StatusFlags::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for StatusFlags {
            type Item = StatusFlags;
            type IntoIter = ::bitflags::iter::Iter<StatusFlags>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
    };
    pub struct ResumeDataFlags(
        <ResumeDataFlags as ::bitflags::__private::PublicFlags>::Internal,
    );
    #[automatically_derived]
    impl ::core::fmt::Debug for ResumeDataFlags {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "ResumeDataFlags",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ResumeDataFlags {
        #[inline]
        fn clone(&self) -> ResumeDataFlags {
            let _: ::core::clone::AssertParamIsClone<
                <ResumeDataFlags as ::bitflags::__private::PublicFlags>::Internal,
            >;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ResumeDataFlags {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ResumeDataFlags {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ResumeDataFlags {
        #[inline]
        fn eq(&self, other: &ResumeDataFlags) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ResumeDataFlags {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <ResumeDataFlags as ::bitflags::__private::PublicFlags>::Internal,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ResumeDataFlags {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ResumeDataFlags,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ResumeDataFlags {
        #[inline]
        fn cmp(&self, other: &ResumeDataFlags) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ResumeDataFlags {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    impl ResumeDataFlags {
        /// The disk cache will be flushed before creating the resume data.
        /// This avoids a problem with file timestamps in the resume data in
        /// case the cache hasn't been flushed yet.
        #[allow(deprecated, non_upper_case_globals)]
        pub const FluskDiskCache: Self = Self::from_bits_retain(1 << 0);
        /// The resume data will contain the metadata from the torrent file as
        /// well. This is useful for clients that don't keep .torrent files
        /// around separately, or for torrents that were added via a magnet link.
        #[allow(deprecated, non_upper_case_globals)]
        pub const SaveInfoDict: Self = Self::from_bits_retain(1 << 1);
        /// This flag has the same behavior as the combination of:
        /// [`ResumeDataFlags::IfCountersChanged`] | [`ResumeDataFlags::IfDownloadProgress`] |
        /// [`ResumeDataFlags::IfConfigChanged`] | [`ResumeDataFlags::IfStateChanged`] |
        /// [`ResumeDataFlags::IfMetadataChanged`].
        #[allow(deprecated, non_upper_case_globals)]
        pub const OnlyIfModified: Self = Self::from_bits_retain(1 << 2);
        /// Save resume data if any counters has changed since the last time
        /// resume data was saved. This includes upload/download counters, active
        /// time counters and scrape data. A torrent that is not paused will have
        /// its active time counters incremented continuously.
        #[allow(deprecated, non_upper_case_globals)]
        pub const IfCountersChanged: Self = Self::from_bits_retain(1 << 3);
        /// Save the resume data if any blocks have been downloaded since the
        /// last time resume data was saved. This includes:
        /// * checking existing files on disk
        /// * downloading a block from a peer
        #[allow(deprecated, non_upper_case_globals)]
        pub const IfDownloadProgress: Self = Self::from_bits_retain(1 << 4);
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
        #[allow(deprecated, non_upper_case_globals)]
        pub const IfConfigChanged: Self = Self::from_bits_retain(1 << 5);
        /// Save the resume data if torrent state has changed since last time the
        /// resume data was saved. This includes:
        /// * upload mode
        /// * paused state
        /// * super-seeding
        /// * seed-mode
        #[allow(deprecated, non_upper_case_globals)]
        pub const IfStateChanged: Self = Self::from_bits_retain(1 << 6);
        /// Save the resume data if any *metadata* changed since the last time
        /// resume data was saved. This includes:
        /// * add/remove web seeds
        /// * add/remove trackers
        /// * receiving metadata for a magnet link
        #[allow(deprecated, non_upper_case_globals)]
        pub const IfMetadataChanged: Self = Self::from_bits_retain(1 << 7);
    }
    impl ::bitflags::Flags for ResumeDataFlags {
        const FLAGS: &'static [::bitflags::Flag<ResumeDataFlags>] = &[
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("FluskDiskCache", ResumeDataFlags::FluskDiskCache)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("SaveInfoDict", ResumeDataFlags::SaveInfoDict)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("OnlyIfModified", ResumeDataFlags::OnlyIfModified)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new(
                    "IfCountersChanged",
                    ResumeDataFlags::IfCountersChanged,
                )
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new(
                    "IfDownloadProgress",
                    ResumeDataFlags::IfDownloadProgress,
                )
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new(
                    "IfConfigChanged",
                    ResumeDataFlags::IfConfigChanged,
                )
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("IfStateChanged", ResumeDataFlags::IfStateChanged)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new(
                    "IfMetadataChanged",
                    ResumeDataFlags::IfMetadataChanged,
                )
            },
        ];
        type Bits = u8;
        fn bits(&self) -> u8 {
            ResumeDataFlags::bits(self)
        }
        fn from_bits_retain(bits: u8) -> ResumeDataFlags {
            ResumeDataFlags::from_bits_retain(bits)
        }
    }
    #[allow(
        dead_code,
        deprecated,
        unused_doc_comments,
        unused_attributes,
        unused_mut,
        unused_imports,
        non_upper_case_globals,
        clippy::assign_op_pattern,
        clippy::indexing_slicing,
        clippy::same_name_method,
        clippy::iter_without_into_iter,
    )]
    const _: () = {
        #[repr(transparent)]
        pub struct InternalBitFlags(u8);
        #[automatically_derived]
        impl ::core::clone::Clone for InternalBitFlags {
            #[inline]
            fn clone(&self) -> InternalBitFlags {
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for InternalBitFlags {
            #[inline]
            fn eq(&self, other: &InternalBitFlags) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for InternalBitFlags {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u8>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for InternalBitFlags {
            #[inline]
            fn partial_cmp(
                &self,
                other: &InternalBitFlags,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for InternalBitFlags {
            #[inline]
            fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for InternalBitFlags {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        impl ::bitflags::__private::PublicFlags for ResumeDataFlags {
            type Primitive = u8;
            type Internal = InternalBitFlags;
        }
        impl ::bitflags::__private::core::default::Default for InternalBitFlags {
            #[inline]
            fn default() -> Self {
                InternalBitFlags::empty()
            }
        }
        impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
            ) -> ::bitflags::__private::core::fmt::Result {
                if self.is_empty() {
                    f.write_fmt(format_args!("{0:#x}", <u8 as ::bitflags::Bits>::EMPTY))
                } else {
                    ::bitflags::__private::core::fmt::Display::fmt(self, f)
                }
            }
        }
        impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::parser::to_writer(&ResumeDataFlags(*self), f)
            }
        }
        impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
            type Err = ::bitflags::parser::ParseError;
            fn from_str(
                s: &str,
            ) -> ::bitflags::__private::core::result::Result<Self, Self::Err> {
                ::bitflags::parser::from_str::<ResumeDataFlags>(s).map(|flags| flags.0)
            }
        }
        impl ::bitflags::__private::core::convert::AsRef<u8> for InternalBitFlags {
            fn as_ref(&self) -> &u8 {
                &self.0
            }
        }
        impl ::bitflags::__private::core::convert::From<u8> for InternalBitFlags {
            fn from(bits: u8) -> Self {
                Self::from_bits_retain(bits)
            }
        }
        #[allow(dead_code, deprecated, unused_attributes)]
        impl InternalBitFlags {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                Self(<u8 as ::bitflags::Bits>::EMPTY)
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                let mut truncated = <u8 as ::bitflags::Bits>::EMPTY;
                let mut i = 0;
                {
                    {
                        let flag = <ResumeDataFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <ResumeDataFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <ResumeDataFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <ResumeDataFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <ResumeDataFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <ResumeDataFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <ResumeDataFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                {
                    {
                        let flag = <ResumeDataFlags as ::bitflags::Flags>::FLAGS[i]
                            .value()
                            .bits();
                        truncated = truncated | flag;
                        i += 1;
                    }
                };
                let _ = i;
                Self(truncated)
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> u8 {
                self.0
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(
                bits: u8,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let truncated = Self::from_bits_truncate(bits).0;
                if truncated == bits {
                    ::bitflags::__private::core::option::Option::Some(Self(bits))
                } else {
                    ::bitflags::__private::core::option::Option::None
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: u8) -> Self {
                Self(bits & Self::all().0)
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: u8) -> Self {
                Self(bits)
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(
                name: &str,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                {
                    if name == "FluskDiskCache" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(ResumeDataFlags::FluskDiskCache.bits()),
                        );
                    }
                };
                {
                    if name == "SaveInfoDict" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(ResumeDataFlags::SaveInfoDict.bits()),
                        );
                    }
                };
                {
                    if name == "OnlyIfModified" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(ResumeDataFlags::OnlyIfModified.bits()),
                        );
                    }
                };
                {
                    if name == "IfCountersChanged" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(ResumeDataFlags::IfCountersChanged.bits()),
                        );
                    }
                };
                {
                    if name == "IfDownloadProgress" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(ResumeDataFlags::IfDownloadProgress.bits()),
                        );
                    }
                };
                {
                    if name == "IfConfigChanged" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(ResumeDataFlags::IfConfigChanged.bits()),
                        );
                    }
                };
                {
                    if name == "IfStateChanged" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(ResumeDataFlags::IfStateChanged.bits()),
                        );
                    }
                };
                {
                    if name == "IfMetadataChanged" {
                        return ::bitflags::__private::core::option::Option::Some(
                            Self(ResumeDataFlags::IfMetadataChanged.bits()),
                        );
                    }
                };
                let _ = name;
                ::bitflags::__private::core::option::Option::None
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.0 == <u8 as ::bitflags::Bits>::EMPTY
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                Self::all().0 | self.0 == self.0
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                self.0 & other.0 != <u8 as ::bitflags::Bits>::EMPTY
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                self.0 & other.0 == other.0
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                *self = Self(self.0).union(other);
            }
            /// The intersection of a source flags value with the complement of a target flags
            /// value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                *self = Self(self.0).difference(other);
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                *self = Self(self.0).symmetric_difference(other);
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                if value {
                    self.insert(other);
                } else {
                    self.remove(other);
                }
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
            /// The intersection of a source flags value with the complement of a target flags
            /// value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                Self(self.0 & !other.0)
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                Self(self.0 ^ other.0)
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                Self::from_bits_truncate(!self.0)
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: InternalBitFlags) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for InternalBitFlags {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for InternalBitFlags {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<InternalBitFlags>
        for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags>
        for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<
                T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
            >(iterator: T) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl InternalBitFlags {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<ResumeDataFlags> {
                ::bitflags::iter::Iter::__private_const_new(
                    <ResumeDataFlags as ::bitflags::Flags>::FLAGS,
                    ResumeDataFlags::from_bits_retain(self.bits()),
                    ResumeDataFlags::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(
                &self,
            ) -> ::bitflags::iter::IterNames<ResumeDataFlags> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <ResumeDataFlags as ::bitflags::Flags>::FLAGS,
                    ResumeDataFlags::from_bits_retain(self.bits()),
                    ResumeDataFlags::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for InternalBitFlags {
            type Item = ResumeDataFlags;
            type IntoIter = ::bitflags::iter::Iter<ResumeDataFlags>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
        impl InternalBitFlags {
            /// Returns a mutable reference to the raw value of the flags currently stored.
            #[inline]
            pub fn bits_mut(&mut self) -> &mut u8 {
                &mut self.0
            }
        }
        #[allow(dead_code, deprecated, unused_attributes)]
        impl ResumeDataFlags {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                Self(InternalBitFlags::empty())
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                Self(InternalBitFlags::all())
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> u8 {
                self.0.bits()
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(
                bits: u8,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                match InternalBitFlags::from_bits(bits) {
                    ::bitflags::__private::core::option::Option::Some(bits) => {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    }
                    ::bitflags::__private::core::option::Option::None => {
                        ::bitflags::__private::core::option::Option::None
                    }
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: u8) -> Self {
                Self(InternalBitFlags::from_bits_truncate(bits))
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: u8) -> Self {
                Self(InternalBitFlags::from_bits_retain(bits))
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(
                name: &str,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                match InternalBitFlags::from_name(name) {
                    ::bitflags::__private::core::option::Option::Some(bits) => {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    }
                    ::bitflags::__private::core::option::Option::None => {
                        ::bitflags::__private::core::option::Option::None
                    }
                }
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                self.0.is_all()
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                self.0.intersects(other.0)
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                self.0.contains(other.0)
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                self.0.insert(other.0)
            }
            /// The intersection of a source flags value with the complement of a target flags
            /// value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                self.0.remove(other.0)
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                self.0.toggle(other.0)
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                self.0.set(other.0, value)
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                Self(self.0.intersection(other.0))
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                Self(self.0.union(other.0))
            }
            /// The intersection of a source flags value with the complement of a target flags
            /// value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                Self(self.0.difference(other.0))
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                Self(self.0.symmetric_difference(other.0))
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                Self(self.0.complement())
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for ResumeDataFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for ResumeDataFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for ResumeDataFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for ResumeDataFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for ResumeDataFlags {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: ResumeDataFlags) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for ResumeDataFlags {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for ResumeDataFlags {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for ResumeDataFlags {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for ResumeDataFlags {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for ResumeDataFlags {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for ResumeDataFlags {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for ResumeDataFlags {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for ResumeDataFlags {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<ResumeDataFlags>
        for ResumeDataFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<ResumeDataFlags>
        for ResumeDataFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<
                T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
            >(iterator: T) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl ResumeDataFlags {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<ResumeDataFlags> {
                ::bitflags::iter::Iter::__private_const_new(
                    <ResumeDataFlags as ::bitflags::Flags>::FLAGS,
                    ResumeDataFlags::from_bits_retain(self.bits()),
                    ResumeDataFlags::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(
                &self,
            ) -> ::bitflags::iter::IterNames<ResumeDataFlags> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <ResumeDataFlags as ::bitflags::Flags>::FLAGS,
                    ResumeDataFlags::from_bits_retain(self.bits()),
                    ResumeDataFlags::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for ResumeDataFlags {
            type Item = ResumeDataFlags;
            type IntoIter = ::bitflags::iter::Iter<ResumeDataFlags>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
    };
}
pub mod torrent_status {
    use crate::{alerts::TorrentState, torrent_handle::TorrentHandle};
    /// Holds a snapshot of the status of a torrent, as queried by [`TorrentHandle::status()`].
    pub struct TorrentStatus {
        pub handle: TorrentHandle,
        pub state: TorrentState,
        pub progress: f64,
    }
}
mod ffi {
    pub mod alerts {
        pub mod file_completed {
            #[deny(improper_ctypes, improper_ctypes_definitions)]
            #[allow(clippy::unknown_lints)]
            #[allow(
                non_camel_case_types,
                non_snake_case,
                clippy::extra_unused_type_parameters,
                clippy::items_after_statements,
                clippy::no_effect_underscore_binding,
                clippy::ptr_as_ptr,
                clippy::ref_as_ptr,
                clippy::unsafe_derive_deserialize,
                clippy::upper_case_acronyms,
                clippy::use_self,
            )]
            pub(crate) mod ffi {
                pub type file_completed_alert = crate::ffi::ffi::file_completed_alert;
                #[doc(hidden)]
                const _: () = {
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        file_completed_alert,
                        (
                            ::cxx::l,
                            ::cxx::i,
                            ::cxx::b,
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            (),
                            ::cxx::f,
                            ::cxx::i,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::__,
                            ::cxx::c,
                            ::cxx::o,
                            ::cxx::m,
                            ::cxx::p,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::t,
                            ::cxx::e,
                            ::cxx::d,
                            ::cxx::__,
                            ::cxx::a,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::t,
                        ),
                    >;
                };
            }
        }
        pub mod file_rename_failed {
            #[deny(improper_ctypes, improper_ctypes_definitions)]
            #[allow(clippy::unknown_lints)]
            #[allow(
                non_camel_case_types,
                non_snake_case,
                clippy::extra_unused_type_parameters,
                clippy::items_after_statements,
                clippy::no_effect_underscore_binding,
                clippy::ptr_as_ptr,
                clippy::ref_as_ptr,
                clippy::unsafe_derive_deserialize,
                clippy::upper_case_acronyms,
                clippy::use_self,
            )]
            pub(crate) mod ffi {
                pub type file_rename_failed_alert = crate::ffi::ffi::file_rename_failed_alert;
                pub type Error = crate::ffi::error::ffi::Error;
                pub unsafe fn file_rename_failed_alert_get_error(
                    alert: *mut file_rename_failed_alert,
                ) -> Error {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$file_rename_failed_alert_get_error"]
                        fn __file_rename_failed_alert_get_error(
                            alert: *mut ::cxx::core::ffi::c_void,
                            __return: *mut Error,
                        );
                    }
                    unsafe {
                        let mut __return = ::cxx::core::mem::MaybeUninit::<
                            Error,
                        >::uninit();
                        __file_rename_failed_alert_get_error(
                            alert.cast(),
                            __return.as_mut_ptr(),
                        );
                        __return.assume_init()
                    }
                }
                #[doc(hidden)]
                const _: () = {
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        file_rename_failed_alert,
                        (
                            ::cxx::l,
                            ::cxx::i,
                            ::cxx::b,
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            (),
                            ::cxx::f,
                            ::cxx::i,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::__,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::a,
                            ::cxx::m,
                            ::cxx::e,
                            ::cxx::__,
                            ::cxx::f,
                            ::cxx::a,
                            ::cxx::i,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::d,
                            ::cxx::__,
                            ::cxx::a,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::t,
                        ),
                    >;
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        Error,
                        (
                            ::cxx::l,
                            ::cxx::t,
                            ::cxx::r,
                            ::cxx::s,
                            (),
                            ::cxx::E,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::o,
                            ::cxx::r,
                        ),
                    >;
                    const _: fn() = ::cxx::private::verify_extern_kind::<
                        Error,
                        ::cxx::kind::Trivial,
                    >;
                };
            }
        }
        pub mod file_renamed {
            #[deny(improper_ctypes, improper_ctypes_definitions)]
            #[allow(clippy::unknown_lints)]
            #[allow(
                non_camel_case_types,
                non_snake_case,
                clippy::extra_unused_type_parameters,
                clippy::items_after_statements,
                clippy::no_effect_underscore_binding,
                clippy::ptr_as_ptr,
                clippy::ref_as_ptr,
                clippy::unsafe_derive_deserialize,
                clippy::upper_case_acronyms,
                clippy::use_self,
            )]
            pub(crate) mod ffi {
                pub type file_renamed_alert = crate::ffi::ffi::file_renamed_alert;
                pub unsafe fn file_renamed_alert_get_old_name<'a>(
                    alert: *mut file_renamed_alert,
                ) -> &'a str {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$file_renamed_alert_get_old_name"]
                        fn __file_renamed_alert_get_old_name<'a>(
                            alert: *mut ::cxx::core::ffi::c_void,
                        ) -> ::cxx::private::RustStr;
                    }
                    unsafe { __file_renamed_alert_get_old_name(alert.cast()).as_str() }
                }
                pub unsafe fn file_renamed_alert_get_new_name<'a>(
                    alert: *mut file_renamed_alert,
                ) -> &'a str {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$file_renamed_alert_get_new_name"]
                        fn __file_renamed_alert_get_new_name<'a>(
                            alert: *mut ::cxx::core::ffi::c_void,
                        ) -> ::cxx::private::RustStr;
                    }
                    unsafe { __file_renamed_alert_get_new_name(alert.cast()).as_str() }
                }
                #[doc(hidden)]
                const _: () = {
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        file_renamed_alert,
                        (
                            ::cxx::l,
                            ::cxx::i,
                            ::cxx::b,
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            (),
                            ::cxx::f,
                            ::cxx::i,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::__,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::a,
                            ::cxx::m,
                            ::cxx::e,
                            ::cxx::d,
                            ::cxx::__,
                            ::cxx::a,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::t,
                        ),
                    >;
                };
            }
        }
        pub mod performance {
            #[deny(improper_ctypes, improper_ctypes_definitions)]
            #[allow(clippy::unknown_lints)]
            #[allow(
                non_camel_case_types,
                non_snake_case,
                clippy::extra_unused_type_parameters,
                clippy::items_after_statements,
                clippy::no_effect_underscore_binding,
                clippy::ptr_as_ptr,
                clippy::ref_as_ptr,
                clippy::unsafe_derive_deserialize,
                clippy::upper_case_acronyms,
                clippy::use_self,
            )]
            pub(crate) mod ffi {
                pub type performance_alert = crate::ffi::ffi::performance_alert;
                pub unsafe fn performance_alert_get_warning_code(
                    alert: *mut performance_alert,
                ) -> u8 {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$performance_alert_get_warning_code"]
                        fn __performance_alert_get_warning_code(
                            alert: *mut ::cxx::core::ffi::c_void,
                        ) -> u8;
                    }
                    unsafe { __performance_alert_get_warning_code(alert.cast()) }
                }
                #[doc(hidden)]
                const _: () = {
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        performance_alert,
                        (
                            ::cxx::l,
                            ::cxx::i,
                            ::cxx::b,
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            (),
                            ::cxx::p,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::f,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::m,
                            ::cxx::a,
                            ::cxx::n,
                            ::cxx::c,
                            ::cxx::e,
                            ::cxx::__,
                            ::cxx::a,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::t,
                        ),
                    >;
                };
            }
        }
        pub mod read_piece {
            #[deny(improper_ctypes, improper_ctypes_definitions)]
            #[allow(clippy::unknown_lints)]
            #[allow(
                non_camel_case_types,
                non_snake_case,
                clippy::extra_unused_type_parameters,
                clippy::items_after_statements,
                clippy::no_effect_underscore_binding,
                clippy::ptr_as_ptr,
                clippy::ref_as_ptr,
                clippy::unsafe_derive_deserialize,
                clippy::upper_case_acronyms,
                clippy::use_self,
            )]
            pub(crate) mod ffi {
                pub type read_piece_alert = crate::ffi::ffi::read_piece_alert;
                pub type Error = crate::ffi::error::ffi::Error;
                pub unsafe fn read_piece_alert_get_size(
                    alert: *mut read_piece_alert,
                ) -> i32 {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$read_piece_alert_get_size"]
                        fn __read_piece_alert_get_size(
                            alert: *mut ::cxx::core::ffi::c_void,
                        ) -> i32;
                    }
                    unsafe { __read_piece_alert_get_size(alert.cast()) }
                }
                pub unsafe fn read_piece_alert_get_error(
                    alert: *mut read_piece_alert,
                ) -> Error {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$read_piece_alert_get_error"]
                        fn __read_piece_alert_get_error(
                            alert: *mut ::cxx::core::ffi::c_void,
                            __return: *mut Error,
                        );
                    }
                    unsafe {
                        let mut __return = ::cxx::core::mem::MaybeUninit::<
                            Error,
                        >::uninit();
                        __read_piece_alert_get_error(
                            alert.cast(),
                            __return.as_mut_ptr(),
                        );
                        __return.assume_init()
                    }
                }
                #[doc(hidden)]
                const _: () = {
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        read_piece_alert,
                        (
                            ::cxx::l,
                            ::cxx::i,
                            ::cxx::b,
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            (),
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::a,
                            ::cxx::d,
                            ::cxx::__,
                            ::cxx::p,
                            ::cxx::i,
                            ::cxx::e,
                            ::cxx::c,
                            ::cxx::e,
                            ::cxx::__,
                            ::cxx::a,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::t,
                        ),
                    >;
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        Error,
                        (
                            ::cxx::l,
                            ::cxx::t,
                            ::cxx::r,
                            ::cxx::s,
                            (),
                            ::cxx::E,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::o,
                            ::cxx::r,
                        ),
                    >;
                    const _: fn() = ::cxx::private::verify_extern_kind::<
                        Error,
                        ::cxx::kind::Trivial,
                    >;
                };
            }
        }
        pub mod state_changed {
            #[deny(improper_ctypes, improper_ctypes_definitions)]
            #[allow(clippy::unknown_lints)]
            #[allow(
                non_camel_case_types,
                non_snake_case,
                clippy::extra_unused_type_parameters,
                clippy::items_after_statements,
                clippy::no_effect_underscore_binding,
                clippy::ptr_as_ptr,
                clippy::ref_as_ptr,
                clippy::unsafe_derive_deserialize,
                clippy::upper_case_acronyms,
                clippy::use_self,
            )]
            pub(crate) mod ffi {
                pub type state_changed_alert = crate::ffi::ffi::state_changed_alert;
                pub unsafe fn state_changed_alert_get_state(
                    alert: *mut state_changed_alert,
                ) -> u8 {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$state_changed_alert_get_state"]
                        fn __state_changed_alert_get_state(
                            alert: *mut ::cxx::core::ffi::c_void,
                        ) -> u8;
                    }
                    unsafe { __state_changed_alert_get_state(alert.cast()) }
                }
                pub unsafe fn state_changed_alert_get_prev_state(
                    alert: *mut state_changed_alert,
                ) -> u8 {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$state_changed_alert_get_prev_state"]
                        fn __state_changed_alert_get_prev_state(
                            alert: *mut ::cxx::core::ffi::c_void,
                        ) -> u8;
                    }
                    unsafe { __state_changed_alert_get_prev_state(alert.cast()) }
                }
                #[doc(hidden)]
                const _: () = {
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        state_changed_alert,
                        (
                            ::cxx::l,
                            ::cxx::i,
                            ::cxx::b,
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            (),
                            ::cxx::s,
                            ::cxx::t,
                            ::cxx::a,
                            ::cxx::t,
                            ::cxx::e,
                            ::cxx::__,
                            ::cxx::c,
                            ::cxx::h,
                            ::cxx::a,
                            ::cxx::n,
                            ::cxx::g,
                            ::cxx::e,
                            ::cxx::d,
                            ::cxx::__,
                            ::cxx::a,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::t,
                        ),
                    >;
                };
            }
            pub(crate) mod ffi2 {
                pub type state_changed_alert = crate::ffi::ffi::state_changed_alert;
                #[inline]
                pub unsafe fn state_changed_alert_get_state(
                    alert: *mut state_changed_alert,
                ) -> u8 {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$state_changed_alert_get_state"]
                        fn __state_changed_alert_get_state(
                            alert: *mut ::cxx::core::ffi::c_void,
                        ) -> u8;
                    }
                    unsafe { __state_changed_alert_get_state(alert.cast()) }
                }
            }
        }
        pub mod torrent_alert {
            #[deny(improper_ctypes, improper_ctypes_definitions)]
            #[allow(clippy::unknown_lints)]
            #[allow(
                non_camel_case_types,
                non_snake_case,
                clippy::extra_unused_type_parameters,
                clippy::items_after_statements,
                clippy::no_effect_underscore_binding,
                clippy::ptr_as_ptr,
                clippy::ref_as_ptr,
                clippy::unsafe_derive_deserialize,
                clippy::upper_case_acronyms,
                clippy::use_self,
            )]
            pub(crate) mod ffi {
                pub type torrent_alert = crate::ffi::ffi::torrent_alert;
                pub type torrent_handle = crate::ffi::torrent_handle::ffi::torrent_handle;
                pub unsafe fn torrent_alert_handle(
                    alert: *mut torrent_alert,
                ) -> ::cxx::UniquePtr<torrent_handle> {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$torrent_alert_handle"]
                        fn __torrent_alert_handle(
                            alert: *mut ::cxx::core::ffi::c_void,
                        ) -> *mut ::cxx::core::ffi::c_void;
                    }
                    unsafe {
                        ::cxx::UniquePtr::from_raw(
                            __torrent_alert_handle(alert.cast()).cast(),
                        )
                    }
                }
                pub unsafe fn torrent_alert_message(
                    alert: *const torrent_alert,
                ) -> ::cxx::alloc::string::String {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$torrent_alert_message"]
                        fn __torrent_alert_message(
                            alert: *const ::cxx::core::ffi::c_void,
                            __return: *mut ::cxx::private::RustString,
                        );
                    }
                    unsafe {
                        let mut __return = ::cxx::core::mem::MaybeUninit::<
                            ::cxx::private::RustString,
                        >::uninit();
                        __torrent_alert_message(alert.cast(), __return.as_mut_ptr());
                        __return.assume_init().into_string()
                    }
                }
                pub unsafe fn torrent_alert_torrent_name<'a>(
                    alert: *const torrent_alert,
                ) -> &'a str {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$torrent_alert_torrent_name"]
                        fn __torrent_alert_torrent_name<'a>(
                            alert: *const ::cxx::core::ffi::c_void,
                        ) -> ::cxx::private::RustStr;
                    }
                    unsafe { __torrent_alert_torrent_name(alert.cast()).as_str() }
                }
                #[doc(hidden)]
                const _: () = {
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        torrent_alert,
                        (
                            ::cxx::l,
                            ::cxx::i,
                            ::cxx::b,
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            (),
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            ::cxx::__,
                            ::cxx::a,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::t,
                        ),
                    >;
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        torrent_handle,
                        (
                            ::cxx::l,
                            ::cxx::i,
                            ::cxx::b,
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            (),
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            ::cxx::__,
                            ::cxx::h,
                            ::cxx::a,
                            ::cxx::n,
                            ::cxx::d,
                            ::cxx::l,
                            ::cxx::e,
                        ),
                    >;
                };
            }
        }
        pub mod torrent_removed {
            #[deny(improper_ctypes, improper_ctypes_definitions)]
            #[allow(clippy::unknown_lints)]
            #[allow(
                non_camel_case_types,
                non_snake_case,
                clippy::extra_unused_type_parameters,
                clippy::items_after_statements,
                clippy::no_effect_underscore_binding,
                clippy::ptr_as_ptr,
                clippy::ref_as_ptr,
                clippy::unsafe_derive_deserialize,
                clippy::upper_case_acronyms,
                clippy::use_self,
            )]
            pub(crate) mod ffi {
                pub type torrent_removed_alert = crate::ffi::ffi::torrent_removed_alert;
                pub type InfoHashCpp = crate::ffi::ffi::InfoHashCpp;
                pub unsafe fn torrent_removed_alert_get_info_hashes(
                    a: *mut torrent_removed_alert,
                ) -> InfoHashCpp {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$torrent_removed_alert_get_info_hashes"]
                        fn __torrent_removed_alert_get_info_hashes(
                            a: *mut ::cxx::core::ffi::c_void,
                            __return: *mut InfoHashCpp,
                        );
                    }
                    unsafe {
                        let mut __return = ::cxx::core::mem::MaybeUninit::<
                            InfoHashCpp,
                        >::uninit();
                        __torrent_removed_alert_get_info_hashes(
                            a.cast(),
                            __return.as_mut_ptr(),
                        );
                        __return.assume_init()
                    }
                }
                #[doc(hidden)]
                const _: () = {
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        torrent_removed_alert,
                        (
                            ::cxx::l,
                            ::cxx::i,
                            ::cxx::b,
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            (),
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            ::cxx::__,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::m,
                            ::cxx::o,
                            ::cxx::v,
                            ::cxx::e,
                            ::cxx::d,
                            ::cxx::__,
                            ::cxx::a,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::t,
                        ),
                    >;
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        InfoHashCpp,
                        (
                            ::cxx::l,
                            ::cxx::t,
                            ::cxx::r,
                            ::cxx::s,
                            (),
                            ::cxx::I,
                            ::cxx::n,
                            ::cxx::f,
                            ::cxx::o,
                            ::cxx::H,
                            ::cxx::a,
                            ::cxx::s,
                            ::cxx::h,
                            ::cxx::C,
                            ::cxx::p,
                            ::cxx::p,
                        ),
                    >;
                    const _: fn() = ::cxx::private::verify_extern_kind::<
                        InfoHashCpp,
                        ::cxx::kind::Trivial,
                    >;
                };
            }
        }
        pub mod tracker_alert {
            #[deny(improper_ctypes, improper_ctypes_definitions)]
            #[allow(clippy::unknown_lints)]
            #[allow(
                non_camel_case_types,
                non_snake_case,
                clippy::extra_unused_type_parameters,
                clippy::items_after_statements,
                clippy::no_effect_underscore_binding,
                clippy::ptr_as_ptr,
                clippy::ref_as_ptr,
                clippy::unsafe_derive_deserialize,
                clippy::upper_case_acronyms,
                clippy::use_self,
            )]
            pub(crate) mod ffi {
                pub type tracker_alert = crate::ffi::ffi::tracker_alert;
                pub unsafe fn tracker_alert_get_tracker_url<'a>(
                    alert: *mut tracker_alert,
                ) -> &'a str {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$tracker_alert_get_tracker_url"]
                        fn __tracker_alert_get_tracker_url<'a>(
                            alert: *mut ::cxx::core::ffi::c_void,
                        ) -> ::cxx::private::RustStr;
                    }
                    unsafe { __tracker_alert_get_tracker_url(alert.cast()).as_str() }
                }
                #[doc(hidden)]
                const _: () = {
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        tracker_alert,
                        (
                            ::cxx::l,
                            ::cxx::i,
                            ::cxx::b,
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            (),
                            ::cxx::t,
                            ::cxx::r,
                            ::cxx::a,
                            ::cxx::c,
                            ::cxx::k,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::__,
                            ::cxx::a,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::t,
                        ),
                    >;
                };
            }
        }
        pub mod tracker_error {
            #[deny(improper_ctypes, improper_ctypes_definitions)]
            #[allow(clippy::unknown_lints)]
            #[allow(
                non_camel_case_types,
                non_snake_case,
                clippy::extra_unused_type_parameters,
                clippy::items_after_statements,
                clippy::no_effect_underscore_binding,
                clippy::ptr_as_ptr,
                clippy::ref_as_ptr,
                clippy::unsafe_derive_deserialize,
                clippy::upper_case_acronyms,
                clippy::use_self,
            )]
            pub(crate) mod ffi {
                pub type tracker_error_alert = crate::ffi::ffi::tracker_error_alert;
                pub type Error = crate::ffi::error::ffi::Error;
                pub unsafe fn tracker_error_alert_get_failure_reason<'a>(
                    a: *mut tracker_error_alert,
                ) -> &'a str {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$tracker_error_alert_get_failure_reason"]
                        fn __tracker_error_alert_get_failure_reason<'a>(
                            a: *mut ::cxx::core::ffi::c_void,
                        ) -> ::cxx::private::RustStr;
                    }
                    unsafe {
                        __tracker_error_alert_get_failure_reason(a.cast()).as_str()
                    }
                }
                pub unsafe fn tracker_error_alert_get_times_in_row(
                    a: *mut tracker_error_alert,
                ) -> i32 {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$tracker_error_alert_get_times_in_row"]
                        fn __tracker_error_alert_get_times_in_row(
                            a: *mut ::cxx::core::ffi::c_void,
                        ) -> i32;
                    }
                    unsafe { __tracker_error_alert_get_times_in_row(a.cast()) }
                }
                pub unsafe fn tracker_error_alert_get_error(
                    a: *mut tracker_error_alert,
                ) -> Error {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$tracker_error_alert_get_error"]
                        fn __tracker_error_alert_get_error(
                            a: *mut ::cxx::core::ffi::c_void,
                            __return: *mut Error,
                        );
                    }
                    unsafe {
                        let mut __return = ::cxx::core::mem::MaybeUninit::<
                            Error,
                        >::uninit();
                        __tracker_error_alert_get_error(a.cast(), __return.as_mut_ptr());
                        __return.assume_init()
                    }
                }
                pub unsafe fn tracker_error_alert_get_op(
                    a: *mut tracker_error_alert,
                ) -> u8 {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$tracker_error_alert_get_op"]
                        fn __tracker_error_alert_get_op(
                            a: *mut ::cxx::core::ffi::c_void,
                        ) -> u8;
                    }
                    unsafe { __tracker_error_alert_get_op(a.cast()) }
                }
                pub unsafe fn tracker_error_alert_get_version(
                    a: *mut tracker_error_alert,
                ) -> u8 {
                    unsafe extern "C" {
                        #[link_name = "ltrs$cxxbridge1$tracker_error_alert_get_version"]
                        fn __tracker_error_alert_get_version(
                            a: *mut ::cxx::core::ffi::c_void,
                        ) -> u8;
                    }
                    unsafe { __tracker_error_alert_get_version(a.cast()) }
                }
                #[doc(hidden)]
                const _: () = {
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        tracker_error_alert,
                        (
                            ::cxx::l,
                            ::cxx::i,
                            ::cxx::b,
                            ::cxx::t,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::e,
                            ::cxx::n,
                            ::cxx::t,
                            (),
                            ::cxx::t,
                            ::cxx::r,
                            ::cxx::a,
                            ::cxx::c,
                            ::cxx::k,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::__,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::o,
                            ::cxx::r,
                            ::cxx::__,
                            ::cxx::a,
                            ::cxx::l,
                            ::cxx::e,
                            ::cxx::r,
                            ::cxx::t,
                        ),
                    >;
                    const _: fn() = ::cxx::private::verify_extern_type::<
                        Error,
                        (
                            ::cxx::l,
                            ::cxx::t,
                            ::cxx::r,
                            ::cxx::s,
                            (),
                            ::cxx::E,
                            ::cxx::r,
                            ::cxx::r,
                            ::cxx::o,
                            ::cxx::r,
                        ),
                    >;
                    const _: fn() = ::cxx::private::verify_extern_kind::<
                        Error,
                        ::cxx::kind::Trivial,
                    >;
                };
            }
        }
    }
    pub mod error {
        #[deny(improper_ctypes, improper_ctypes_definitions)]
        #[allow(clippy::unknown_lints)]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::extra_unused_type_parameters,
            clippy::items_after_statements,
            clippy::no_effect_underscore_binding,
            clippy::ptr_as_ptr,
            clippy::ref_as_ptr,
            clippy::unsafe_derive_deserialize,
            clippy::upper_case_acronyms,
            clippy::use_self,
        )]
        pub(crate) mod ffi {
            #[repr(transparent)]
            pub struct ErrorCategory {
                #[allow(missing_docs)]
                pub repr: u8,
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ErrorCategory {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<u8>;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ErrorCategory {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ErrorCategory {
                #[inline]
                fn eq(&self, other: &ErrorCategory) -> bool {
                    self.repr == other.repr
                }
            }
            #[allow(non_upper_case_globals)]
            impl ErrorCategory {
                #[allow(dead_code)]
                pub const LibtorrentError: Self = ErrorCategory { repr: 0 };
                #[allow(dead_code)]
                pub const HttpError: Self = ErrorCategory { repr: 1 };
                #[allow(dead_code)]
                pub const GzipError: Self = ErrorCategory { repr: 2 };
                #[allow(dead_code)]
                pub const I2pError: Self = ErrorCategory { repr: 3 };
                #[allow(dead_code)]
                pub const PcpError: Self = ErrorCategory { repr: 4 };
                #[allow(dead_code)]
                pub const BdecodeError: Self = ErrorCategory { repr: 5 };
                #[allow(dead_code)]
                pub const SocksError: Self = ErrorCategory { repr: 6 };
                #[allow(dead_code)]
                pub const UpnpError: Self = ErrorCategory { repr: 7 };
                #[allow(dead_code)]
                pub const Unknown: Self = ErrorCategory { repr: 8 };
            }
            #[automatically_derived]
            unsafe impl ::cxx::ExternType for ErrorCategory {
                #[allow(unused_attributes)]
                #[doc(hidden)]
                type Id = (
                    ::cxx::l,
                    ::cxx::t,
                    ::cxx::r,
                    ::cxx::s,
                    (),
                    ::cxx::E,
                    ::cxx::r,
                    ::cxx::r,
                    ::cxx::o,
                    ::cxx::r,
                    ::cxx::C,
                    ::cxx::a,
                    ::cxx::t,
                    ::cxx::e,
                    ::cxx::g,
                    ::cxx::o,
                    ::cxx::r,
                    ::cxx::y,
                );
                type Kind = ::cxx::kind::Trivial;
            }
            #[automatically_derived]
            impl ::cxx::core::marker::Copy for ErrorCategory {}
            #[automatically_derived]
            #[allow(clippy::expl_impl_clone_on_copy)]
            impl ::cxx::core::clone::Clone for ErrorCategory {
                fn clone(&self) -> Self {
                    *self
                }
            }
            #[repr(C)]
            pub struct Error {
                pub category: ErrorCategory,
                pub code: i32,
            }
            #[automatically_derived]
            unsafe impl ::cxx::ExternType for Error {
                #[allow(unused_attributes)]
                #[doc(hidden)]
                type Id = (
                    ::cxx::l,
                    ::cxx::t,
                    ::cxx::r,
                    ::cxx::s,
                    (),
                    ::cxx::E,
                    ::cxx::r,
                    ::cxx::r,
                    ::cxx::o,
                    ::cxx::r,
                );
                type Kind = ::cxx::kind::Trivial;
            }
            #[doc(hidden)]
            const _: () = {
                mod forbid {
                    pub trait Drop {}
                    #[automatically_derived]
                    #[allow(drop_bounds)]
                    impl<
                        T: ?::cxx::core::marker::Sized + ::cxx::core::ops::Drop,
                    > self::Drop for T {}
                    #[automatically_derived]
                    impl self::Drop for super::Error {}
                }
            };
        }
    }
    pub mod torrent_handle {
        #[deny(improper_ctypes, improper_ctypes_definitions)]
        #[allow(clippy::unknown_lints)]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::extra_unused_type_parameters,
            clippy::items_after_statements,
            clippy::no_effect_underscore_binding,
            clippy::ptr_as_ptr,
            clippy::ref_as_ptr,
            clippy::unsafe_derive_deserialize,
            clippy::upper_case_acronyms,
            clippy::use_self,
        )]
        pub(crate) mod ffi {
            #[repr(C)]
            pub struct torrent_handle {
                _private: ::cxx::private::Opaque,
            }
            #[automatically_derived]
            unsafe impl ::cxx::ExternType for torrent_handle {
                #[allow(unused_attributes)]
                #[doc(hidden)]
                type Id = (
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    ::cxx::t,
                    ::cxx::o,
                    ::cxx::r,
                    ::cxx::r,
                    ::cxx::e,
                    ::cxx::n,
                    ::cxx::t,
                    (),
                    ::cxx::t,
                    ::cxx::o,
                    ::cxx::r,
                    ::cxx::r,
                    ::cxx::e,
                    ::cxx::n,
                    ::cxx::t,
                    ::cxx::__,
                    ::cxx::h,
                    ::cxx::a,
                    ::cxx::n,
                    ::cxx::d,
                    ::cxx::l,
                    ::cxx::e,
                );
                type Kind = ::cxx::kind::Opaque;
            }
            pub type torrent_status = crate::ffi::ffi::torrent_status;
            pub type InfoHashCpp = crate::ffi::ffi::InfoHashCpp;
            pub fn torrent_handle_in_session(handle: &torrent_handle) -> bool {
                unsafe extern "C" {
                    #[link_name = "ltrs$cxxbridge1$torrent_handle_in_session"]
                    fn __torrent_handle_in_session(handle: &torrent_handle) -> bool;
                }
                unsafe { __torrent_handle_in_session(handle) }
            }
            pub fn torrent_handle_read_piece(handle: &torrent_handle, piece: i32) {
                unsafe extern "C" {
                    #[link_name = "ltrs$cxxbridge1$torrent_handle_read_piece"]
                    fn __torrent_handle_read_piece(handle: &torrent_handle, piece: i32);
                }
                unsafe {
                    __torrent_handle_read_piece(handle, piece);
                }
            }
            pub fn torrent_handle_status(
                handle: &torrent_handle,
            ) -> ::cxx::UniquePtr<torrent_status> {
                unsafe extern "C" {
                    #[link_name = "ltrs$cxxbridge1$torrent_handle_status"]
                    fn __torrent_handle_status(
                        handle: &torrent_handle,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe {
                    ::cxx::UniquePtr::from_raw(__torrent_handle_status(handle).cast())
                }
            }
            pub fn torrent_handle_save_resume_data(handle: &torrent_handle, flags: u8) {
                unsafe extern "C" {
                    #[link_name = "ltrs$cxxbridge1$torrent_handle_save_resume_data"]
                    fn __torrent_handle_save_resume_data(
                        handle: &torrent_handle,
                        flags: u8,
                    );
                }
                unsafe {
                    __torrent_handle_save_resume_data(handle, flags);
                }
            }
            pub fn torrent_handle_info_hashes(handle: &torrent_handle) -> InfoHashCpp {
                unsafe extern "C" {
                    #[link_name = "ltrs$cxxbridge1$torrent_handle_info_hashes"]
                    fn __torrent_handle_info_hashes(
                        handle: &torrent_handle,
                        __return: *mut InfoHashCpp,
                    );
                }
                unsafe {
                    let mut __return = ::cxx::core::mem::MaybeUninit::<
                        InfoHashCpp,
                    >::uninit();
                    __torrent_handle_info_hashes(handle, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
            #[automatically_derived]
            unsafe impl ::cxx::memory::UniquePtrTarget for torrent_handle {
                fn __typename(
                    f: &mut ::cxx::core::fmt::Formatter<'_>,
                ) -> ::cxx::core::fmt::Result {
                    f.write_str("torrent_handle")
                }
                fn __null() -> ::cxx::core::mem::MaybeUninit<
                    *mut ::cxx::core::ffi::c_void,
                > {
                    unsafe extern "C" {
                        #[link_name = "cxxbridge1$unique_ptr$libtorrent$torrent_handle$null"]
                        fn __null(
                            this: *mut ::cxx::core::mem::MaybeUninit<
                                *mut ::cxx::core::ffi::c_void,
                            >,
                        );
                    }
                    let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                    unsafe {
                        __null(&raw mut repr);
                    }
                    repr
                }
                unsafe fn __raw(
                    raw: *mut Self,
                ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                    unsafe extern "C" {
                        #[link_name = "cxxbridge1$unique_ptr$libtorrent$torrent_handle$raw"]
                        fn __raw(
                            this: *mut ::cxx::core::mem::MaybeUninit<
                                *mut ::cxx::core::ffi::c_void,
                            >,
                            raw: *mut ::cxx::core::ffi::c_void,
                        );
                    }
                    let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                    unsafe {
                        __raw(&raw mut repr, raw.cast());
                    }
                    repr
                }
                unsafe fn __get(
                    repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                ) -> *const Self {
                    unsafe extern "C" {
                        #[link_name = "cxxbridge1$unique_ptr$libtorrent$torrent_handle$get"]
                        fn __get(
                            this: *const ::cxx::core::mem::MaybeUninit<
                                *mut ::cxx::core::ffi::c_void,
                            >,
                        ) -> *const ::cxx::core::ffi::c_void;
                    }
                    unsafe { __get(&raw const repr).cast() }
                }
                unsafe fn __release(
                    mut repr: ::cxx::core::mem::MaybeUninit<
                        *mut ::cxx::core::ffi::c_void,
                    >,
                ) -> *mut Self {
                    unsafe extern "C" {
                        #[link_name = "cxxbridge1$unique_ptr$libtorrent$torrent_handle$release"]
                        fn __release(
                            this: *mut ::cxx::core::mem::MaybeUninit<
                                *mut ::cxx::core::ffi::c_void,
                            >,
                        ) -> *mut ::cxx::core::ffi::c_void;
                    }
                    unsafe { __release(&raw mut repr).cast() }
                }
                unsafe fn __drop(
                    mut repr: ::cxx::core::mem::MaybeUninit<
                        *mut ::cxx::core::ffi::c_void,
                    >,
                ) {
                    unsafe extern "C" {
                        #[link_name = "cxxbridge1$unique_ptr$libtorrent$torrent_handle$drop"]
                        fn __drop(
                            this: *mut ::cxx::core::mem::MaybeUninit<
                                *mut ::cxx::core::ffi::c_void,
                            >,
                        );
                    }
                    unsafe {
                        __drop(&raw mut repr);
                    }
                }
            }
            #[doc(hidden)]
            const _: () = {
                let _: fn() = {
                    trait __AmbiguousIfImpl<A> {
                        fn infer() {}
                    }
                    #[automatically_derived]
                    impl<T> __AmbiguousIfImpl<()> for T
                    where
                        T: ?::cxx::core::marker::Sized,
                    {}
                    #[allow(dead_code)]
                    struct __Invalid;
                    #[automatically_derived]
                    impl<T> __AmbiguousIfImpl<__Invalid> for T
                    where
                        T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                    {}
                    <torrent_handle as __AmbiguousIfImpl<_>>::infer
                };
                const _: fn() = ::cxx::private::verify_extern_type::<
                    torrent_status,
                    (
                        ::cxx::l,
                        ::cxx::i,
                        ::cxx::b,
                        ::cxx::t,
                        ::cxx::o,
                        ::cxx::r,
                        ::cxx::r,
                        ::cxx::e,
                        ::cxx::n,
                        ::cxx::t,
                        (),
                        ::cxx::t,
                        ::cxx::o,
                        ::cxx::r,
                        ::cxx::r,
                        ::cxx::e,
                        ::cxx::n,
                        ::cxx::t,
                        ::cxx::__,
                        ::cxx::s,
                        ::cxx::t,
                        ::cxx::a,
                        ::cxx::t,
                        ::cxx::u,
                        ::cxx::s,
                    ),
                >;
                const _: fn() = ::cxx::private::verify_extern_type::<
                    InfoHashCpp,
                    (
                        ::cxx::l,
                        ::cxx::t,
                        ::cxx::r,
                        ::cxx::s,
                        (),
                        ::cxx::I,
                        ::cxx::n,
                        ::cxx::f,
                        ::cxx::o,
                        ::cxx::H,
                        ::cxx::a,
                        ::cxx::s,
                        ::cxx::h,
                        ::cxx::C,
                        ::cxx::p,
                        ::cxx::p,
                    ),
                >;
                const _: fn() = ::cxx::private::verify_extern_kind::<
                    InfoHashCpp,
                    ::cxx::kind::Trivial,
                >;
            };
        }
    }
    #[deny(improper_ctypes, improper_ctypes_definitions)]
    #[allow(clippy::unknown_lints)]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::extra_unused_type_parameters,
        clippy::items_after_statements,
        clippy::no_effect_underscore_binding,
        clippy::ptr_as_ptr,
        clippy::ref_as_ptr,
        clippy::unsafe_derive_deserialize,
        clippy::upper_case_acronyms,
        clippy::use_self,
    )]
    pub(crate) mod ffi {
        #[repr(C)]
        pub struct InfoHashCpp {
            pub version: u8,
            pub inner: [u8; 32],
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for InfoHashCpp {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::t,
                ::cxx::r,
                ::cxx::s,
                (),
                ::cxx::I,
                ::cxx::n,
                ::cxx::f,
                ::cxx::o,
                ::cxx::H,
                ::cxx::a,
                ::cxx::s,
                ::cxx::h,
                ::cxx::C,
                ::cxx::p,
                ::cxx::p,
            );
            type Kind = ::cxx::kind::Trivial;
        }
        #[repr(transparent)]
        pub struct AlertType {
            #[allow(missing_docs)]
            pub repr: u8,
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for AlertType {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u8>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for AlertType {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for AlertType {
            #[inline]
            fn eq(&self, other: &AlertType) -> bool {
                self.repr == other.repr
            }
        }
        #[allow(non_upper_case_globals)]
        impl AlertType {
            #[allow(dead_code)]
            pub const TorrentRemoved: Self = AlertType { repr: 4 };
            #[allow(dead_code)]
            pub const ReadPiece: Self = AlertType { repr: 5 };
            #[allow(dead_code)]
            pub const FileCompleted: Self = AlertType { repr: 6 };
            #[allow(dead_code)]
            pub const FileRenamed: Self = AlertType { repr: 7 };
            #[allow(dead_code)]
            pub const FileRenameFailed: Self = AlertType { repr: 8 };
            #[allow(dead_code)]
            pub const Performance: Self = AlertType { repr: 9 };
            #[allow(dead_code)]
            pub const StateChanged: Self = AlertType { repr: 10 };
            #[allow(dead_code)]
            pub const TrackerError: Self = AlertType { repr: 11 };
            #[allow(dead_code)]
            pub const TrackerWarning: Self = AlertType { repr: 12 };
            #[allow(dead_code)]
            pub const ScrapeReply: Self = AlertType { repr: 13 };
            #[allow(dead_code)]
            pub const ScrapeFailed: Self = AlertType { repr: 14 };
            #[allow(dead_code)]
            pub const TrackerReply: Self = AlertType { repr: 15 };
            #[allow(dead_code)]
            pub const DhtReply: Self = AlertType { repr: 16 };
            #[allow(dead_code)]
            pub const TrackerAnnounce: Self = AlertType { repr: 17 };
            #[allow(dead_code)]
            pub const HashFailed: Self = AlertType { repr: 18 };
            #[allow(dead_code)]
            pub const PeerBan: Self = AlertType { repr: 19 };
            #[allow(dead_code)]
            pub const PeerUnsnubbed: Self = AlertType { repr: 20 };
            #[allow(dead_code)]
            pub const PeerSnubbed: Self = AlertType { repr: 21 };
            #[allow(dead_code)]
            pub const PeerError: Self = AlertType { repr: 22 };
            #[allow(dead_code)]
            pub const PeerConnect: Self = AlertType { repr: 23 };
            #[allow(dead_code)]
            pub const PeerDisconnected: Self = AlertType { repr: 24 };
            #[allow(dead_code)]
            pub const InvalidRequest: Self = AlertType { repr: 25 };
            #[allow(dead_code)]
            pub const TorrentFinished: Self = AlertType { repr: 26 };
            #[allow(dead_code)]
            pub const PieceFinished: Self = AlertType { repr: 27 };
            #[allow(dead_code)]
            pub const RequestDropped: Self = AlertType { repr: 28 };
            #[allow(dead_code)]
            pub const BlockTimeout: Self = AlertType { repr: 29 };
            #[allow(dead_code)]
            pub const BlockFinished: Self = AlertType { repr: 30 };
            #[allow(dead_code)]
            pub const BlockDownloading: Self = AlertType { repr: 31 };
            #[allow(dead_code)]
            pub const UnwantedBlock: Self = AlertType { repr: 32 };
            #[allow(dead_code)]
            pub const StorageMoved: Self = AlertType { repr: 33 };
            #[allow(dead_code)]
            pub const StorageMovedFailed: Self = AlertType { repr: 34 };
            #[allow(dead_code)]
            pub const TorrentDeleted: Self = AlertType { repr: 35 };
            #[allow(dead_code)]
            pub const TorrentDeleteFailed: Self = AlertType { repr: 36 };
            #[allow(dead_code)]
            pub const SaveResumeData: Self = AlertType { repr: 37 };
            #[allow(dead_code)]
            pub const SaveResumeDataFailed: Self = AlertType { repr: 38 };
            #[allow(dead_code)]
            pub const TorrentPaused: Self = AlertType { repr: 39 };
            #[allow(dead_code)]
            pub const TorrentResumed: Self = AlertType { repr: 40 };
            #[allow(dead_code)]
            pub const TorrentChecked: Self = AlertType { repr: 41 };
            #[allow(dead_code)]
            pub const UrlSeed: Self = AlertType { repr: 42 };
            #[allow(dead_code)]
            pub const FileError: Self = AlertType { repr: 43 };
            #[allow(dead_code)]
            pub const MetadataFailed: Self = AlertType { repr: 44 };
            #[allow(dead_code)]
            pub const MetadataReceived: Self = AlertType { repr: 45 };
            #[allow(dead_code)]
            pub const UdpError: Self = AlertType { repr: 46 };
            #[allow(dead_code)]
            pub const ExternalIp: Self = AlertType { repr: 47 };
            #[allow(dead_code)]
            pub const ListenFailed: Self = AlertType { repr: 48 };
            #[allow(dead_code)]
            pub const ListenSucceeded: Self = AlertType { repr: 49 };
            #[allow(dead_code)]
            pub const PortmapError: Self = AlertType { repr: 50 };
            #[allow(dead_code)]
            pub const Portmap: Self = AlertType { repr: 51 };
            #[allow(dead_code)]
            pub const PortmapLog: Self = AlertType { repr: 52 };
            #[allow(dead_code)]
            pub const FastresumeRejected: Self = AlertType { repr: 53 };
            #[allow(dead_code)]
            pub const PeerBlocked: Self = AlertType { repr: 54 };
            #[allow(dead_code)]
            pub const DhtAnnounce: Self = AlertType { repr: 55 };
            #[allow(dead_code)]
            pub const DhtGetPeers: Self = AlertType { repr: 56 };
            #[allow(dead_code)]
            pub const CacheFlushed: Self = AlertType { repr: 58 };
            #[allow(dead_code)]
            pub const LsdPeer: Self = AlertType { repr: 60 };
            #[allow(dead_code)]
            pub const Trackerid: Self = AlertType { repr: 61 };
            #[allow(dead_code)]
            pub const DhtBootstrap: Self = AlertType { repr: 62 };
            #[allow(dead_code)]
            pub const TorrentError: Self = AlertType { repr: 64 };
            #[allow(dead_code)]
            pub const TorrentNeedCert: Self = AlertType { repr: 65 };
            #[allow(dead_code)]
            pub const IncomingConnection: Self = AlertType { repr: 66 };
            #[allow(dead_code)]
            pub const AddTorrent: Self = AlertType { repr: 67 };
            #[allow(dead_code)]
            pub const StateUpdate: Self = AlertType { repr: 68 };
            #[allow(dead_code)]
            pub const SessionStats: Self = AlertType { repr: 70 };
            #[allow(dead_code)]
            pub const DhtError: Self = AlertType { repr: 73 };
            #[allow(dead_code)]
            pub const DhtImmutableItem: Self = AlertType { repr: 74 };
            #[allow(dead_code)]
            pub const DhtMutableItem: Self = AlertType { repr: 75 };
            #[allow(dead_code)]
            pub const DhtPut: Self = AlertType { repr: 76 };
            #[allow(dead_code)]
            pub const I2p: Self = AlertType { repr: 77 };
            #[allow(dead_code)]
            pub const DhtOutgoingGetPeers: Self = AlertType { repr: 78 };
            #[allow(dead_code)]
            pub const Log: Self = AlertType { repr: 79 };
            #[allow(dead_code)]
            pub const TorrentLog: Self = AlertType { repr: 80 };
            #[allow(dead_code)]
            pub const PeerLog: Self = AlertType { repr: 81 };
            #[allow(dead_code)]
            pub const LsdError: Self = AlertType { repr: 82 };
            #[allow(dead_code)]
            pub const DhtStats: Self = AlertType { repr: 83 };
            #[allow(dead_code)]
            pub const IncomingRequest: Self = AlertType { repr: 84 };
            #[allow(dead_code)]
            pub const DhtLog: Self = AlertType { repr: 85 };
            #[allow(dead_code)]
            pub const DhtPkt: Self = AlertType { repr: 86 };
            #[allow(dead_code)]
            pub const DhtGetPeersReply: Self = AlertType { repr: 87 };
            #[allow(dead_code)]
            pub const DhtDirectResponse: Self = AlertType { repr: 88 };
            #[allow(dead_code)]
            pub const PickerLog: Self = AlertType { repr: 89 };
            #[allow(dead_code)]
            pub const SessionError: Self = AlertType { repr: 90 };
            #[allow(dead_code)]
            pub const DhtLiveNodes: Self = AlertType { repr: 91 };
            #[allow(dead_code)]
            pub const SessionStatsHeader: Self = AlertType { repr: 92 };
            #[allow(dead_code)]
            pub const DhtSampleInfohashes: Self = AlertType { repr: 93 };
            #[allow(dead_code)]
            pub const BlockUploaded: Self = AlertType { repr: 94 };
            #[allow(dead_code)]
            pub const AlertsDropped: Self = AlertType { repr: 95 };
            #[allow(dead_code)]
            pub const Socks5: Self = AlertType { repr: 96 };
            #[allow(dead_code)]
            pub const FilePrio: Self = AlertType { repr: 97 };
            #[allow(dead_code)]
            pub const OversizedFile: Self = AlertType { repr: 98 };
            #[allow(dead_code)]
            pub const TorrentConflict: Self = AlertType { repr: 99 };
            #[allow(dead_code)]
            pub const PeerInfo: Self = AlertType { repr: 100 };
            #[allow(dead_code)]
            pub const FileProgress: Self = AlertType { repr: 101 };
            #[allow(dead_code)]
            pub const PieceInfo: Self = AlertType { repr: 102 };
            #[allow(dead_code)]
            pub const PieceAvailability: Self = AlertType { repr: 103 };
            #[allow(dead_code)]
            pub const TrackerList: Self = AlertType { repr: 104 };
            #[allow(dead_code)]
            pub const Unknown: Self = AlertType { repr: 105 };
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for AlertType {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::t,
                ::cxx::r,
                ::cxx::s,
                (),
                ::cxx::A,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
                ::cxx::T,
                ::cxx::y,
                ::cxx::p,
                ::cxx::e,
            );
            type Kind = ::cxx::kind::Trivial;
        }
        #[automatically_derived]
        impl ::cxx::core::marker::Copy for AlertType {}
        #[automatically_derived]
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl ::cxx::core::clone::Clone for AlertType {
            fn clone(&self) -> Self {
                *self
            }
        }
        #[repr(C)]
        pub struct CastAlertRaw {
            pub type_: AlertType,
            pub alert: *mut alert,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for CastAlertRaw {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::t,
                ::cxx::r,
                ::cxx::s,
                (),
                ::cxx::C,
                ::cxx::a,
                ::cxx::s,
                ::cxx::t,
                ::cxx::A,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
                ::cxx::R,
                ::cxx::a,
                ::cxx::w,
            );
            type Kind = ::cxx::kind::Trivial;
        }
        pub type torrent_handle = crate::ffi::torrent_handle::ffi::torrent_handle;
        #[repr(C)]
        pub struct session {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for session {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::e,
                ::cxx::s,
                ::cxx::s,
                ::cxx::i,
                ::cxx::o,
                ::cxx::n,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct add_torrent_params {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for add_torrent_params {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::a,
                ::cxx::d,
                ::cxx::d,
                ::cxx::__,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::p,
                ::cxx::a,
                ::cxx::r,
                ::cxx::a,
                ::cxx::m,
                ::cxx::s,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct settings_pack {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for settings_pack {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::e,
                ::cxx::t,
                ::cxx::t,
                ::cxx::i,
                ::cxx::n,
                ::cxx::g,
                ::cxx::s,
                ::cxx::__,
                ::cxx::p,
                ::cxx::a,
                ::cxx::c,
                ::cxx::k,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_status {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_status {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::s,
                ::cxx::t,
                ::cxx::a,
                ::cxx::t,
                ::cxx::u,
                ::cxx::s,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct tracker_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for tracker_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::r,
                ::cxx::a,
                ::cxx::c,
                ::cxx::k,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct peer_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for peer_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_removed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_removed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::m,
                ::cxx::o,
                ::cxx::v,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct read_piece_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for read_piece_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::r,
                ::cxx::e,
                ::cxx::a,
                ::cxx::d,
                ::cxx::__,
                ::cxx::p,
                ::cxx::i,
                ::cxx::e,
                ::cxx::c,
                ::cxx::e,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct file_completed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for file_completed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::f,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::__,
                ::cxx::c,
                ::cxx::o,
                ::cxx::m,
                ::cxx::p,
                ::cxx::l,
                ::cxx::e,
                ::cxx::t,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct file_renamed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for file_renamed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::f,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::a,
                ::cxx::m,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct file_rename_failed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for file_rename_failed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::f,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::a,
                ::cxx::m,
                ::cxx::e,
                ::cxx::__,
                ::cxx::f,
                ::cxx::a,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct performance_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for performance_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::e,
                ::cxx::r,
                ::cxx::f,
                ::cxx::o,
                ::cxx::r,
                ::cxx::m,
                ::cxx::a,
                ::cxx::n,
                ::cxx::c,
                ::cxx::e,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct state_changed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for state_changed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::t,
                ::cxx::a,
                ::cxx::t,
                ::cxx::e,
                ::cxx::__,
                ::cxx::c,
                ::cxx::h,
                ::cxx::a,
                ::cxx::n,
                ::cxx::g,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct tracker_error_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for tracker_error_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::r,
                ::cxx::a,
                ::cxx::c,
                ::cxx::k,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::e,
                ::cxx::r,
                ::cxx::r,
                ::cxx::o,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct tracker_warning_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for tracker_warning_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::r,
                ::cxx::a,
                ::cxx::c,
                ::cxx::k,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::w,
                ::cxx::a,
                ::cxx::r,
                ::cxx::n,
                ::cxx::i,
                ::cxx::n,
                ::cxx::g,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct scrape_reply_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for scrape_reply_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::c,
                ::cxx::r,
                ::cxx::a,
                ::cxx::p,
                ::cxx::e,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::p,
                ::cxx::l,
                ::cxx::y,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct scrape_failed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for scrape_failed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::c,
                ::cxx::r,
                ::cxx::a,
                ::cxx::p,
                ::cxx::e,
                ::cxx::__,
                ::cxx::f,
                ::cxx::a,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct tracker_reply_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for tracker_reply_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::r,
                ::cxx::a,
                ::cxx::c,
                ::cxx::k,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::p,
                ::cxx::l,
                ::cxx::y,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_reply_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_reply_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::p,
                ::cxx::l,
                ::cxx::y,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct tracker_announce_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for tracker_announce_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::r,
                ::cxx::a,
                ::cxx::c,
                ::cxx::k,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::n,
                ::cxx::n,
                ::cxx::o,
                ::cxx::u,
                ::cxx::n,
                ::cxx::c,
                ::cxx::e,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct hash_failed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for hash_failed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::h,
                ::cxx::a,
                ::cxx::s,
                ::cxx::h,
                ::cxx::__,
                ::cxx::f,
                ::cxx::a,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct peer_ban_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for peer_ban_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::b,
                ::cxx::a,
                ::cxx::n,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct peer_unsnubbed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for peer_unsnubbed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::u,
                ::cxx::n,
                ::cxx::s,
                ::cxx::n,
                ::cxx::u,
                ::cxx::b,
                ::cxx::b,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct peer_snubbed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for peer_snubbed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::s,
                ::cxx::n,
                ::cxx::u,
                ::cxx::b,
                ::cxx::b,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct peer_error_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for peer_error_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::e,
                ::cxx::r,
                ::cxx::r,
                ::cxx::o,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct peer_connect_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for peer_connect_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::c,
                ::cxx::o,
                ::cxx::n,
                ::cxx::n,
                ::cxx::e,
                ::cxx::c,
                ::cxx::t,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct peer_disconnected_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for peer_disconnected_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::d,
                ::cxx::i,
                ::cxx::s,
                ::cxx::c,
                ::cxx::o,
                ::cxx::n,
                ::cxx::n,
                ::cxx::e,
                ::cxx::c,
                ::cxx::t,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct invalid_request_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for invalid_request_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::i,
                ::cxx::n,
                ::cxx::v,
                ::cxx::a,
                ::cxx::l,
                ::cxx::i,
                ::cxx::d,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::q,
                ::cxx::u,
                ::cxx::e,
                ::cxx::s,
                ::cxx::t,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_finished_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_finished_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::f,
                ::cxx::i,
                ::cxx::n,
                ::cxx::i,
                ::cxx::s,
                ::cxx::h,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct piece_finished_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for piece_finished_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::i,
                ::cxx::e,
                ::cxx::c,
                ::cxx::e,
                ::cxx::__,
                ::cxx::f,
                ::cxx::i,
                ::cxx::n,
                ::cxx::i,
                ::cxx::s,
                ::cxx::h,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct request_dropped_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for request_dropped_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::r,
                ::cxx::e,
                ::cxx::q,
                ::cxx::u,
                ::cxx::e,
                ::cxx::s,
                ::cxx::t,
                ::cxx::__,
                ::cxx::d,
                ::cxx::r,
                ::cxx::o,
                ::cxx::p,
                ::cxx::p,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct block_timeout_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for block_timeout_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::b,
                ::cxx::l,
                ::cxx::o,
                ::cxx::c,
                ::cxx::k,
                ::cxx::__,
                ::cxx::t,
                ::cxx::i,
                ::cxx::m,
                ::cxx::e,
                ::cxx::o,
                ::cxx::u,
                ::cxx::t,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct block_finished_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for block_finished_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::b,
                ::cxx::l,
                ::cxx::o,
                ::cxx::c,
                ::cxx::k,
                ::cxx::__,
                ::cxx::f,
                ::cxx::i,
                ::cxx::n,
                ::cxx::i,
                ::cxx::s,
                ::cxx::h,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct block_downloading_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for block_downloading_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::b,
                ::cxx::l,
                ::cxx::o,
                ::cxx::c,
                ::cxx::k,
                ::cxx::__,
                ::cxx::d,
                ::cxx::o,
                ::cxx::w,
                ::cxx::n,
                ::cxx::l,
                ::cxx::o,
                ::cxx::a,
                ::cxx::d,
                ::cxx::i,
                ::cxx::n,
                ::cxx::g,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct unwanted_block_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for unwanted_block_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::u,
                ::cxx::n,
                ::cxx::w,
                ::cxx::a,
                ::cxx::n,
                ::cxx::t,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::b,
                ::cxx::l,
                ::cxx::o,
                ::cxx::c,
                ::cxx::k,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct storage_moved_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for storage_moved_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::a,
                ::cxx::g,
                ::cxx::e,
                ::cxx::__,
                ::cxx::m,
                ::cxx::o,
                ::cxx::v,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct storage_moved_failed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for storage_moved_failed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::a,
                ::cxx::g,
                ::cxx::e,
                ::cxx::__,
                ::cxx::m,
                ::cxx::o,
                ::cxx::v,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::f,
                ::cxx::a,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_deleted_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_deleted_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::d,
                ::cxx::e,
                ::cxx::l,
                ::cxx::e,
                ::cxx::t,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_delete_failed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_delete_failed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::d,
                ::cxx::e,
                ::cxx::l,
                ::cxx::e,
                ::cxx::t,
                ::cxx::e,
                ::cxx::__,
                ::cxx::f,
                ::cxx::a,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct save_resume_data_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for save_resume_data_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::a,
                ::cxx::v,
                ::cxx::e,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::s,
                ::cxx::u,
                ::cxx::m,
                ::cxx::e,
                ::cxx::__,
                ::cxx::d,
                ::cxx::a,
                ::cxx::t,
                ::cxx::a,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct save_resume_data_failed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for save_resume_data_failed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::a,
                ::cxx::v,
                ::cxx::e,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::s,
                ::cxx::u,
                ::cxx::m,
                ::cxx::e,
                ::cxx::__,
                ::cxx::d,
                ::cxx::a,
                ::cxx::t,
                ::cxx::a,
                ::cxx::__,
                ::cxx::f,
                ::cxx::a,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_paused_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_paused_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::p,
                ::cxx::a,
                ::cxx::u,
                ::cxx::s,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_resumed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_resumed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::s,
                ::cxx::u,
                ::cxx::m,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_checked_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_checked_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::c,
                ::cxx::h,
                ::cxx::e,
                ::cxx::c,
                ::cxx::k,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct url_seed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for url_seed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::u,
                ::cxx::r,
                ::cxx::l,
                ::cxx::__,
                ::cxx::s,
                ::cxx::e,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct file_error_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for file_error_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::f,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::__,
                ::cxx::e,
                ::cxx::r,
                ::cxx::r,
                ::cxx::o,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct metadata_failed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for metadata_failed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::m,
                ::cxx::e,
                ::cxx::t,
                ::cxx::a,
                ::cxx::d,
                ::cxx::a,
                ::cxx::t,
                ::cxx::a,
                ::cxx::__,
                ::cxx::f,
                ::cxx::a,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct metadata_received_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for metadata_received_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::m,
                ::cxx::e,
                ::cxx::t,
                ::cxx::a,
                ::cxx::d,
                ::cxx::a,
                ::cxx::t,
                ::cxx::a,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::c,
                ::cxx::e,
                ::cxx::i,
                ::cxx::v,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct udp_error_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for udp_error_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::u,
                ::cxx::d,
                ::cxx::p,
                ::cxx::__,
                ::cxx::e,
                ::cxx::r,
                ::cxx::r,
                ::cxx::o,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct external_ip_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for external_ip_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::e,
                ::cxx::x,
                ::cxx::t,
                ::cxx::e,
                ::cxx::r,
                ::cxx::n,
                ::cxx::a,
                ::cxx::l,
                ::cxx::__,
                ::cxx::i,
                ::cxx::p,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct listen_failed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for listen_failed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::l,
                ::cxx::i,
                ::cxx::s,
                ::cxx::t,
                ::cxx::e,
                ::cxx::n,
                ::cxx::__,
                ::cxx::f,
                ::cxx::a,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct listen_succeeded_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for listen_succeeded_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::l,
                ::cxx::i,
                ::cxx::s,
                ::cxx::t,
                ::cxx::e,
                ::cxx::n,
                ::cxx::__,
                ::cxx::s,
                ::cxx::u,
                ::cxx::c,
                ::cxx::c,
                ::cxx::e,
                ::cxx::e,
                ::cxx::d,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct portmap_error_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for portmap_error_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::o,
                ::cxx::r,
                ::cxx::t,
                ::cxx::m,
                ::cxx::a,
                ::cxx::p,
                ::cxx::__,
                ::cxx::e,
                ::cxx::r,
                ::cxx::r,
                ::cxx::o,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct portmap_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for portmap_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::o,
                ::cxx::r,
                ::cxx::t,
                ::cxx::m,
                ::cxx::a,
                ::cxx::p,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct portmap_log_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for portmap_log_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::o,
                ::cxx::r,
                ::cxx::t,
                ::cxx::m,
                ::cxx::a,
                ::cxx::p,
                ::cxx::__,
                ::cxx::l,
                ::cxx::o,
                ::cxx::g,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct fastresume_rejected_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for fastresume_rejected_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::f,
                ::cxx::a,
                ::cxx::s,
                ::cxx::t,
                ::cxx::r,
                ::cxx::e,
                ::cxx::s,
                ::cxx::u,
                ::cxx::m,
                ::cxx::e,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::j,
                ::cxx::e,
                ::cxx::c,
                ::cxx::t,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct peer_blocked_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for peer_blocked_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::b,
                ::cxx::l,
                ::cxx::o,
                ::cxx::c,
                ::cxx::k,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_announce_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_announce_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::a,
                ::cxx::n,
                ::cxx::n,
                ::cxx::o,
                ::cxx::u,
                ::cxx::n,
                ::cxx::c,
                ::cxx::e,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_get_peers_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_get_peers_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::g,
                ::cxx::e,
                ::cxx::t,
                ::cxx::__,
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::s,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct cache_flushed_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for cache_flushed_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::c,
                ::cxx::a,
                ::cxx::c,
                ::cxx::h,
                ::cxx::e,
                ::cxx::__,
                ::cxx::f,
                ::cxx::l,
                ::cxx::u,
                ::cxx::s,
                ::cxx::h,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct lsd_peer_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for lsd_peer_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::l,
                ::cxx::s,
                ::cxx::d,
                ::cxx::__,
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct trackerid_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for trackerid_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::r,
                ::cxx::a,
                ::cxx::c,
                ::cxx::k,
                ::cxx::e,
                ::cxx::r,
                ::cxx::i,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_bootstrap_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_bootstrap_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::b,
                ::cxx::o,
                ::cxx::o,
                ::cxx::t,
                ::cxx::s,
                ::cxx::t,
                ::cxx::r,
                ::cxx::a,
                ::cxx::p,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_error_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_error_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::e,
                ::cxx::r,
                ::cxx::r,
                ::cxx::o,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_need_cert_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_need_cert_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::n,
                ::cxx::e,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::c,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct incoming_connection_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for incoming_connection_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::i,
                ::cxx::n,
                ::cxx::c,
                ::cxx::o,
                ::cxx::m,
                ::cxx::i,
                ::cxx::n,
                ::cxx::g,
                ::cxx::__,
                ::cxx::c,
                ::cxx::o,
                ::cxx::n,
                ::cxx::n,
                ::cxx::e,
                ::cxx::c,
                ::cxx::t,
                ::cxx::i,
                ::cxx::o,
                ::cxx::n,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct add_torrent_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for add_torrent_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::a,
                ::cxx::d,
                ::cxx::d,
                ::cxx::__,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct state_update_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for state_update_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::t,
                ::cxx::a,
                ::cxx::t,
                ::cxx::e,
                ::cxx::__,
                ::cxx::u,
                ::cxx::p,
                ::cxx::d,
                ::cxx::a,
                ::cxx::t,
                ::cxx::e,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct session_stats_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for session_stats_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::e,
                ::cxx::s,
                ::cxx::s,
                ::cxx::i,
                ::cxx::o,
                ::cxx::n,
                ::cxx::__,
                ::cxx::s,
                ::cxx::t,
                ::cxx::a,
                ::cxx::t,
                ::cxx::s,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_error_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_error_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::e,
                ::cxx::r,
                ::cxx::r,
                ::cxx::o,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_immutable_item_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_immutable_item_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::i,
                ::cxx::m,
                ::cxx::m,
                ::cxx::u,
                ::cxx::t,
                ::cxx::a,
                ::cxx::b,
                ::cxx::l,
                ::cxx::e,
                ::cxx::__,
                ::cxx::i,
                ::cxx::t,
                ::cxx::e,
                ::cxx::m,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_mutable_item_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_mutable_item_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::m,
                ::cxx::u,
                ::cxx::t,
                ::cxx::a,
                ::cxx::b,
                ::cxx::l,
                ::cxx::e,
                ::cxx::__,
                ::cxx::i,
                ::cxx::t,
                ::cxx::e,
                ::cxx::m,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_put_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_put_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::p,
                ::cxx::u,
                ::cxx::t,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct i2p_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for i2p_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::i,
                ::cxx::_2,
                ::cxx::p,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_outgoing_get_peers_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_outgoing_get_peers_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::o,
                ::cxx::u,
                ::cxx::t,
                ::cxx::g,
                ::cxx::o,
                ::cxx::i,
                ::cxx::n,
                ::cxx::g,
                ::cxx::__,
                ::cxx::g,
                ::cxx::e,
                ::cxx::t,
                ::cxx::__,
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::s,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct log_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for log_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::l,
                ::cxx::o,
                ::cxx::g,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_log_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_log_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::l,
                ::cxx::o,
                ::cxx::g,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct peer_log_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for peer_log_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::l,
                ::cxx::o,
                ::cxx::g,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct lsd_error_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for lsd_error_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::l,
                ::cxx::s,
                ::cxx::d,
                ::cxx::__,
                ::cxx::e,
                ::cxx::r,
                ::cxx::r,
                ::cxx::o,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_stats_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_stats_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::s,
                ::cxx::t,
                ::cxx::a,
                ::cxx::t,
                ::cxx::s,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct incoming_request_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for incoming_request_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::i,
                ::cxx::n,
                ::cxx::c,
                ::cxx::o,
                ::cxx::m,
                ::cxx::i,
                ::cxx::n,
                ::cxx::g,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::q,
                ::cxx::u,
                ::cxx::e,
                ::cxx::s,
                ::cxx::t,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_log_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_log_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::l,
                ::cxx::o,
                ::cxx::g,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_pkt_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_pkt_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::p,
                ::cxx::k,
                ::cxx::t,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_get_peers_reply_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_get_peers_reply_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::g,
                ::cxx::e,
                ::cxx::t,
                ::cxx::__,
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::s,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::p,
                ::cxx::l,
                ::cxx::y,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_direct_response_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_direct_response_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::d,
                ::cxx::i,
                ::cxx::r,
                ::cxx::e,
                ::cxx::c,
                ::cxx::t,
                ::cxx::__,
                ::cxx::r,
                ::cxx::e,
                ::cxx::s,
                ::cxx::p,
                ::cxx::o,
                ::cxx::n,
                ::cxx::s,
                ::cxx::e,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct picker_log_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for picker_log_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::i,
                ::cxx::c,
                ::cxx::k,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::l,
                ::cxx::o,
                ::cxx::g,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct session_error_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for session_error_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::e,
                ::cxx::s,
                ::cxx::s,
                ::cxx::i,
                ::cxx::o,
                ::cxx::n,
                ::cxx::__,
                ::cxx::e,
                ::cxx::r,
                ::cxx::r,
                ::cxx::o,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_live_nodes_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_live_nodes_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::l,
                ::cxx::i,
                ::cxx::v,
                ::cxx::e,
                ::cxx::__,
                ::cxx::n,
                ::cxx::o,
                ::cxx::d,
                ::cxx::e,
                ::cxx::s,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct session_stats_header_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for session_stats_header_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::e,
                ::cxx::s,
                ::cxx::s,
                ::cxx::i,
                ::cxx::o,
                ::cxx::n,
                ::cxx::__,
                ::cxx::s,
                ::cxx::t,
                ::cxx::a,
                ::cxx::t,
                ::cxx::s,
                ::cxx::__,
                ::cxx::h,
                ::cxx::e,
                ::cxx::a,
                ::cxx::d,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct dht_sample_infohashes_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for dht_sample_infohashes_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::d,
                ::cxx::h,
                ::cxx::t,
                ::cxx::__,
                ::cxx::s,
                ::cxx::a,
                ::cxx::m,
                ::cxx::p,
                ::cxx::l,
                ::cxx::e,
                ::cxx::__,
                ::cxx::i,
                ::cxx::n,
                ::cxx::f,
                ::cxx::o,
                ::cxx::h,
                ::cxx::a,
                ::cxx::s,
                ::cxx::h,
                ::cxx::e,
                ::cxx::s,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct block_uploaded_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for block_uploaded_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::b,
                ::cxx::l,
                ::cxx::o,
                ::cxx::c,
                ::cxx::k,
                ::cxx::__,
                ::cxx::u,
                ::cxx::p,
                ::cxx::l,
                ::cxx::o,
                ::cxx::a,
                ::cxx::d,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct alerts_dropped_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for alerts_dropped_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
                ::cxx::s,
                ::cxx::__,
                ::cxx::d,
                ::cxx::r,
                ::cxx::o,
                ::cxx::p,
                ::cxx::p,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct socks5_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for socks5_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::s,
                ::cxx::o,
                ::cxx::c,
                ::cxx::k,
                ::cxx::s,
                ::cxx::_5,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct file_prio_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for file_prio_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::f,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::__,
                ::cxx::p,
                ::cxx::r,
                ::cxx::i,
                ::cxx::o,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct oversized_file_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for oversized_file_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::o,
                ::cxx::v,
                ::cxx::e,
                ::cxx::r,
                ::cxx::s,
                ::cxx::i,
                ::cxx::z,
                ::cxx::e,
                ::cxx::d,
                ::cxx::__,
                ::cxx::f,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_conflict_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_conflict_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::c,
                ::cxx::o,
                ::cxx::n,
                ::cxx::f,
                ::cxx::l,
                ::cxx::i,
                ::cxx::c,
                ::cxx::t,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct peer_info_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for peer_info_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::e,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::i,
                ::cxx::n,
                ::cxx::f,
                ::cxx::o,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct file_progress_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for file_progress_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::f,
                ::cxx::i,
                ::cxx::l,
                ::cxx::e,
                ::cxx::__,
                ::cxx::p,
                ::cxx::r,
                ::cxx::o,
                ::cxx::g,
                ::cxx::r,
                ::cxx::e,
                ::cxx::s,
                ::cxx::s,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct piece_info_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for piece_info_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::i,
                ::cxx::e,
                ::cxx::c,
                ::cxx::e,
                ::cxx::__,
                ::cxx::i,
                ::cxx::n,
                ::cxx::f,
                ::cxx::o,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct piece_availability_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for piece_availability_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::p,
                ::cxx::i,
                ::cxx::e,
                ::cxx::c,
                ::cxx::e,
                ::cxx::__,
                ::cxx::a,
                ::cxx::v,
                ::cxx::a,
                ::cxx::i,
                ::cxx::l,
                ::cxx::a,
                ::cxx::b,
                ::cxx::i,
                ::cxx::l,
                ::cxx::i,
                ::cxx::t,
                ::cxx::y,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct tracker_list_alert {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for tracker_list_alert {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::r,
                ::cxx::a,
                ::cxx::c,
                ::cxx::k,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::l,
                ::cxx::i,
                ::cxx::s,
                ::cxx::t,
                ::cxx::__,
                ::cxx::a,
                ::cxx::l,
                ::cxx::e,
                ::cxx::r,
                ::cxx::t,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct torrent_info {
            _private: ::cxx::private::Opaque,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for torrent_info {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::i,
                ::cxx::b,
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                (),
                ::cxx::t,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::__,
                ::cxx::i,
                ::cxx::n,
                ::cxx::f,
                ::cxx::o,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct AddTorrentParamsValues {
            pub version: i32,
            pub ti: ::cxx::SharedPtr<torrent_info>,
            pub name: ::cxx::alloc::string::String,
            pub trackers: ::cxx::alloc::vec::Vec<::cxx::alloc::string::String>,
            pub tracker_tiers: ::cxx::alloc::vec::Vec<i32>,
            pub save_path: ::cxx::alloc::string::String,
        }
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for AddTorrentParamsValues {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::l,
                ::cxx::t,
                ::cxx::r,
                ::cxx::s,
                (),
                ::cxx::A,
                ::cxx::d,
                ::cxx::d,
                ::cxx::T,
                ::cxx::o,
                ::cxx::r,
                ::cxx::r,
                ::cxx::e,
                ::cxx::n,
                ::cxx::t,
                ::cxx::P,
                ::cxx::a,
                ::cxx::r,
                ::cxx::a,
                ::cxx::m,
                ::cxx::s,
                ::cxx::V,
                ::cxx::a,
                ::cxx::l,
                ::cxx::u,
                ::cxx::e,
                ::cxx::s,
            );
            type Kind = ::cxx::kind::Trivial;
        }
        pub fn lt_parse_magnet_uri(uri: &str) -> ::cxx::UniquePtr<add_torrent_params> {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_parse_magnet_uri"]
                fn __lt_parse_magnet_uri(
                    uri: ::cxx::private::RustStr,
                ) -> *mut add_torrent_params;
            }
            unsafe {
                ::cxx::UniquePtr::from_raw(
                    __lt_parse_magnet_uri(::cxx::private::RustStr::from(uri)),
                )
            }
        }
        pub fn lt_create_session() -> ::cxx::UniquePtr<session> {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_create_session"]
                fn __lt_create_session() -> *mut session;
            }
            unsafe { ::cxx::UniquePtr::from_raw(__lt_create_session()) }
        }
        pub fn lt_create_session_with_settings(
            settings: &settings_pack,
        ) -> ::cxx::UniquePtr<session> {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_create_session_with_settings"]
                fn __lt_create_session_with_settings(
                    settings: &settings_pack,
                ) -> *mut session;
            }
            unsafe {
                ::cxx::UniquePtr::from_raw(__lt_create_session_with_settings(settings))
            }
        }
        pub fn lt_session_add_torrent(
            session: ::cxx::core::pin::Pin<&mut session>,
            params: &add_torrent_params,
        ) -> ::cxx::UniquePtr<torrent_handle> {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_session_add_torrent"]
                fn __lt_session_add_torrent(
                    session: ::cxx::core::pin::Pin<&mut session>,
                    params: &add_torrent_params,
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            unsafe {
                ::cxx::UniquePtr::from_raw(
                    __lt_session_add_torrent(session, params).cast(),
                )
            }
        }
        pub fn lt_session_async_add_torrent(
            session: ::cxx::core::pin::Pin<&mut session>,
            params: &add_torrent_params,
        ) {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_session_async_add_torrent"]
                fn __lt_session_async_add_torrent(
                    session: ::cxx::core::pin::Pin<&mut session>,
                    params: &add_torrent_params,
                );
            }
            unsafe {
                __lt_session_async_add_torrent(session, params);
            }
        }
        pub fn lt_session_pop_alerts(
            session: ::cxx::core::pin::Pin<&mut session>,
        ) -> ::cxx::alloc::vec::Vec<CastAlertRaw> {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_session_pop_alerts"]
                fn __lt_session_pop_alerts(
                    session: ::cxx::core::pin::Pin<&mut session>,
                    __return: *mut ::cxx::private::RustVec<CastAlertRaw>,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<
                    ::cxx::private::RustVec<CastAlertRaw>,
                >::uninit();
                __lt_session_pop_alerts(session, __return.as_mut_ptr());
                __return.assume_init().into_vec()
            }
        }
        pub fn lt_session_post_torrent_updates(
            session: ::cxx::core::pin::Pin<&mut session>,
            flags: u32,
        ) {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_session_post_torrent_updates"]
                fn __lt_session_post_torrent_updates(
                    session: ::cxx::core::pin::Pin<&mut session>,
                    flags: u32,
                );
            }
            unsafe {
                __lt_session_post_torrent_updates(session, flags);
            }
        }
        pub fn lt_create_settings_pack() -> ::cxx::UniquePtr<settings_pack> {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_create_settings_pack"]
                fn __lt_create_settings_pack() -> *mut settings_pack;
            }
            unsafe { ::cxx::UniquePtr::from_raw(__lt_create_settings_pack()) }
        }
        pub fn lt_set_alert_mask(
            pack: ::cxx::core::pin::Pin<&mut settings_pack>,
            mask: u32,
        ) {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_set_alert_mask"]
                fn __lt_set_alert_mask(
                    pack: ::cxx::core::pin::Pin<&mut settings_pack>,
                    mask: u32,
                );
            }
            unsafe {
                __lt_set_alert_mask(pack, mask);
            }
        }
        pub fn lt_set_add_torrent_params_path(
            params: ::cxx::core::pin::Pin<&mut add_torrent_params>,
            path: &str,
        ) {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_set_add_torrent_params_path"]
                fn __lt_set_add_torrent_params_path(
                    params: ::cxx::core::pin::Pin<&mut add_torrent_params>,
                    path: ::cxx::private::RustStr,
                );
            }
            unsafe {
                __lt_set_add_torrent_params_path(
                    params,
                    ::cxx::private::RustStr::from(path),
                );
            }
        }
        pub fn lt_add_torrent_params_info_hash(
            params: &add_torrent_params,
        ) -> InfoHashCpp {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_add_torrent_params_info_hash"]
                fn __lt_add_torrent_params_info_hash(
                    params: &add_torrent_params,
                    __return: *mut InfoHashCpp,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<
                    InfoHashCpp,
                >::uninit();
                __lt_add_torrent_params_info_hash(params, __return.as_mut_ptr());
                __return.assume_init()
            }
        }
        pub fn lt_write_resume_data_buf(
            params: &add_torrent_params,
        ) -> ::cxx::alloc::vec::Vec<u8> {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_write_resume_data_buf"]
                fn __lt_write_resume_data_buf(
                    params: &add_torrent_params,
                    __return: *mut ::cxx::private::RustVec<u8>,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<
                    ::cxx::private::RustVec<u8>,
                >::uninit();
                __lt_write_resume_data_buf(params, __return.as_mut_ptr());
                __return.assume_init().into_vec()
            }
        }
        pub fn lt_read_resume_data(buf: &[u8]) -> ::cxx::UniquePtr<add_torrent_params> {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_read_resume_data"]
                fn __lt_read_resume_data(
                    buf: ::cxx::private::RustSlice,
                ) -> *mut add_torrent_params;
            }
            unsafe {
                ::cxx::UniquePtr::from_raw(
                    __lt_read_resume_data(::cxx::private::RustSlice::from_ref(buf)),
                )
            }
        }
        pub fn lt_torrent_status_handle(
            status: &torrent_status,
        ) -> ::cxx::UniquePtr<torrent_handle> {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_torrent_status_handle"]
                fn __lt_torrent_status_handle(
                    status: &torrent_status,
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            unsafe {
                ::cxx::UniquePtr::from_raw(__lt_torrent_status_handle(status).cast())
            }
        }
        pub fn lt_torrent_status_state(status: &torrent_status) -> u8 {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_torrent_status_state"]
                fn __lt_torrent_status_state(status: &torrent_status) -> u8;
            }
            unsafe { __lt_torrent_status_state(status) }
        }
        pub fn lt_torrent_status_progress(status: &torrent_status) -> f64 {
            unsafe extern "C" {
                #[link_name = "ltrs$cxxbridge1$lt_torrent_status_progress"]
                fn __lt_torrent_status_progress(status: &torrent_status) -> f64;
            }
            unsafe { __lt_torrent_status_progress(status) }
        }
        #[automatically_derived]
        unsafe impl ::cxx::memory::UniquePtrTarget for torrent_status {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("torrent_status")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$torrent_status$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __null(&raw mut repr);
                }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$torrent_status$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __raw(&raw mut repr, raw.cast());
                }
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$torrent_status$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { __get(&raw const repr).cast() }
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$torrent_status$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe { __release(&raw mut repr).cast() }
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$torrent_status$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                unsafe {
                    __drop(&raw mut repr);
                }
            }
        }
        #[automatically_derived]
        unsafe impl ::cxx::memory::SharedPtrTarget for torrent_info {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("torrent_info")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$libtorrent$torrent_info$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                unsafe {
                    __null(new);
                }
            }
            #[track_caller]
            unsafe fn __raw(new: *mut ::cxx::core::ffi::c_void, raw: *mut Self) {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$libtorrent$torrent_info$raw"]
                    fn __raw(
                        new: *const ::cxx::core::ffi::c_void,
                        raw: *mut ::cxx::core::ffi::c_void,
                    ) -> ::cxx::core::primitive::bool;
                }
                if !unsafe { __raw(new, raw as *mut ::cxx::core::ffi::c_void) } {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("libtorrent::torrent_info is not destructible"),
                        );
                    };
                }
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$libtorrent$torrent_info$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                unsafe {
                    __clone(this, new);
                }
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$libtorrent$torrent_info$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { __get(this).cast() }
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$libtorrent$torrent_info$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                unsafe {
                    __drop(this);
                }
            }
        }
        #[automatically_derived]
        unsafe impl ::cxx::memory::UniquePtrTarget for add_torrent_params {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("add_torrent_params")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$add_torrent_params$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __null(&raw mut repr);
                }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$add_torrent_params$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __raw(&raw mut repr, raw.cast());
                }
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$add_torrent_params$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { __get(&raw const repr).cast() }
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$add_torrent_params$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe { __release(&raw mut repr).cast() }
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$add_torrent_params$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                unsafe {
                    __drop(&raw mut repr);
                }
            }
        }
        #[automatically_derived]
        unsafe impl ::cxx::memory::UniquePtrTarget for session {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("session")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$session$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __null(&raw mut repr);
                }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$session$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __raw(&raw mut repr, raw.cast());
                }
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$session$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { __get(&raw const repr).cast() }
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$session$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe { __release(&raw mut repr).cast() }
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$session$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                unsafe {
                    __drop(&raw mut repr);
                }
            }
        }
        #[automatically_derived]
        unsafe impl ::cxx::memory::UniquePtrTarget for settings_pack {
            fn __typename(
                f: &mut ::cxx::core::fmt::Formatter<'_>,
            ) -> ::cxx::core::fmt::Result {
                f.write_str("settings_pack")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$settings_pack$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __null(&raw mut repr);
                }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$settings_pack$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __raw(&raw mut repr, raw.cast());
                }
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$settings_pack$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { __get(&raw const repr).cast() }
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$settings_pack$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe { __release(&raw mut repr).cast() }
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                unsafe extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$libtorrent$settings_pack$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<
                            *mut ::cxx::core::ffi::c_void,
                        >,
                    );
                }
                unsafe {
                    __drop(&raw mut repr);
                }
            }
        }
        #[doc(hidden)]
        const _: () = {
            const _: fn() = ::cxx::private::verify_extern_type::<
                torrent_handle,
                (
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    ::cxx::t,
                    ::cxx::o,
                    ::cxx::r,
                    ::cxx::r,
                    ::cxx::e,
                    ::cxx::n,
                    ::cxx::t,
                    (),
                    ::cxx::t,
                    ::cxx::o,
                    ::cxx::r,
                    ::cxx::r,
                    ::cxx::e,
                    ::cxx::n,
                    ::cxx::t,
                    ::cxx::__,
                    ::cxx::h,
                    ::cxx::a,
                    ::cxx::n,
                    ::cxx::d,
                    ::cxx::l,
                    ::cxx::e,
                ),
            >;
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <session as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <add_torrent_params as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <settings_pack as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_status as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <tracker_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <peer_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_removed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <read_piece_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <file_completed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <file_renamed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <file_rename_failed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <performance_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <state_changed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <tracker_error_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <tracker_warning_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <scrape_reply_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <scrape_failed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <tracker_reply_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_reply_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <tracker_announce_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <hash_failed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <peer_ban_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <peer_unsnubbed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <peer_snubbed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <peer_error_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <peer_connect_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <peer_disconnected_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <invalid_request_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_finished_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <piece_finished_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <request_dropped_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <block_timeout_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <block_finished_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <block_downloading_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <unwanted_block_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <storage_moved_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <storage_moved_failed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_deleted_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_delete_failed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <save_resume_data_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <save_resume_data_failed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_paused_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_resumed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_checked_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <url_seed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <file_error_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <metadata_failed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <metadata_received_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <udp_error_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <external_ip_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <listen_failed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <listen_succeeded_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <portmap_error_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <portmap_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <portmap_log_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <fastresume_rejected_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <peer_blocked_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_announce_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_get_peers_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <cache_flushed_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <lsd_peer_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <trackerid_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_bootstrap_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_error_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_need_cert_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <incoming_connection_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <add_torrent_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <state_update_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <session_stats_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_error_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_immutable_item_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_mutable_item_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_put_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <i2p_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_outgoing_get_peers_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <log_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_log_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <peer_log_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <lsd_error_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_stats_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <incoming_request_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_log_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_pkt_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_get_peers_reply_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_direct_response_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <picker_log_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <session_error_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_live_nodes_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <session_stats_header_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <dht_sample_infohashes_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <block_uploaded_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <alerts_dropped_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <socks5_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <file_prio_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <oversized_file_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_conflict_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <peer_info_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <file_progress_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <piece_info_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <piece_availability_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <tracker_list_alert as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<()> for T
                where
                    T: ?::cxx::core::marker::Sized,
                {}
                #[allow(dead_code)]
                struct __Invalid;
                #[automatically_derived]
                impl<T> __AmbiguousIfImpl<__Invalid> for T
                where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
                {}
                <torrent_info as __AmbiguousIfImpl<_>>::infer
            };
            #[automatically_derived]
            #[doc(hidden)]
            unsafe impl ::cxx::private::ImplVec for CastAlertRaw {}
            #[doc(hidden)]
            #[unsafe(export_name = "cxxbridge1$rust_vec$ltrs$CastAlertRaw$new")]
            unsafe extern "C" fn CastAlertRaw__vec_new(
                this: *mut ::cxx::private::RustVec<CastAlertRaw>,
            ) {
                unsafe {
                    ::cxx::core::ptr::write(this, ::cxx::private::RustVec::new());
                }
            }
            #[doc(hidden)]
            #[unsafe(export_name = "cxxbridge1$rust_vec$ltrs$CastAlertRaw$drop")]
            unsafe extern "C" fn CastAlertRaw__vec_drop(
                this: *mut ::cxx::private::RustVec<CastAlertRaw>,
            ) {
                let __fn = "<lt_rs::ffi::ffi::CastAlertRaw as Drop>::drop";
                ::cxx::private::prevent_unwind(
                    __fn,
                    || unsafe { ::cxx::core::ptr::drop_in_place(this) },
                );
            }
            #[doc(hidden)]
            #[unsafe(export_name = "cxxbridge1$rust_vec$ltrs$CastAlertRaw$len")]
            unsafe extern "C" fn CastAlertRaw__vec_len(
                this: *const ::cxx::private::RustVec<CastAlertRaw>,
            ) -> ::cxx::core::primitive::usize {
                unsafe { (*this).len() }
            }
            #[doc(hidden)]
            #[unsafe(export_name = "cxxbridge1$rust_vec$ltrs$CastAlertRaw$capacity")]
            unsafe extern "C" fn CastAlertRaw__vec_capacity(
                this: *const ::cxx::private::RustVec<CastAlertRaw>,
            ) -> ::cxx::core::primitive::usize {
                unsafe { (*this).capacity() }
            }
            #[doc(hidden)]
            #[unsafe(export_name = "cxxbridge1$rust_vec$ltrs$CastAlertRaw$data")]
            unsafe extern "C" fn CastAlertRaw__vec_data(
                this: *const ::cxx::private::RustVec<CastAlertRaw>,
            ) -> *const CastAlertRaw {
                unsafe { (*this).as_ptr() }
            }
            #[doc(hidden)]
            #[unsafe(
                export_name = "cxxbridge1$rust_vec$ltrs$CastAlertRaw$reserve_total"
            )]
            unsafe extern "C" fn CastAlertRaw__vec_reserve_total(
                this: *mut ::cxx::private::RustVec<CastAlertRaw>,
                new_cap: ::cxx::core::primitive::usize,
            ) {
                unsafe {
                    (*this).reserve_total(new_cap);
                }
            }
            #[doc(hidden)]
            #[unsafe(export_name = "cxxbridge1$rust_vec$ltrs$CastAlertRaw$set_len")]
            unsafe extern "C" fn CastAlertRaw__vec_set_len(
                this: *mut ::cxx::private::RustVec<CastAlertRaw>,
                len: ::cxx::core::primitive::usize,
            ) {
                unsafe {
                    (*this).set_len(len);
                }
            }
            #[doc(hidden)]
            #[unsafe(export_name = "cxxbridge1$rust_vec$ltrs$CastAlertRaw$truncate")]
            unsafe extern "C" fn CastAlertRaw__vec_truncate(
                this: *mut ::cxx::private::RustVec<CastAlertRaw>,
                len: ::cxx::core::primitive::usize,
            ) {
                let __fn = "<lt_rs::ffi::ffi::CastAlertRaw as Drop>::drop";
                ::cxx::private::prevent_unwind(
                    __fn,
                    || unsafe { (*this).truncate(len) },
                );
            }
            mod forbid {
                pub trait Drop {}
                #[automatically_derived]
                #[allow(drop_bounds)]
                impl<T: ?::cxx::core::marker::Sized + ::cxx::core::ops::Drop> self::Drop
                for T {}
                #[automatically_derived]
                impl self::Drop for super::InfoHashCpp {}
                #[automatically_derived]
                impl self::Drop for super::CastAlertRaw {}
                #[automatically_derived]
                impl self::Drop for super::AddTorrentParamsValues {}
            }
        };
    }
}
