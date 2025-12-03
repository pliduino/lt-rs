#pragma once

#include <libtorrent/alert_types.hpp>
#include "rust/cxx.h"
#include "cpp/error.h"

namespace ltrs {
    rust::Str torrent_error_alert_get_filename(lt::torrent_error_alert* alert);
    Error torrent_error_alert_get_error(lt::torrent_error_alert* alert);
}
