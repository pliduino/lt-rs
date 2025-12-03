#include "./torrent_error.h"
#include "cpp/error.h"

namespace ltrs {
    rust::Str torrent_error_alert_get_filename (lt::torrent_error_alert* alert) {
        return alert->filename();
    }

    Error torrent_error_alert_get_error (lt::torrent_error_alert* alert) {
        return error_code_to_error(alert->error);
    }
}
