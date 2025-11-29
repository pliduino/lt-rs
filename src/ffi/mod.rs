pub mod alerts {
    pub mod read_piece;
    pub mod torrent_alert;
    pub mod torrent_removed;
}

#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    pub struct InfoHashCpp {
        version: u8, // 1 for v1, 2 for v2
        inner: [u8; 32],
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

    #[repr(i32)]
    enum ErrorCodeRaw {
        no_error = 0,
        file_collision,
        failed_hash_check,
        torrent_is_no_dict,
        torrent_missing_info,
        torrent_info_no_dict,
        torrent_missing_piece_length,
        torrent_missing_name,
        torrent_invalid_name,
        torrent_invalid_length,
        torrent_file_parse_failed,
        torrent_missing_pieces,
        torrent_invalid_hashes,
        too_many_pieces_in_torrent,
        invalid_swarm_metadata,
        invalid_bencoding,
        no_files_in_torrent,
        invalid_escaped_string,
        session_is_closing,
        duplicate_torrent,
        invalid_torrent_handle,
        invalid_entry_type,
        missing_info_hash_in_uri,
        file_too_short,
        unsupported_url_protocol,
        url_parse_error,
        peer_sent_empty_piece,
        parse_failed,
        invalid_file_tag,
        missing_info_hash,
        mismatching_info_hash,
        invalid_hostname,
        invalid_port,
        port_blocked,
        expected_close_bracket_in_address,
        destructing_torrent,
        timed_out,
        upload_upload_connection,
        uninteresting_upload_peer,
        invalid_info_hash,
        torrent_paused,
        invalid_have,
        invalid_bitfield_size,
        too_many_requests_when_choked,
        invalid_piece,
        no_memory,
        torrent_aborted,
        self_connection,
        invalid_piece_size,
        timed_out_no_interest,
        timed_out_inactivity,
        timed_out_no_handshake,
        timed_out_no_request,
        invalid_choke,
        invalid_unchoke,
        invalid_interested,
        invalid_not_interested,
        invalid_request,
        invalid_hash_list,
        invalid_hash_piece,
        invalid_cancel,
        invalid_dht_port,
        invalid_suggest,
        invalid_have_all,
        invalid_have_none,
        invalid_reject,
        invalid_allow_fast,
        invalid_extended,
        invalid_message,
        sync_hash_not_found,
        invalid_encryption_constant,
        no_plaintext_mode,
        no_rc4_mode,
        unsupported_encryption_mode,
        unsupported_encryption_mode_selected,
        invalid_pad_size,
        invalid_encrypt_handshake,
        no_incoming_encrypted,
        no_incoming_regular,
        duplicate_peer_id,
        torrent_removed,
        packet_too_large,

        reserved,

        http_error,
        missing_location,
        invalid_redirection,
        redirecting,
        invalid_range,
        no_content_length,
        banned_by_ip_filter,
        too_many_connections,
        peer_banned,
        stopping_torrent,
        too_many_corrupt_pieces,
        torrent_not_ready,
        peer_not_constructed,
        session_closing,
        optimistic_disconnect,
        torrent_finished,
        no_router,
        metadata_too_large,
        invalid_metadata_request,
        invalid_metadata_size,
        invalid_metadata_offset,
        invalid_metadata_message,
        pex_message_too_large,
        invalid_pex_message,
        invalid_lt_tracker_message,
        too_frequent_pex,
        no_metadata,
        invalid_dont_have,
        requires_ssl_connection,
        invalid_ssl_cert,
        not_an_ssl_torrent,
        banned_by_port_filter,
        invalid_session_handle,
        invalid_listen_socket,
        invalid_hash_request,
        invalid_hashes,
        invalid_hash_reject,

        unsupported_protocol_version = 120,
        natpmp_not_authorized,
        network_failure,
        no_resources,
        unsupported_opcode,

        missing_file_sizes = 130,
        no_files_in_resume_data,
        missing_pieces,
        mismatching_number_of_files,
        mismatching_file_size,
        mismatching_file_timestamp,
        not_a_dictionary,
        invalid_blocks_per_piece,
        missing_slots,
        too_many_slots,
        invalid_slot_list,
        invalid_piece_index,
        pieces_need_reorder,
        resume_data_not_modified,
        invalid_save_path,

        http_parse_error = 150,
        http_missing_location,
        http_failed_decompress,

        no_i2p_router = 160,
        no_i2p_endpoint = 161,

        scrape_not_available = 170,
        invalid_tracker_response,
        invalid_peer_dict,
        tracker_failure,
        invalid_files_entry,
        invalid_hash_entry,
        invalid_peers_entry,
        invalid_tracker_response_length,

        invalid_tracker_transaction_id,
        invalid_tracker_action,
        announce_skipped,

        expected_string = 190,
        expected_colon,
        unexpected_eof,
        expected_value,
        depth_exceeded,
        limit_exceeded,
        overflow,

        no_entropy = 200,
        ssrf_mitigation,
        blocked_by_idna,

        torrent_unknown_version = 210,
        torrent_missing_file_tree,
        torrent_missing_meta_version,
        torrent_inconsistent_files,
        torrent_missing_piece_layer,
        torrent_invalid_piece_layer,
        torrent_missing_pieces_root,
        torrent_inconsistent_hashes,
        torrent_invalid_pad_file,

        error_code_max,
    }

    enum ErrorCategory {
        LibtorrentError,
        HttpError,
        GzipError,
        I2pError,
        PcpError,
        BdecodeError,
        SocksError,
        UpnpError,
        Unknown,
    }

    struct Error {
        category: ErrorCategory,
        code: i32,
    }

    #[namespace = "libtorrent"]
    extern "C++" {
        type session;
        type add_torrent_params;
        type settings_pack;
        type torrent_handle;
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
    }

    unsafe extern "C++" {
        include!("cpp/lt.h");

        fn lt_parse_magnet_uri(uri: &str) -> UniquePtr<add_torrent_params>;

        // ╔===========================================================================╗
        // ║                                  Session                                  ║
        // ╚===========================================================================╝

        fn lt_create_session() -> UniquePtr<session>;
        fn lt_create_session_with_settings(settings: &settings_pack) -> UniquePtr<session>;
        fn lt_session_add_torrent(
            session: Pin<&mut session>,
            params: &add_torrent_params,
        ) -> UniquePtr<torrent_handle>;
        fn lt_session_async_add_torrent(session: Pin<&mut session>, params: &add_torrent_params);
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

        fn lt_set_add_torrent_params_path(params: Pin<&mut add_torrent_params>, path: &str);
        fn lt_add_torrent_params_info_hash(params: &add_torrent_params) -> InfoHashCpp;
        fn lt_write_resume_data_buf(params: &add_torrent_params) -> Vec<u8>;
        fn lt_read_resume_data(buf: &[u8]) -> UniquePtr<add_torrent_params>;

        // ╔===========================================================================╗
        // ║                              Torrent Handle                               ║
        // ╚===========================================================================╝

        fn lt_torrent_handle_in_session(handle: &torrent_handle) -> bool;
        fn lt_torrent_handle_read_piece(handle: &torrent_handle, piece: i32);
        fn lt_torrent_handle_status(handle: &torrent_handle) -> UniquePtr<torrent_status>;
        fn lt_torrent_handle_save_resume_data(handle: &torrent_handle, flags: u8);

        fn lt_torrent_handle_info_hash(handle: &torrent_handle) -> InfoHashCpp;

        // ╔===========================================================================╗
        // ║                              Torrent Status                               ║
        // ╚===========================================================================╝

        fn lt_torrent_status_handle(status: &torrent_status) -> UniquePtr<torrent_handle>;
        fn lt_torrent_status_state(status: &torrent_status) -> u8;
        fn lt_torrent_status_progress(status: &torrent_status) -> f64;

        // ╔===========================================================================╗
        // ║                                  Alerts                                   ║
        // ╚===========================================================================╝

    }
}
