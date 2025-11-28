#pragma once

#include <libtorrent/alert_types.hpp>
#include <lt-rs/src/ffi/mod.rs.h>

namespace ltrs {
    int read_piece_alert_get_size(lt::read_piece_alert* a);
    ErrorCodeRaw read_piece_alert_get_error(lt::read_piece_alert* a);
}
