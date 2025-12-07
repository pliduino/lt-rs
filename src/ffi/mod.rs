pub(crate) mod alerts {
    pub(crate) mod add_torrent;
    pub(crate) mod block_downloading;
    pub(crate) mod block_timeout;
    pub(crate) mod dht_reply;
    pub(crate) mod file_completed;
    pub(crate) mod file_error;
    pub(crate) mod file_rename_failed;
    pub(crate) mod file_renamed;
    pub(crate) mod hash_failed;
    pub(crate) mod invalid_request;
    pub(crate) mod metadata_failed;
    pub(crate) mod peer_alert;
    pub(crate) mod peer_connect;
    pub(crate) mod peer_disconnected;
    pub(crate) mod peer_error;
    pub(crate) mod performance;
    pub(crate) mod piece_finished;
    pub(crate) mod read_piece;
    pub(crate) mod request_dropped;
    pub(crate) mod save_resume_data;
    pub(crate) mod save_resume_data_failed;
    pub(crate) mod scrape_failed;
    pub(crate) mod scrape_reply;
    pub(crate) mod state_changed;
    pub(crate) mod state_update;
    pub(crate) mod storage_moved;
    pub(crate) mod storage_moved_failed;
    pub(crate) mod torrent_alert;
    pub(crate) mod torrent_delete_failed;
    pub(crate) mod torrent_deleted;
    pub(crate) mod torrent_error;
    pub(crate) mod torrent_removed;
    pub(crate) mod tracker_alert;
    pub(crate) mod tracker_announce;
    pub(crate) mod tracker_error;
    pub(crate) mod tracker_reply;
    pub(crate) mod tracker_warning;
    pub(crate) mod unwanted_block;
    pub(crate) mod url_seed;
}

pub mod error;
pub mod torrent_handle;

