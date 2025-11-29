use crate::{
    alerts::ReadPieceAlert,
    errors::{LibtorrentError, LtrsError},
    ffi::alerts::read_piece::ffi::{read_piece_alert_get_error, read_piece_alert_get_size},
    torrent_handle::TorrentHandle,
};

impl ReadPieceAlert {
    pub fn handle<'a>(&'a self) -> TorrentHandle<'a> {
        self.as_torrent_alert().handle()
    }

    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    pub fn message(&self) -> String {
        self.as_torrent_alert().message()
    }

    pub fn piece(&self) -> u32 {
        unimplemented!()
    }

    pub fn buffer(&self) -> &[u8] {
        unimplemented!()
    }

    pub fn size(&self) -> i32 {
        unsafe { read_piece_alert_get_size(self.0) }
    }

    pub fn error(&self) -> LtrsError {
        unsafe { read_piece_alert_get_error(self.0) }.into()
    }
}
