#[cxx::bridge(namespace = "libtorrent")]
pub(crate) mod ffi {
    struct InfoHashCpp {
        version: u8, // 1 for v1, 2 for v2
        inner: [u8; 32],
    }

    unsafe extern "C++" {
        include!("src/lt.h");

        fn lt_parse_magnet_uri(uri: &str) -> UniquePtr<add_torrent_params>;

        // ╔===========================================================================╗
        // ║                                  Session                                  ║
        // ╚===========================================================================╝

        type session;
        fn lt_create_session() -> UniquePtr<session>;
        fn lt_create_session_with_settings(settings: &settings_pack) -> UniquePtr<session>;
        fn lt_session_add_torrent(
            session: Pin<&mut session>,
            params: &add_torrent_params,
        ) -> UniquePtr<torrent_handle>;
        fn lt_session_async_add_torrent(session: Pin<&mut session>, params: &add_torrent_params);
        fn lt_session_pop_alerts(session: Pin<&mut session>) -> UniquePtr<AlertListCpp>;
        fn lt_session_post_torrent_updates(session: Pin<&mut session>, flags: u32);

        // ╔===========================================================================╗
        // ║                               Settings Pack                               ║
        // ╚===========================================================================╝

        type settings_pack;
        fn lt_create_settings_pack() -> UniquePtr<settings_pack>;
        fn lt_set_alert_mask(pack: Pin<&mut settings_pack>, mask: u32);

        // ╔===========================================================================╗
        // ║                            Add Torrent Params                             ║
        // ╚===========================================================================╝

        type add_torrent_params;
        fn lt_set_add_torrent_params_path(params: Pin<&mut add_torrent_params>, path: &str);
        fn lt_add_torrent_params_info_hash(params: &add_torrent_params) -> InfoHashCpp;
        fn lt_write_resume_data_buf(params: &add_torrent_params) -> Vec<u8>;
        fn lt_read_resume_data(buf: &[u8]) -> UniquePtr<add_torrent_params>;

        // ╔===========================================================================╗
        // ║                              Torrent Handle                               ║
        // ╚===========================================================================╝

        type torrent_handle;
        fn lt_torrent_handle_in_session(handle: &torrent_handle) -> bool;
        fn lt_torrent_handle_read_piece(handle: &torrent_handle, piece: i32);
        fn lt_torrent_handle_status(handle: &torrent_handle) -> UniquePtr<torrent_status>;
        fn lt_torrent_handle_save_resume_data(handle: &torrent_handle, flags: u8);

        fn lt_torrent_handle_info_hash(handle: &torrent_handle) -> InfoHashCpp;

        // ╔===========================================================================╗
        // ║                              Torrent Status                               ║
        // ╚===========================================================================╝

        type torrent_status;
        fn lt_torrent_status_handle(status: &torrent_status) -> UniquePtr<torrent_handle>;
        fn lt_torrent_status_state(status: &torrent_status) -> u8;
        fn lt_torrent_status_progress(status: &torrent_status) -> f64;

        // ╔===========================================================================╗
        // ║                                  Alerts                                   ║
        // ╚===========================================================================╝

        /// Unsafe type, don't use it directly
        type AlertListCpp;
        type alert;

        /// Returns nullptr if index is out of range
        fn get(self: &AlertListCpp, index: usize) -> *mut alert;
        fn len(self: &AlertListCpp) -> usize;

        // ==========================  Torrent Finished  ===========================
        type torrent_finished_alert;
        unsafe fn lt_alert_torrent_finished_cast(alert: *mut alert) -> *mut torrent_finished_alert;
        unsafe fn lt_alert_torrent_finished_handle(
            alert: *mut torrent_finished_alert,
        ) -> UniquePtr<torrent_handle>;

        // =============================  Add Torrent  =============================
        type add_torrent_alert;
        unsafe fn lt_alert_add_torrent_cast(alert: *mut alert) -> *mut add_torrent_alert;
        unsafe fn lt_alert_add_torrent_handle(
            alert: *mut add_torrent_alert,
        ) -> UniquePtr<torrent_handle>;
        // TODO: Convert errors properly
        unsafe fn lt_alert_add_torrent_error(alert: *mut add_torrent_alert) -> i32;
        unsafe fn lt_alert_add_torrent_params(
            alert: *mut add_torrent_alert,
        ) -> UniquePtr<add_torrent_params>;

        // ============================  State Changed  ============================
        type state_changed_alert;
        unsafe fn lt_alert_state_changed_cast(alert: *mut alert) -> *mut state_changed_alert;
        unsafe fn lt_alert_state_changed_handle(
            alert: *mut state_changed_alert,
        ) -> UniquePtr<torrent_handle>;
        unsafe fn lt_alert_state_changed_state(alert: *mut state_changed_alert) -> u8;
        unsafe fn lt_alert_state_changed_prev_state(alert: *mut state_changed_alert) -> u8;

        // ============================  State Update  =============================
        type state_update_alert;

        unsafe fn lt_alert_state_update_cast(alert: *mut alert) -> *mut state_update_alert;
        unsafe fn lt_alert_state_update_status(
            alert: *mut state_update_alert,
        ) -> UniquePtr<CxxVector<torrent_status>>;

        // ==========================  Save Resume Data  ===========================
        type save_resume_data_alert;

        unsafe fn lt_alert_save_resume_data_cast(alert: *mut alert) -> *mut save_resume_data_alert;
        unsafe fn lt_alert_save_resume_data_handle(
            alert: *mut save_resume_data_alert,
        ) -> UniquePtr<torrent_handle>;
        unsafe fn lt_alert_save_resume_data_params(
            alert: *mut save_resume_data_alert,
        ) -> UniquePtr<add_torrent_params>;

        // =======================  Save Resume Data Failed  =======================
        type save_resume_data_failed_alert;

        unsafe fn lt_alert_save_resume_data_failed_cast(
            alert: *mut alert,
        ) -> *mut save_resume_data_failed_alert;
        unsafe fn lt_alert_save_resume_data_failed_handle(
            alert: *mut save_resume_data_failed_alert,
        ) -> UniquePtr<torrent_handle>;
        unsafe fn lt_alert_save_resume_data_failed_error(
            alert: *mut save_resume_data_failed_alert,
        ) -> i32;
    }
}