#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    pub struct InfoHashCpp {
        version: u8, // 1 for v1, 2 for v2
        inner: [u8; 32],
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PieceIndex {
        inner: i32,
    }

    /// Represents a byte range within a piece. Internally this is is used for
    /// incoming piece requests.
    struct PeerRequest {
        /// The index of the piece in which the range starts.
        piece: PieceIndex,
        /// The byte offset within that piece where the range starts.
        start: i32,
        /// The size of the range, in bytes.
        length: i32,
    }

    enum AlertType {
        // TorrentAdded = 3,
        TorrentRemoved = 4,
        ReadPiece = 5,
        FileCompleted = 6,
        FileRenamed = 7,
        FileRenameFailed = 8,
        Performance = 9,
        StateChanged = 10,
        TrackerError = 11,
        TrackerWarning = 12,
        ScrapeReply = 13,
        ScrapeFailed = 14,
        TrackerReply = 15,
        DhtReply = 16,
        TrackerAnnounce = 17,
        HashFailed = 18,
        PeerBan = 19,
        PeerUnsnubbed = 20,
        PeerSnubbed = 21,
        PeerError = 22,
        PeerConnect = 23,
        PeerDisconnected = 24,
        InvalidRequest = 25,
        TorrentFinished = 26,
        PieceFinished = 27,
        RequestDropped = 28,
        BlockTimeout = 29,
        BlockFinished = 30,
        BlockDownloading = 31,
        UnwantedBlock = 32,
        StorageMoved = 33,
        StorageMovedFailed = 34,
        TorrentDeleted = 35,
        TorrentDeleteFailed = 36,
        SaveResumeData = 37,
        SaveResumeDataFailed = 38,
        TorrentPaused = 39,
        TorrentResumed = 40,
        TorrentChecked = 41,
        UrlSeed = 42,
        FileError = 43,
        MetadataFailed = 44,
        MetadataReceived = 45,
        UdpError = 46,
        ExternalIp = 47,
        ListenFailed = 48,
        ListenSucceeded = 49,
        PortmapError = 50,
        Portmap = 51,
        PortmapLog = 52,
        FastresumeRejected = 53,
        PeerBlocked = 54,
        DhtAnnounce = 55,
        DhtGetPeers = 56,
        // Stats = 57,
        CacheFlushed = 58,
        // AnonymousMode = 59,
        LsdPeer = 60,
        Trackerid = 61,
        DhtBootstrap = 62,
        TorrentError = 64,
        TorrentNeedCert = 65,
        IncomingConnection = 66,
        AddTorrent = 67,
        StateUpdate = 68,
        // MmapCache = 69,
        SessionStats = 70,
        DhtError = 73,
        DhtImmutableItem = 74,
        DhtMutableItem = 75,
        DhtPut = 76,
        I2p = 77,
        DhtOutgoingGetPeers = 78,
        Log = 79,
        TorrentLog = 80,
        PeerLog = 81,
        LsdError = 82,
        DhtStats = 83,
        IncomingRequest = 84,
        DhtLog = 85,
        DhtPkt = 86,
        DhtGetPeersReply = 87,
        DhtDirectResponse = 88,
        PickerLog = 89,
        SessionError = 90,
        DhtLiveNodes = 91,
        SessionStatsHeader = 92,
        DhtSampleInfohashes = 93,
        BlockUploaded = 94,
        AlertsDropped = 95,
        Socks5 = 96,
        FilePrio = 97,
        OversizedFile = 98,
        TorrentConflict = 99,
        PeerInfo = 100,
        FileProgress = 101,
        PieceInfo = 102,
        PieceAvailability = 103,
        TrackerList = 104,

        Unknown,
    }

    struct CastAlertRaw {
        type_: AlertType,
        alert: *mut alert,
    }

    #[namespace = "libtorrent"]
    extern "C++" {
        type torrent_handle = crate::ffi::torrent_handle::ffi::torrent_handle;
        type session;
        type add_torrent_params;
        type settings_pack;
        type torrent_status;
        type alert;
        type torrent_alert;
        type tracker_alert;
        type peer_alert;
        type torrent_removed_alert;
        type read_piece_alert;
        type file_completed_alert;
        type file_renamed_alert;
        type file_rename_failed_alert;
        type performance_alert;
        type state_changed_alert;
        type tracker_error_alert;
        type tracker_warning_alert;
        type scrape_reply_alert;
        type scrape_failed_alert;
        type tracker_reply_alert;
        type dht_reply_alert;
        type tracker_announce_alert;
        type hash_failed_alert;
        type peer_ban_alert;
        type peer_unsnubbed_alert;
        type peer_snubbed_alert;
        type peer_error_alert;
        type peer_connect_alert;
        type peer_disconnected_alert;
        type invalid_request_alert;
        type torrent_finished_alert;
        type piece_finished_alert;
        type request_dropped_alert;
        type block_timeout_alert;
        type block_finished_alert;
        type block_downloading_alert;
        type unwanted_block_alert;
        type storage_moved_alert;
        type storage_moved_failed_alert;
        type torrent_deleted_alert;
        type torrent_delete_failed_alert;
        type save_resume_data_alert;
        type save_resume_data_failed_alert;
        type torrent_paused_alert;
        type torrent_resumed_alert;
        type torrent_checked_alert;
        type url_seed_alert;
        type file_error_alert;
        type metadata_failed_alert;
        type metadata_received_alert;
        type udp_error_alert;
        type external_ip_alert;
        type listen_failed_alert;
        type listen_succeeded_alert;
        type portmap_error_alert;
        type portmap_alert;
        type portmap_log_alert;
        type fastresume_rejected_alert;
        type peer_blocked_alert;
        type dht_announce_alert;
        type dht_get_peers_alert;
        // type stats_alert;
        type cache_flushed_alert;
        // type anonymous_mode_alert;
        type lsd_peer_alert;
        type trackerid_alert;
        type dht_bootstrap_alert;
        type torrent_error_alert;
        type torrent_need_cert_alert;
        type incoming_connection_alert;
        type add_torrent_alert;
        type state_update_alert;
        // type mmap_cache_alert;
        type session_stats_alert;
        type dht_error_alert;
        type dht_immutable_item_alert;
        type dht_mutable_item_alert;
        type dht_put_alert;
        type i2p_alert;
        type dht_outgoing_get_peers_alert;
        type log_alert;
        type torrent_log_alert;
        type peer_log_alert;
        type lsd_error_alert;
        type dht_stats_alert;
        type incoming_request_alert;
        type dht_log_alert;
        type dht_pkt_alert;
        type dht_get_peers_reply_alert;
        type dht_direct_response_alert;
        type picker_log_alert;
        type session_error_alert;
        type dht_live_nodes_alert;
        type session_stats_header_alert;
        type dht_sample_infohashes_alert;
        type block_uploaded_alert;
        type alerts_dropped_alert;
        type socks5_alert;
        type file_prio_alert;
        type oversized_file_alert;
        type torrent_conflict_alert;
        type peer_info_alert;
        type file_progress_alert;
        type piece_info_alert;
        type piece_availability_alert;
        type tracker_list_alert;

        type torrent_info;
    }

    impl UniquePtr<torrent_status> {}
    impl CxxVector<torrent_status> {}

    struct AddTorrentParamsValues {
        version: i32,
        ti: SharedPtr<torrent_info>,
        name: String,
        trackers: Vec<String>,
        tracker_tiers: Vec<i32>,
        // dht_nodes: Vec<i32>,
        save_path: String,
    }

    unsafe extern "C++" {
        include!("cpp/lt.h");

        fn lt_parse_magnet_uri(uri: &str) -> UniquePtr<add_torrent_params>;

        // ╔===========================================================================╗
        // ║                                  Session                                  ║
        // ╚===========================================================================╝

        fn lt_create_session() -> UniquePtr<session>;
        fn lt_create_session_with_settings(settings: &settings_pack) -> UniquePtr<session>;
        unsafe fn lt_session_add_torrent(
            session: Pin<&mut session>,
            params: *mut add_torrent_params,
        ) -> UniquePtr<torrent_handle>;
        unsafe fn lt_session_async_add_torrent(
            session: Pin<&mut session>,
            params: *mut add_torrent_params,
        );
        fn lt_session_pop_alerts(session: Pin<&mut session>) -> Vec<CastAlertRaw>;
        fn lt_session_post_torrent_updates(session: Pin<&mut session>, flags: u32);

        // ╔===========================================================================╗
        // ║                               Settings Pack                               ║
        // ╚===========================================================================╝

        fn lt_create_settings_pack() -> UniquePtr<settings_pack>;
        fn lt_set_alert_mask(pack: Pin<&mut settings_pack>, mask: u32);

        // ╔===========================================================================╗
        // ║                            Add Torrent Params                             ║
        // ╚===========================================================================╝

        unsafe fn lt_set_add_torrent_params_path(params: *mut add_torrent_params, path: &str);
        unsafe fn lt_add_torrent_params_info_hash(params: *mut add_torrent_params) -> InfoHashCpp;
        unsafe fn lt_write_resume_data_buf(params: *mut add_torrent_params) -> Vec<u8>;
        unsafe fn lt_read_resume_data(buf: &[u8]) -> UniquePtr<add_torrent_params>;

        // ╔===========================================================================╗
        // ║                              Torrent Handle                               ║
        // ╚===========================================================================╝

        // ╔===========================================================================╗
        // ║                              Torrent Status                               ║
        // ╚===========================================================================╝

        unsafe fn lt_torrent_status_handle(
            status: *mut torrent_status,
        ) -> UniquePtr<torrent_handle>;
        unsafe fn lt_torrent_status_state(status: *mut torrent_status) -> u8;
        unsafe fn lt_torrent_status_progress(status: *mut torrent_status) -> f64;

        // ╔===========================================================================╗
        // ║                                  Alerts                                   ║
        // ╚===========================================================================╝

    }
}
