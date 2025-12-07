#pragma once
#include <libtorrent/alert_types.hpp>
#include "lt-rs/src/ffi/mod.rs.h"

namespace ltrs {
int unwanted_block_alert_get_block_index(lt::unwanted_block_alert* alert);
PieceIndex unwanted_block_alert_get_piece_index(lt::unwanted_block_alert* alert);

} // namespace ltrs
