#pragma once
#include "lt-rs/src/ffi/mod.rs.h"
#include <libtorrent/alert_types.hpp>

namespace ltrs {
    int request_dropped_alert_get_block_index(lt::request_dropped_alert* alert);
    PieceIndex request_dropped_alert_get_piece_index(lt::request_dropped_alert* alert);

} // namespace ltrs
