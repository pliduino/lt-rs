#pragma once
#include <libtorrent/alert_types.hpp>
#include "lt-rs/src/ffi/mod.rs.h"

namespace ltrs {
    PieceIndex piece_finished_alert_get_piece_index(lt::piece_finished_alert* alert);

} // namespace ltrs
