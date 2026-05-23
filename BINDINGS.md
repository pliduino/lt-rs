# libtorrent-rs Bindings

libtorrent version: **2.0.11** (`vendor/libtorrent` submodule pinned to `9d7443f`)

21 integration tests pass against native libtorrent C++.

---

## Coverage

### Session (`lt::session`)
`new()`, `new_with_settings()`, `async_add_torrent()`, `add_torrent()`, `remove_torrent()`, `pop_alerts()`, `post_torrent_updates()`, `set_ip_filter()`, `get_ip_filter()`, `session_state()` / `load_state()`

### AddTorrentParams
`parse_magnet_uri()`, `make_magnet_uri()`, `set_path()`, `set_storage_mode()`, `write_resume_data_buf()`, `load_resume_data()`, `info_hash()`

### TorrentHandle
`in_session()`, `make_magnet_uri()`, `info_hashes()`, `status()`, `save_resume_data()`, `pause()`, `resume()`, `force_recheck()`, `move_storage()`, `save_path()`, `name()`, `set_upload_limit()`, `set_download_limit()`, `upload_limit()`, `download_limit()`, `set_flags()`, `unset_flags()`, `flags()`, `prioritize_files()`, `file_priorities()`, `add_tracker()`, `trackers()` → `Vec<AnnounceEntry>`

### TorrentStatus (full snapshot, one FFI call)
`download_rate`, `upload_rate`, `all_time_download`, `all_time_upload`, `total_done`, `total_wanted`, `total_size`, `num_peers`, `num_seeds`, `num_complete`, `num_incomplete`, `progress`, `state`, `is_seeding`, `is_finished`, `is_paused`, `pieces` (bitfield with `has_piece(n)` + `num_pieces_done()`), `save_path`, `name`

### TorrentInfo (`TorrentInfoView`)
`name`, `total_size`, `num_files`, `num_pieces`, `piece_length`, `files` → `Vec<FileEntry { path, size }>`

### IpFilter
`new()`, `add_rule(start, end, flags)`, `access(ip)`, `session.set_ip_filter()`, `session.get_ip_filter()`

### CreateTorrent + FileStorage
`FileStorage::new()`, `add_files()`, `num_files()`, `total_size()`
`CreateTorrent::new(fs, piece_size)`, `add_tracker()`, `set_comment()`, `set_creator()`, `set_priv()`, `set_piece_hashes()` (blocking, call from `spawn_blocking`), `generate()` -> `Vec<u8>` (raw bencode)

### Alerts
`add_torrent_alert`, `torrent_finished_alert`, `torrent_checked_alert`, `save_resume_data_alert`, `save_resume_data_failed_alert`, `state_update_alert`, `metadata_received_alert`, `tracker_error_alert`, `tracker_warning_alert`, `storage_moved_alert`, `storage_moved_failed_alert`, `torrent_paused_alert`, `torrent_resumed_alert`, `fastresume_rejected_alert`, `state_changed_alert`, `torrent_removed_alert`, `torrent_deleted_alert`, `torrent_error_alert`, `file_completed_alert`, `file_renamed_alert`, `file_rename_failed_alert`, `file_error_alert`, `hash_failed_alert`, `piece_finished_alert`, `peer_connect_alert`, `peer_disconnected_alert`, `peer_ban_alert`, `peer_error_alert`, `performance_alert`

---

## Integration Tests (`tests/integration.rs`)

| Test | What it covers |
|------|---------------|
| `session_creates_and_drops` | Session lifecycle |
| `session_get_torrent_hashes_empty` | Empty session |
| `parse_magnet_uri_roundtrip` | Magnet parse + re-encode |
| `resume_data_roundtrip` | Write + load resume data |
| `add_torrent_params_total_stats` | Stats from add params |
| `create_torrent_single_file` | File storage + torrent generation |
| `create_torrent_with_tracker` | Tracker URL in .torrent |
| `create_torrent_private` | Private flag in .torrent |
| `ip_filter_block_range` | Block IP range |
| `ip_filter_allow_after_block` | Allow within blocked range |
| `ip_filter_applied_to_session` | set/get roundtrip on session |
| `session_find_torrent_by_hash` | Handle lookup by info_hash |
| `session_save_load_state` | Session state persistence |
| `torrent_handle_status_snapshot` | Full status struct fields |
| `torrent_handle_force_recheck` | Force hash recheck |
| `async_add_torrent_fires_alert` | Alert delivery on add |
| `torrent_handle_trackers_announce_entry` | Tracker list + add_tracker |
| `torrent_handle_flags` | set/unset/get flags |
| `torrent_handle_rate_limits` | Per-torrent bandwidth limits |
| `torrent_handle_pause_resume` | Pause + resume alerts |
| `torrent_handle_move_storage_fires_alert` | Move storage + alert |

---

## Build

Requires `clang++`, `b2` (boost-build), `libssl-dev`. Compiles libtorrent from `vendor/libtorrent` on first build, cached in `OUT_DIR` after.

```bash
cargo build
cargo test
```
