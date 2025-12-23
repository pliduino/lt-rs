#pragma once
#include "lt-rs/src/ffi/mod.rs.h"
#include <libtorrent/alert_types.hpp>

namespace ltrs {
int block_downloading_alert_get_block_index(lt::block_downloading_alert* alert);
PieceIndex block_downloading_alert_get_piece_index(lt::block_downloading_alert* alert);

} // namespace ltrs
