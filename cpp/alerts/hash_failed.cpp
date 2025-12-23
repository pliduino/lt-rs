#include "./hash_failed.h"
#include "lt-rs/src/ffi/mod.rs.h"

namespace ltrs {

PieceIndex hash_failed_alert_get_piece_index(lt::hash_failed_alert* alert) {
    return PieceIndex { .inner = static_cast<int>(alert->piece_index) };
}

} // namespace ltrs
