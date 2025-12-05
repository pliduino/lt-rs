#pragma once
#include <cstdint>
#include <libtorrent/alert_types.hpp>
#include "lt-rs/src/ffi/mod.rs.h"

namespace ltrs {
    PieceIndex hash_failed_alert_get_piece_index(lt::hash_failed_alert* alert);

} // namespace ltrs
