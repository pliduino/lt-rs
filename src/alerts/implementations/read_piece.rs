use crate::{
    alerts::ReadPieceAlert,
    errors::LtrsError,
    ffi::alerts::read_piece::ffi::{read_piece_alert_get_error, read_piece_alert_get_size},
    torrent_handle::TorrentHandle,
};

impl ReadPieceAlert {
    #[inline(always)]
    pub fn handle(&self) -> TorrentHandle {
        self.as_torrent_alert().handle()
    }

    #[inline(always)]
    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    #[inline(always)]
    pub fn message(&self) -> String {
        self.as_torrent_alert().message()
    }

    #[inline(always)]
    pub fn piece(&self) {
        unimplemented!()
    }

    #[inline(always)]
    pub fn buffer(&self) {
        unimplemented!()
    }

    #[inline(always)]
    pub fn size(&self) -> i32 {
        unsafe { read_piece_alert_get_size(self.0) }
    }

    #[inline(always)]
    pub fn error(&self) -> LtrsError {
        unsafe { read_piece_alert_get_error(self.0) }.into()
    }
}
