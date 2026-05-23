# libtorrent-rs Bindings Reference

libtorrent version: **2.0.11** (`vendor/libtorrent` submodule)

## Session

`new()`, `new_with_settings()`, `async_add_torrent()`, `add_torrent()`,
`remove_torrent()`, `pop_alerts()`, `post_torrent_updates()`,
`get_torrent_hashes()`, `find_torrent()` (v1 + v2 info-hash),
`set_ip_filter()`, `get_ip_filter()`,
`save_state()` / `load_state()`, `apply_settings()`

## AddTorrentParams

`parse_magnet_uri()`, `from_torrent_bytes()`, `make_magnet_uri()`,
`set_path()`, `set_storage_mode()`, `write_resume_data_buf()`,
`load_resume_data()`, `info_hash()`, `file_paths()`,
`set_total_uploaded()`, `set_total_downloaded()`

## TorrentHandle

`in_session()`, `make_magnet_uri()`, `info_hashes()`, `status()`,
`save_resume_data()`, `pause()`, `resume()`, `force_recheck()`,
`force_reannounce()`, `move_storage()`, `save_path()`, `name()`,
`set_upload_limit()`, `set_download_limit()`, `upload_limit()`, `download_limit()`,
`set_flags()`, `unset_flags()`, `flags()`,
`prioritize_files()`, `file_priorities()`,
`piece_priority()`, `piece_priorities()`, `prioritize_pieces()`,
`set_piece_deadline()`, `reset_piece_deadline()`, `clear_piece_deadlines()`,
`set_first_last_piece_priority()`,
`add_tracker()`, `trackers()` → `Vec<AnnounceEntry>`,
`replace_trackers()`, `scrape_tracker()`,
`add_url_seed()`, `remove_url_seed()`, `url_seeds()`,
`rename_file()`,
`queue_position()`, `queue_position_up()`, `queue_position_down()`,
`queue_position_top()`, `queue_position_bottom()`,
`post_peer_info()`

## SettingsPack

`upload_rate_limit()`, `download_rate_limit()`, `connections_limit()`,
`active_downloads()`, `active_seeds()`, `active_limit()`,
`seed_time_ratio_limit()`, `seed_time_limit()`, `share_ratio_limit()`,
`max_failcount()`,
`set_dht_enabled()`, `set_lsd_enabled()`, `set_upnp_enabled()`, `set_natpmp_enabled()`,
`set_listen_interfaces()`, `set_outgoing_interfaces()`, `set_proxy()`

## TorrentStatus (full snapshot)

`download_rate`, `upload_rate`, `all_time_download`, `all_time_upload`,
`total_done`, `total_wanted`, `total_size`, `num_peers`, `num_seeds`,
`num_complete`, `num_incomplete`, `progress`, `state`,
`is_seeding`, `is_finished`, `is_paused`, `queue_position`,
`pieces` bitfield (`has_piece(n)`, `num_pieces_done()`),
`save_path`, `name`

## TorrentInfo (`TorrentInfoView`)

`name`, `total_size`, `num_files`, `num_pieces`, `piece_length`,
`files` → `Vec<FileEntry { path, size }>`

## PeerInfo

Decoded from `peer_info_alert` after `post_peer_info()`:
`ip`, `port`, `client`, `down_speed`, `up_speed`,
`total_download`, `total_upload`, `progress`, `flags`, `source`, `country`,
`is_seed()`, `is_local()`, `is_interesting()`, `is_choked()`

## IpFilter

`new()`, `add_rule(start, end, flags)`, `access(ip)`,
applied via `session.set_ip_filter()` / `session.get_ip_filter()`

## CreateTorrent + FileStorage

`FileStorage::new()`, `add_files()`, `num_files()`, `total_size()`  
`CreateTorrent::new(fs, piece_size)`, `add_tracker()`, `set_comment()`,
`set_creator()`, `set_priv()`, `set_piece_hashes()` → `Result<(), String>`,
`generate()` → `Vec<u8>`

## Alerts

`add_torrent_alert`, `torrent_finished_alert`, `torrent_checked_alert`,
`save_resume_data_alert`, `save_resume_data_failed_alert`,
`state_update_alert`, `metadata_received_alert`,
`tracker_error_alert`, `tracker_warning_alert`,
`storage_moved_alert`, `storage_moved_failed_alert`,
`torrent_paused_alert`, `torrent_resumed_alert`,
`fastresume_rejected_alert`, `state_changed_alert`,
`torrent_removed_alert`, `torrent_deleted_alert`, `torrent_error_alert`,
`file_completed_alert`, `file_renamed_alert`, `file_rename_failed_alert`,
`file_error_alert`, `hash_failed_alert`, `piece_finished_alert`,
`peer_connect_alert`, `peer_disconnected_alert`, `peer_ban_alert`,
`peer_error_alert`, `peer_info_alert`, `performance_alert`
