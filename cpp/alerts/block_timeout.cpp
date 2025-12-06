#include "./block_timeout.h"

namespace ltrs {

int block_timeout_alert_get_block_index(lt::block_timeout_alert* alert) {
    return alert->block_index;
}

PieceIndex block_timeout_alert_get_piece_index(lt::block_timeout_alert* alert) {
    return PieceIndex { .inner = static_cast<int32_t>(alert->piece_index) };
}

} // namespace ltrs
